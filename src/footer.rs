use crate::Filter;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yew_functional::*;

#[derive(Properties, Clone, PartialEq)]
pub struct FooterProps {
	/// Called when the filter selection changes
	pub on_filterchange: Callback<Filter>,
	pub selected_filter: Filter,
}

pub struct FooterFunction {}
impl FunctionProvider for FooterFunction {
	type TProps = FooterProps;

	fn run(props: &Self::TProps) -> Html {
		let props = props.clone();
		html! {
			<footer class="footer">
				<span class="todo-count">
					<strong>{"X"}</strong>
					<span>{" items left"}</span>
				</span>
				<ul class="filters">
					{
						for Filter::iter().map(|filter| {
							html! {
								<li>
									<a
										href="./#"
										class=if filter == props.selected_filter {"selected"} else {""}
										onclick=Callback::from({
											let props = props.clone();
											move |_ev| {
												props.on_filterchange.emit(filter);
											}}
										)
									>{format!("{:?}", filter)}</a>
								</li>
							}
						})
					}
				</ul>
			</footer>
		}
	}
}
pub type Footer = FunctionComponent<FooterFunction>;

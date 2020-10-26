use yew::prelude::*;
use yew_functional::*;

pub struct CopyrightFunction {}
impl FunctionProvider for CopyrightFunction {
	type TProps = ();

	fn run(_props: &Self::TProps) -> Html {
		html! {
			<footer class="info">
				<p>{"Double-click to edit a todo"}</p>
				<p>
					{"Created by "}
					<a href="https://github.com/lukechu10" target="_blank">{"lukechu10"}</a>
				</p>
				<p>
					{"Part of "}
					<a href="http://todomvc.com">{"TodoMVC"}</a>
				</p>
			</footer>
		}
	}
}
pub type Copyright = FunctionComponent<CopyrightFunction>;

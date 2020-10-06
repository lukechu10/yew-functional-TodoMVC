use crate::{TodoEntry, TodoStatus};
use std::rc::Rc;
use yew::prelude::*;
use yew_functional::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ItemProps {
	pub todo: TodoEntry,
	/// Called whenever todo.status should be toggled.
	pub toggle_completed: Callback<()>,
}

pub struct ItemFunction {}
impl FunctionProvider for ItemFunction {
	type TProps = ItemProps;

	fn run(props: &Self::TProps) -> Html {
		let props = props.clone();

		let (editing, set_editing) = use_state(|| false);
		let set_editing = Rc::new(set_editing);

		let handle_edit = Callback::from({
			let set_editing = set_editing.clone();
			move |_ev| set_editing(true)
		});

		let handle_blur = Callback::from({
			let set_editing = set_editing.clone();
			move |_ev| set_editing(false)
		});

		let name = props.todo.name.clone();
		let completed = props.todo.status == TodoStatus::Completed;

		html! {
			<li class=format!("{} {}", if *editing {"editing"} else {""}, if completed {"completed"} else {""})>
				<div class="view">
					<input class="toggle" type="checkbox"
						checked=completed
						oninput=Callback::from(move |_ev| {
							log::info!("Checked!");
							props.toggle_completed.emit(());
						}) />
					<label ondblclick=handle_edit>
						{&name}
					</label>
					<button class="destroy" />
				</div>
				{
					if *editing {
						html! {
							<input class="edit" value={&name} onblur=handle_blur />
						}
					}
					else {
						html! {}
					}
				}
			</li>
		}
	}
}
pub type Item = FunctionComponent<ItemFunction>;

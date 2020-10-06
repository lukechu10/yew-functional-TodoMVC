use crate::{TodoEntry, TodoStatus};
use std::rc::Rc;
use yew::prelude::*;
use yew_functional::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ItemProps {
	pub todo: TodoEntry,
	/// Called whenever todo.status should be toggled.
	pub toggle_completed: Callback<()>,
	/// Called whenever the todo should be removed from the todo list.
	pub clear_todo: Callback<()>,
	pub rename_todo: Callback<String>,
}

pub struct ItemFunction {}
impl FunctionProvider for ItemFunction {
	type TProps = ItemProps;

	fn run(props: &Self::TProps) -> Html {
		let props = props.clone();

		let (editing, set_editing) = use_state(|| false);
		let set_editing = Rc::new(set_editing);

		let input_ref = use_ref(|| NodeRef::default());

		let handle_edit = Callback::from({
			let set_editing = set_editing.clone();
			let input_ref = input_ref.clone();
			move |_ev| {
				set_editing(true);
				input_ref
					.borrow()
					.cast::<web_sys::HtmlInputElement>()
					.unwrap()
					.focus()
					.unwrap(); // focus input
			}
		});

		let handle_blur = Callback::from({
			let set_editing = set_editing.clone();
			let rename_todo = props.rename_todo.clone();
			let clear_todo = props.clear_todo.clone();
			let input_ref = input_ref.clone();
			move |_ev| {
				let mut new_name = input_ref
					.borrow()
					.cast::<web_sys::HtmlInputElement>()
					.unwrap()
					.value();
				new_name = new_name.trim().to_string(); // trim input
				if new_name != "" {
					rename_todo.emit(new_name);
				} else {
					// destroy todo
					clear_todo.emit(());
				}
				set_editing(false);
			}
		});

		let name = props.todo.name.clone();
		let completed = props.todo.status == TodoStatus::Completed;

		let toggle_completed = props.toggle_completed;
		let clear_todo = props.clear_todo;
		html! {
			<li class=format!("{} {}", if *editing {"editing"} else {""}, if completed {"completed"} else {""})>
				<div class="view">
					<input class="toggle" type="checkbox"
						checked=completed
						oninput=Callback::from(move |_ev| toggle_completed.emit(())
						) />
					<label ondblclick=handle_edit>
						{&name}
					</label>
					<button class="destroy" onclick=Callback::from(move |_ev| clear_todo.emit(())) />
				</div>
				{
					if *editing {
						html! {
							<input class="edit" value={&name} onblur=handle_blur ref=input_ref.clone().borrow().clone() />
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

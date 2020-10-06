use crate::item::Item;
use crate::TodoEntry;
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;
use yew_functional::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ListProps {
	pub todo_list: Rc<Vec<TodoEntry>>,
	/// Called whenever a todo's status is toggled. The uuid of the todo is sent to the callback.
	pub toggle_completed: Callback<Uuid>,
}

pub struct ListFunction {}
impl FunctionProvider for ListFunction {
	type TProps = ListProps;

	fn run(props: &Self::TProps) -> Html {
		let props = props.clone();

		let todos = props.todo_list.clone();
		let toggle_completed = props.toggle_completed;
		html! {
			<section class="main">
				<input id="toggle-all" class="toggle-all" type="checkbox" readonly=true />
				<label for="toggle-all" />

				<ul class="todo-list">
				{
					for todos.iter().map(|todo| {
						let toggle_completed_callback = Callback::from({
							let toggle_completed = toggle_completed.clone();
							let todo_id = todo.id.clone();
							move |_|toggle_completed.emit(todo_id)
						});

						html! {
							<Item
								todo=todo
								toggle_completed=toggle_completed_callback
							/>
						}
					})
				}
				</ul>
			</section>
		}
	}
}
pub type List = FunctionComponent<ListFunction>;

use crate::item::Item;
use crate::TodoEntry;
use enclose::enc;
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;
use yew_functional::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ListProps {
    pub todo_list: Rc<Vec<TodoEntry>>,
    /// Called whenever a todo's status is toggled. The uuid of the todo is sent to the callback.
    pub toggle_completed: Callback<Uuid>,
    pub clear_todo: Callback<Uuid>,
    pub all_completed: bool,
    pub toggle_complete_all: Callback<()>,
    pub rename_todo: Callback<(Uuid, String)>,
}

#[function_component(List)]
pub fn list(props: &ListProps) -> Html {
    let props = props.clone();

    let todos = props.todo_list.clone();
    let toggle_completed = props.toggle_completed;
    let clear_todo = props.clear_todo;
    let toggle_complete_all = props.toggle_complete_all;
    let rename_todo = props.rename_todo;
    html! {
        <section class="main">
            <input
                id="toggle-all" class="toggle-all" type="checkbox"
                oninput=Callback::from(move |_ev| toggle_complete_all.emit(()))
                checked=props.all_completed
                readonly=true
            />
            <label for="toggle-all" />

            <ul class="todo-list">
            {
                for todos.iter().map(|todo| {
                    let id = todo.id;

                    let toggle_completed_callback = Callback::from(enc!((toggle_completed)
                        move |_| toggle_completed.emit(id)
                    ));

                    let clear_todo_callback = Callback::from(enc!((clear_todo)
                        move |_| clear_todo.emit(id)
                    ));

                    let rename_todo_callback = Callback::from(enc!((rename_todo)
                        move |new_name| rename_todo.emit((id, new_name))
                    ));

                    html! {
                        <Item
                            key=id.to_string()
                            todo=todo
                            toggle_completed=toggle_completed_callback
                            clear_todo=clear_todo_callback
                            rename_todo=rename_todo_callback
                        />
                    }
                })
            }
            </ul>
        </section>
    }
}

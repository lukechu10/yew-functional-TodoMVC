use crate::item::Item;
use crate::{cb, TodoEntry};
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
    let todos = props.todo_list.clone();
    let toggle_completed = props.toggle_completed.clone();
    let clear_todo = props.clear_todo.clone();
    let toggle_complete_all = props.toggle_complete_all.clone();
    let rename_todo = props.rename_todo.clone();

    html! {
        <section class="main">
            <input
                id="toggle-all" class="toggle-all" type="checkbox"
                checked=props.all_completed
                readonly=true
                oninput=cb!((toggle_complete_all) move |_ev| toggle_complete_all.emit(()))
            />
            <label for="toggle-all" />

            <ul class="todo-list">
            {
                for todos.iter().map(|todo| {
                    let id = todo.id;

                    let toggle_completed = cb!((toggle_completed) move |_| toggle_completed.emit(id));

                    let clear_todo = cb!((clear_todo) move |_| clear_todo.emit(id));

                    let rename_todo = cb!((rename_todo) move |new_name| rename_todo.emit((id, new_name)));

                    html! {
                        <Item
                            key=id.to_string()
                            todo=todo
                            toggle_completed=toggle_completed
                            clear_todo=clear_todo
                            rename_todo=rename_todo
                        />
                    }
                })
            }
            </ul>
        </section>
    }
}

use crate::{cb, TodoEntry, TodoStatus};
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

#[function_component(Item)]
pub fn item(props: &ItemProps) -> Html {
    let props = props.clone();

    let (editing, set_editing) = use_state(|| false);

    let input_ref = use_ref(|| NodeRef::default());

    let handle_edit = cb!((set_editing, input_ref) move |_ev| {
        set_editing(true);
        input_ref
            .borrow()
            .cast::<web_sys::HtmlInputElement>()
            .unwrap()
            .focus()
            .unwrap(); // focus input
    });

    let handle_blur = cb!((set_editing, input_ref, props) move |_| {
        let mut new_name = input_ref
            .borrow()
            .cast::<web_sys::HtmlInputElement>()
            .unwrap()
            .value();
        new_name = new_name.trim().to_string(); // trim input
        if new_name != "" {
            props.rename_todo.emit(new_name);
        } else {
            // destroy todo
            props.clear_todo.emit(());
        }
        set_editing(false);
    });

    let handle_submit = cb!((handle_blur, input_ref, props) move |ev: KeyboardEvent| {
        match ev.key().as_str() {
            "Enter" => handle_blur.emit(()),
            "Escape" => {
                input_ref
                    .borrow()
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .set_value(&props.todo.name);
                set_editing(false); // do not rename todo
            }
            _ => {}
        }
    });

    let completed = props.todo.status == TodoStatus::Completed;

    let toggle_completed = props.toggle_completed;
    let clear_todo = props.clear_todo;
    html! {
        <li class=format!("{} {}", if *editing {"editing"} else {""}, if completed {"completed"} else {""})>
            <div class="view">
                <input class="toggle" type="checkbox"
                    checked=completed
                    oninput=cb!((toggle_completed) move |_ev| toggle_completed.emit(()))
                />
                <label ondblclick=handle_edit>
                    {&props.todo.name}
                </label>
                <button class="destroy" onclick=cb!((clear_todo) move |_ev| clear_todo.emit(())) />
            </div>
            {
                if *editing {
                    html! {
                        <input class="edit" value={&props.todo.name}
                            onblur=cb!((handle_blur) move |_ev| handle_blur.emit(()))
                            onkeyup=handle_submit
                            ref=input_ref.borrow().clone()
                        />
                    }
                }
                else {
                    html! {}
                }
            }
        </li>
    }
}

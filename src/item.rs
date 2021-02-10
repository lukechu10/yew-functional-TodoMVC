use crate::{TodoEntry, TodoStatus};
use enclose::enc;
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

        let handle_edit = enc!((set_editing, input_ref) move |_ev| {
            set_editing(true);
            input_ref
                .borrow()
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .focus()
                .unwrap(); // focus input
        });

        let handle_blur = enc!((set_editing, input_ref, props) move || {
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

        let handle_submit = enc!((handle_blur, input_ref, props) move |ev: KeyboardEvent| {
            match ev.key().as_str() {
                "Enter" => handle_blur(),
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
                        oninput=Callback::from(move |_ev| toggle_completed.emit(()))
                    />
                    <label ondblclick=Callback::from(handle_edit)>
                        {&props.todo.name}
                    </label>
                    <button class="destroy" onclick=Callback::from(move |_ev| clear_todo.emit(())) />
                </div>
                {
                    if *editing {
                        html! {
                            <input class="edit" value={&props.todo.name}
                                onblur=Callback::from(move |_ev| handle_blur())
                                onkeyup=Callback::from(handle_submit)
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
}
pub type Item = FunctionComponent<ItemFunction>;

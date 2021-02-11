use crate::{cb, Filter};
use strum::IntoEnumIterator;
use yew::prelude::*;
use yew_functional::*;

#[derive(Properties, Clone, PartialEq)]
pub struct FooterProps {
    /// Called when the filter selection changes
    pub on_filterchange: Callback<Filter>,
    pub selected_filter: Filter,
    pub todos_left: u32,
    pub todos_completed: u32,
    pub clear_completed: Callback<()>,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let items_text = match props.todos_left {
        1 => "item",
        _ => "items",
    };

    html! {
        <footer class="footer">
            <span class="todo-count">
                <strong>{props.todos_left}</strong>
                <span>{format!(" {} left", items_text)}</span>
            </span>
            <ul class="filters">
                {
                    for Filter::iter().map(|filter| {
                        html! {
                            <li>
                                <a
                                    href=filter
                                    class=if filter == props.selected_filter {"selected"} else {""}
                                    onclick=cb!((props) move |_ev| {
                                            props.on_filterchange.emit(filter);
                                        })
                                >{format!("{:?}", filter)}</a>
                            </li>
                        }
                    })
                }
            </ul>
            {
                if props.todos_completed > 0 {
                    html! {
                        <button
                            class="clear-completed"
                            onclick=cb!((props) move |_ev| props.clear_completed.emit(())) >
                            {"Clear completed"}
                        </button>
                    }
                }
                else {
                    html! {}
                }
            }
        </footer>
    }
}

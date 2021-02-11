use yew::prelude::*;
use yew_functional::*;

#[function_component(Copyright)]
pub fn copyright(_props: &()) -> Html {
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

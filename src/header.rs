use enclose::enclose;
use std::rc::Rc;
use yew::prelude::*;
use yew_functional::*;

#[derive(Properties, Clone, PartialEq)]
pub struct HeaderProps {
    pub on_create: Callback<String>,
}

pub struct HeaderFunction {}
impl FunctionProvider for HeaderFunction {
    type TProps = HeaderProps;

    fn run(props: &Self::TProps) -> Html {
        let (name, set_name) = use_state(|| format!("")); // input state
        let set_name = Rc::new(set_name);

        // oninput
        let handle_input = enclose!((set_name) move |ev: InputData| set_name(ev.value));

        // onkeyup (for detecting "Enter" key)
        let handle_submit = enclose!((name, set_name, props) move |ev: KeyboardEvent| {
            // make sure name is not empty string
            if ev.key() == "Enter" {
                let mut name = name.to_owned().to_string();
                name = name.trim().to_string();
                if name != "" {
                    props.on_create.emit(name);
                    // reset name to blank
                    set_name(format!(""));
                }
            }
        });

        html! {
            <header class="header">
                <h1>{"todos"}</h1>
                <input
                    class="new-todo"
                    placeholder="What needs to be done?"
                    value=&name
                    oninput=Callback::from(handle_input)
                    onkeyup=Callback::from(handle_submit)
                />
            </header>
        }
    }
}
pub type Header = FunctionComponent<HeaderFunction>;

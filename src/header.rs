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
		let props = props.clone();

		// input state
		let (name_s, set_name_s) = use_state(|| format!(""));
		let set_name_s = Rc::new(set_name_s);

		// oninput
		let set_name = set_name_s.clone();
		let handle_input = Callback::from(move |ev: InputData| {
			let set_name = &set_name;
			set_name(ev.value);
		});

		// onkeyup (for detecting "Enter" key)
		let name = name_s.clone();
		let set_name = set_name_s.clone();
		let handle_submit = Callback::from(move |ev: KeyboardEvent| {
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
					value=name_s.clone()
					oninput=handle_input
					onkeyup=handle_submit
				/>
			</header>
		}
	}
}
pub type Header = FunctionComponent<HeaderFunction>;

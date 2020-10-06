use crate::copyright::Copywright;
use crate::footer::Footer;
use crate::header::Header;
use crate::list::List;
use crate::{Filter, TodoEntry, TodoStatus};
use log::*;
use std::rc::Rc;
use yew::prelude::*;
use yew_functional::*;

pub struct AppFunction {}
impl FunctionProvider for AppFunction {
	type TProps = ();

	fn run(_props: &Self::TProps) -> Html {
		info!("rendered!");

		// state
		let (todo_list, set_todo_list) = use_state(|| Vec::<TodoEntry>::new());
		let set_todo_list = Rc::new(set_todo_list);

		let (filter, set_filter) = use_state(|| Filter::All);
		let set_filter = Rc::new(set_filter);

		// callbacks
		let on_create = Callback::from({
			let set_todo_list = set_todo_list.clone();
			let todo_list = todo_list.clone();
			move |todo_name| {
				let new_todo = TodoEntry::new(todo_name);

				set_todo_list({
					let mut todo_list = (*todo_list).clone();
					todo_list.push(new_todo);
					todo_list
				});
			}
		});

		let on_filterchange = Callback::from({
			let set_filter = set_filter.clone();
			move |new_filter| set_filter(new_filter)
		});

		let filtered_todo_list = {
			let todo_list = todo_list.clone();
			move |filter: Filter| match filter {
				Filter::All => todo_list,
				Filter::Active => Rc::new(
					todo_list
						.iter()
						.filter(|todo| todo.status == TodoStatus::Active)
						.map(|todo| todo.clone())
						.collect(),
				),
				Filter::Completed => Rc::new(
					todo_list
						.iter()
						.filter(|todo| todo.status == TodoStatus::Completed)
						.map(|todo| todo.clone())
						.collect(),
				),
			}
		};

		let toggle_completed = Callback::from({
			let set_todo_list = set_todo_list.clone();
			let todo_list = todo_list.clone();
			move |uuid| {
				set_todo_list({
					let mut todo_list = (*todo_list).clone();
					for todo in &mut todo_list {
						if todo.id == uuid {
							todo.toggle_status()
						}
					}
					todo_list
				})
			}
		});

		let clear_todo = Callback::from({
			let set_todo_list = set_todo_list.clone();
			let todo_list = todo_list.clone();
			move |uuid| {
				set_todo_list(
					todo_list
						.iter()
						.filter(|todo| todo.id != uuid)
						.map(|todo| todo.clone())
						.collect(),
				)
			}
		});
		let clear_completed = Callback::from({
			let set_todo_list = set_todo_list.clone();
			let todo_list = todo_list.clone();
			move |_| {
				set_todo_list(
					todo_list
						.iter()
						.filter(|todo| todo.status == TodoStatus::Active)
						.map(|todo| todo.clone())
						.collect(),
				)
			}
		});

		let todos_left = {
			let todo_list = todo_list.clone();
			move || {
				todo_list.iter().fold(0, |acc, todo| {
					if todo.status == TodoStatus::Active {
						acc + 1
					} else {
						acc
					}
				})
			}
		};

		let toggle_complete_all = Callback::from({
			let set_todo_list = set_todo_list.clone();
			let todo_list = todo_list.clone();
			let todos_left = todos_left();
			move |_| {
				set_todo_list({
					if todos_left == 0 {
						// make all todos active
						todo_list
							.iter()
							.map(|todo| {
								let mut todo = todo.clone();
								todo.status = TodoStatus::Active;
								todo
							})
							.map(|todo| todo.clone())
							.collect()
					} else {
						// make all todos completed
						todo_list
							.iter()
							.map(|todo| {
								let mut todo = todo.clone();
								todo.status = TodoStatus::Completed;
								todo
							})
							.collect()
					}
				})
			}
		});

		html! {
			<div id="app">
				<section class="todoapp">
					<Header
						on_create=on_create
					/>
					{
						if todo_list.len() > 0 {
							let todos_left = todos_left();
							html! {
								<>
									<List
										todo_list=filtered_todo_list(*filter)
										toggle_completed=toggle_completed
										clear_todo=clear_todo
										all_completed=todos_left == 0
										toggle_complete_all=toggle_complete_all
									/>
									<Footer
										on_filterchange=on_filterchange
										selected_filter=*filter
										todos_left=todos_left
										todos_completed=todo_list.len() as u32 - todos_left
										clear_completed=clear_completed
									/>
								</>
							}
						}
						else {
							html! {}
						}
					}
				</section>
				<Copywright />
			</div>
		}
	}
}
pub type App = FunctionComponent<AppFunction>;

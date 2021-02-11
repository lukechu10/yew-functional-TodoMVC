use crate::copyright::Copyright;
use crate::footer::Footer;
use crate::header::Header;
use crate::list::List;
use crate::{Filter, TodoEntry, TodoStatus};
use anyhow::Result;
use enclose::enc;
use log::*;
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;
use yew_functional::*;
use yew_services::storage::{Area, StorageService};

const KEY: &'static str = "yew.todomvc.self";

pub struct AppFunction {}
impl FunctionProvider for AppFunction {
    type TProps = ();

    fn run(_props: &Self::TProps) -> Html {
        info!("rendered!");

        let (storage_service, _) = use_state(|| {
            RefCell::new(StorageService::new(Area::Local).expect("storage was disabled by user"))
        });

        let (todo_list, set_todo_list) = use_state(enc!((storage_service) move || {
            // try to restore state from localStorage
            let json: Result<String> = storage_service.borrow().restore(KEY);
            match json {
                Ok(json) => serde_json::from_str(&json).unwrap(),
                Err(_) => Vec::<TodoEntry>::new(),
            }
        }));
        let set_todo_list = Rc::new(set_todo_list);

        let (filter, set_filter) = use_state(|| {
            let hash = web_sys::window().unwrap().location().hash().unwrap();
            match hash.as_str() {
                "#/active" => Filter::Active,
                "#/completed" => Filter::Completed,
                _ => Filter::All,
            }
        });
        let set_filter = Rc::new(set_filter);

        // save todo_list to localStorage
        use_effect_with_deps(
            enc!((storage_service, todo_list) move |_| {
                // serialize as json
                let json = serde_json::to_string(todo_list.as_ref()).unwrap();
                storage_service.borrow_mut().store(KEY, Ok(json));
                move || ()
            }),
            todo_list.clone(),
        );

        let on_create = enc!((set_todo_list, todo_list) move |todo_name: String| {
            let new_todo = TodoEntry::new(todo_name);

            set_todo_list({
                let mut todo_list = (*todo_list).clone();
                todo_list.push(new_todo);
                todo_list
            });
        });

        let on_filterchange = enc!((set_filter) move |new_filter| set_filter(new_filter));

        let filtered_todo_list = enc!((todo_list) move |filter: Filter| match filter {
            Filter::All => todo_list,
            Filter::Active => Rc::new(
                todo_list
                    .iter()
                    .filter(|todo| todo.status == TodoStatus::Active)
                    .cloned()
                    .collect(),
            ),
            Filter::Completed => Rc::new(
                todo_list
                    .iter()
                    .filter(|todo| todo.status == TodoStatus::Completed)
                    .cloned()
                    .collect(),
            ),
        });

        let toggle_completed = enc!((set_todo_list, todo_list) move |uuid| {
            set_todo_list({
                let mut todo_list = (*todo_list).clone();
                for todo in &mut todo_list {
                    if todo.id == uuid {
                        todo.toggle_status()
                    }
                }
                todo_list
            })
        });

        let clear_todo = enc!((set_todo_list, todo_list) move |uuid| {
            set_todo_list(
                todo_list
                    .iter()
                    .filter(|todo| todo.id != uuid)
                    .cloned()
                    .collect(),
            )
        });

        let clear_completed = enc!((set_todo_list, todo_list) move |_| {
            set_todo_list(
                todo_list
                    .iter()
                    .filter(|todo| todo.status == TodoStatus::Active)
                    .cloned()
                    .collect(),
            )
        });

        let rename_todo = enc!((set_todo_list, todo_list) move |(uuid, new_name): (Uuid, String)| {
            set_todo_list(
                todo_list
                    .iter()
                    .map(|todo| {
                        let mut todo = todo.clone();
                        if todo.id == uuid {
                            todo.name = new_name.clone();
                        }
                        todo
                    })
                    .collect(),
            )
        });
        let todos_left = enc!((todo_list) move || {
            todo_list.iter().fold(0, |acc, todo| {
                if todo.status == TodoStatus::Active {
                    acc + 1
                } else {
                    acc
                }
            })
        });

        let toggle_complete_all = enc!((set_todo_list, todo_list, todos_left) move |_| {
            set_todo_list({
                if todos_left() == 0 {
                    // make all todos active
                    todo_list
                        .iter()
                        .map(|todo| {
                            let mut todo = todo.clone();
                            todo.status = TodoStatus::Active;
                            todo
                        })
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
        });

        html! {
            <div id="app">
                <section class="todoapp">
                    <Header
                        on_create=Callback::from(on_create)
                    />
                    {
                        if todo_list.len() > 0 {
                            let todos_left = todos_left();
                            html! {
                                <>
                                    <List
                                        todo_list=filtered_todo_list(*filter)
                                        toggle_completed=Callback::from(toggle_completed)
                                        clear_todo=Callback::from(clear_todo)
                                        all_completed=todos_left == 0
                                        toggle_complete_all=Callback::from(toggle_complete_all)
                                        rename_todo=Callback::from(rename_todo)
                                    />
                                    <Footer
                                        on_filterchange=Callback::from(on_filterchange)
                                        selected_filter=*filter
                                        todos_left=todos_left
                                        todos_completed=todo_list.len() as u32 - todos_left
                                        clear_completed=Callback::from(clear_completed)
                                    />
                                </>
                            }
                        }
                        else {
                            html! {}
                        }
                    }
                </section>
                <Copyright />
            </div>
        }
    }
}
pub type App = FunctionComponent<AppFunction>;

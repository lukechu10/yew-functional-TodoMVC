use crate::copyright::Copyright;
use crate::footer::Footer;
use crate::header::Header;
use crate::list::List;
use crate::{cb, Filter, TodoEntry, TodoStatus};

use anyhow::Result;
use enclose::enc;
use log::*;
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;
use yew_functional::*;
use yew_services::storage::{Area, StorageService};

const KEY: &'static str = "yew.todomvc.self";

#[function_component(App)]
pub fn app(_props: &()) -> Html {
    info!("rendered!");

    let storage_service =
        use_ref(|| StorageService::new(Area::Local).expect("storage was disabled by user"));

    let (todo_list, set_todo_list) = use_state(enc!((storage_service) move || {
        // try to restore state from localStorage
        let json: Result<String> = storage_service.borrow().restore(KEY);
        match json {
            Ok(json) => match serde_json::from_str(&json) {
                Ok(todos) => todos,
                Err(_) => Vec::<TodoEntry>::new(), // bad JSON state
            },
            Err(_) => Vec::<TodoEntry>::new(),
        }
    }));

    let (filter, set_filter) = use_state(|| {
        let hash = web_sys::window().unwrap().location().hash().unwrap();
        match hash.as_str() {
            "#/active" => Filter::Active,
            "#/completed" => Filter::Completed,
            _ => Filter::All,
        }
    });

    // save todo_list to localStorage
    use_effect_with_deps(
        enc!((storage_service, todo_list) move |_| {
            // serialize as json
            let json = serde_json::to_string(todo_list.as_ref()).unwrap();
            storage_service.borrow_mut().store(KEY, Ok(json));
            || ()
        }),
        todo_list.clone(),
    );

    let on_create = cb!((set_todo_list, todo_list) move |todo_name: String| {
        let new_todo = TodoEntry::new(todo_name);

        set_todo_list({
            let mut todo_list = (*todo_list).clone();
            todo_list.push(new_todo);
            todo_list
        });
    });

    let on_filterchange = cb!((set_filter) move |new_filter| set_filter(new_filter));

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

    let toggle_completed = cb!((set_todo_list, todo_list) move |uuid| {
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

    let clear_todo = cb!((set_todo_list, todo_list) move |uuid| {
        set_todo_list(
            todo_list
                .iter()
                .filter(|todo| todo.id != uuid)
                .cloned()
                .collect(),
        )
    });

    let clear_completed = cb!((set_todo_list, todo_list) move |_| {
        set_todo_list(
            todo_list
                .iter()
                .filter(|todo| todo.status == TodoStatus::Active)
                .cloned()
                .collect(),
        )
    });

    let rename_todo = cb!((set_todo_list, todo_list) move |(uuid, new_name): (Uuid, String)| {
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

    let toggle_complete_all = cb!((set_todo_list, todo_list, todos_left) move |_| {
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
                                    rename_todo=rename_todo
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
            <Copyright />
        </div>
    }
}

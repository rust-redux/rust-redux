extern crate rust_redux;

use std::io;
use std::sync::Arc;
use tokio;
use reqwest;
use serde::{Deserialize, Serialize};
use rust_redux::{Store};
use TodoAction::*;
use VisibilityFilter::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct State {
    pub todos: Vec<Todo>,
    pub visibility_filter: VisibilityFilter,
}

impl State {
    pub fn with_defaults() -> State {
        State {
            todos: Vec::new(),
            visibility_filter: VisibilityFilter::ShowAll,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: i16,
    pub title: String,
    pub completed: bool,
    pub deleted: bool,
}

impl Todo {
    pub fn new(id: i16, title: String) -> Todo {
        Todo {
            id: id,
            title: title,
            completed: false,
            deleted: false,
        }
    }
}

pub enum Action {
    Standard(StandardAction),
    Thunk(Arc<dyn Fn(Arc<Store<State, Action>>) + Send + Sync>),
}

impl Clone for Action {
    fn clone(&self) -> Self {
        match self {
            Action::Standard(standard_action) => Action::Standard(standard_action.clone()),
            Action::Thunk(thunk) => Action::Thunk(Arc::clone(thunk)),
        }
    }
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Standard(standard_action) => write!(f, "Action::Standard({:?})", standard_action),
            Action::Thunk(_) => write!(f, "Action::Thunk(...)"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum StandardAction {
    Todos(TodoAction),
    Visibility(VisibilityFilter),
}

#[derive(Clone, Debug)]
pub enum TodoAction {
    Add(String),
    Toggle(i16),
    Remove(i16),
}

#[derive(Clone, Debug)]
pub enum VisibilityFilter {
    ShowActive,
    ShowAll,
    ShowCompleted,
}

fn reducer(state: &State, action: &Action) -> State {
    State {
        todos: todo_reducer(&state.todos, action),
        visibility_filter: visibility_reducer(&state.visibility_filter, action),
    }
}

fn todo_reducer(state: &Vec<Todo>, action: &Action) -> Vec<Todo> {
    let mut new_state: Vec<Todo> = state.clone();

    match action {
        Action::Standard(StandardAction::Todos(todo_action)) => match todo_action {
            Add(title) => {
                let new_id = new_state.len() as i16 + 1;
                new_state.push(Todo::new(new_id, title.to_string()))
            }
            Toggle(todo_id) => {
                if let Some(todo) = get_mut_todo(&mut new_state, *todo_id) {
                    todo.completed = !todo.completed;
                }
            }
            Remove(todo_id) => {
                if let Some(todo) = get_mut_todo(&mut new_state, *todo_id) {
                    todo.deleted = true;
                }
            }
        },
        _ => (),
    }
    new_state
}

fn visibility_reducer(state: &VisibilityFilter, action: &Action) -> VisibilityFilter {
    match action {
        Action::Standard(StandardAction::Visibility(vis_action)) => vis_action.clone(),
        _ => state.clone(),
    }
}

fn get_mut_todo(todos: &mut Vec<Todo>, todo_id: i16) -> Option<&mut Todo> {
    todos.iter_mut().find(|todo|todo.id == todo_id)
}

// Selector for getting visible todos based on the current visibility filter
fn get_visible_todos(state: &State) -> Vec<&Todo> {
    state.todos.iter()
        .filter(|todo| !todo.deleted)
        .filter(|todo| match state.visibility_filter {
            VisibilityFilter::ShowAll => true,
            VisibilityFilter::ShowActive => !todo.completed,
            VisibilityFilter::ShowCompleted => todo.completed,
        })
        .collect()
}

fn print_todo(todo: &Todo) {
    let done = if todo.completed { "" } else { " " };
    println!("[{}] {} {}", done, todo.id, todo.title);
}

fn print_instructions() {
    println!("\nAvailable commands: \nadd [text] - add-random - toggle [id] - remove [id]\nshow [all|active|completed]");
}

fn invalid_command(command: &str) {
    println!("Invalid command: {}", command);
}

fn render(state: &State) {
    let visible_todos = get_visible_todos(state);
    println!("\n\nTodo List:\n-------------------");
    for todo in visible_todos {
        print_todo(&todo);
    }
    println!("-------------------\nVisibility filter:  {:?}", state.visibility_filter);
    print_instructions();
}

fn thunk_middleware(
    store: &Store<State, Action>,
    action: &Action,
    next: &dyn Fn(&Action),
) {
    match action {
        Action::Thunk(thunk) => {
            thunk(Arc::new(store.clone()));
        },
        Action::Standard(_) => next(action),
    }
}

fn logger_middleware(
    store: &Store<State, Action>,
    action: &Action,
    next: &dyn Fn(&Action),
) {
    println!("Dispatching action: {:?}", action);
    next(action);
    println!("New state: {:?}", store.get_state());
}

#[derive(Serialize, Deserialize, Debug)]
struct RandomTodo {
    id: i32,
    todo: String,
    completed: bool,
    #[serde(rename = "userId")]
    user_id: i32,
}

async fn fetch_random_todo() -> Result<RandomTodo, reqwest::Error> {
    let resp = reqwest::get("https://dummyjson.com/todos/random").await?;
    resp.json::<RandomTodo>().await
}

fn add_random_todo(store: Arc<Store<State, Action>>) {
    tokio::spawn(async move {
        match fetch_random_todo().await {
            Ok(random_todo) => {
                let todo_title = random_todo.todo.clone();
                store.dispatch(&Action::Standard(StandardAction::Todos(Add(todo_title))));
                println!("Added random todo: {}", random_todo.todo);
            }
            Err(e) => println!("Failed to fetch random todo: {}", e),
        }
    });
}

#[tokio::main]
async fn main() {
    let store = Arc::new(
        Store::new(reducer as fn(&State, &Action) -> State, State::with_defaults())
            .with_middleware(vec![
                Arc::new(logger_middleware),
                Arc::new(thunk_middleware),
            ])
    );
    
    let store_clone = Arc::clone(&store);
    store.subscribe(move |state| render(state));

    print_instructions();
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("failed to read line");
        let command_parts: Vec<&str> = command.split_whitespace().collect();

        match command_parts.len() {
            0 => invalid_command(&command),
            _ => {
                match command_parts[0] {
                    "add" => store_clone.dispatch(&Action::Standard(StandardAction::Todos(Add(command_parts[1..].join(" ").to_string())))),
                    "add-random" => {
                        let thunk = Arc::new(|store: Arc<Store<State, Action>>| add_random_todo(store));
                        store_clone.dispatch(&Action::Thunk(thunk));
                    },
                    "remove" => if let Ok(num) = command_parts[1].parse::<i16>() {
                        store_clone.dispatch(&Action::Standard(StandardAction::Todos(Remove(num))));
                    },
                    "toggle" => if let Ok(num) = command_parts[1].parse::<i16>() {
                        store_clone.dispatch(&Action::Standard(StandardAction::Todos(Toggle(num))));
                    },
                    "show" => match command_parts[1] {
                        "all" => store_clone.dispatch(&Action::Standard(StandardAction::Visibility(ShowAll))),
                        "active" => store_clone.dispatch(&Action::Standard(StandardAction::Visibility(ShowActive))),
                        "completed" => store_clone.dispatch(&Action::Standard(StandardAction::Visibility(ShowCompleted))),
                        _ => invalid_command(&command)
                    },
                    _ => invalid_command(&command),
                }
            },
        }
    }
}
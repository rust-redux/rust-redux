extern crate rust_redux;

use std::io;
use rust_redux::Store;
use Action::*;
use TodoAction::*;
use VisibilityFilter::*;

use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Clone, Debug)]
pub struct State {
    pub todos: Vec<Todo>,
    pub visibility_filter: VisibilityFilter
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

#[derive(Clone, Debug)]
pub enum Action {
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
    // Always return a new state
    State {
        todos: todo_reducer(&state.todos, action),
        visibility_filter: visibility_reducer(&state.visibility_filter, action),
    }
}

fn todo_reducer(state: &Vec<Todo>, action: &Action) -> Vec<Todo> {
    let mut new_state: Vec<Todo> = state.clone();

    // First we make sure it's a `Todos` action, otherwise return clone of incoming state
    match *action {
        Todos(ref todo_action) => match *todo_action {
            // Pretty simple from here on, check the type of Todos enum type
            // If Add push a new item, and if `Toggle` or `Remove` use our get_mut_todo
            // helper function and then change a property on the todo
            Add(ref title) => {
                let new_id = new_state.len() as i16 + 1;
                new_state.push(Todo::new(new_id, title.to_string()))
            },
            Toggle(todo_id) => {
                if let Some(todo) = get_mut_todo(&mut new_state, todo_id) {
                    if todo.completed { todo.completed = false; } else { todo.completed = true; }
                }
            },
            Remove(todo_id) => {
                if let Some(todo) = get_mut_todo(&mut new_state, todo_id) {
                    todo.deleted = true;
                }
            },
        },
        // If it's not a Todos action change nothing
        _ => (),
    }
    return new_state;
}

fn visibility_reducer(state: &VisibilityFilter, action: &Action) -> VisibilityFilter {
    match *action {
        Visibility(ref vis_action) => vis_action.clone(),
        _ => state.clone(),
    }
}

fn get_mut_todo(todos: &mut Vec<Todo>, todo_id: i16) -> Option<&mut Todo> {
    todos.iter_mut().find(|todo|todo.id == todo_id)
}

fn print_todo(todo: &Todo) {
    let done = if todo.completed { "✔" } else { " " };
    println!("[{}] {} {}", done, todo.id, todo.title);
}

fn print_instructions() {
    println!("\nAvailable commands: \nadd [text] - toggle [id] - remove [id]\nshow [all|active|completed]");
}

fn invalid_command(command: &str) {
    println!("Invalid command: {}", command);
}

fn render(state: &State) {
    let visibility = &state.visibility_filter;
    println!("\n\nTodo List:\n-------------------");
    for i in 0..state.todos.len() {
        let todo = &state.todos[i];
        if !todo.deleted {
            match *visibility {
                ShowAll => print_todo(&todo),
                ShowCompleted => if todo.completed { print_todo(&todo) },
                ShowActive => if !todo.completed { print_todo(&todo) },
            }
        }
    }
    println!("-------------------\nVisibility filter:  {:?}", visibility);
    print_instructions();
}

#[allow(unused_must_use, dead_code, unused_variables)]
fn logger(state:&State, dispatch:&Fn(Action), action: &Action) {
    let mut log_file = OpenOptions::new()
    .write(true)
    .create(true)
    .append(true)
    .open("log.txt")
    .expect("Failed to open log file.");
    log_file.write(b"----------------------------------------------------\n");
    log_file.write(format!("ACTION DISPATCHED: {:?}\n", action).as_bytes());
    log_file.write(b"***************************************************\n");
    log_file.write(b"UPDATED STATE\n");
    log_file.write(b"***************************************************\n");
    log_file.write(format!("Visibility: {:?}\n", state.visibility_filter).as_bytes());
    log_file.write(b"Todo List: \n");
    for todo in &state.todos{
        log_file.write(format!("{:?}\n", todo).as_bytes());
    }
    log_file.write(b"----------------------------------------------------\n\n\n");
}

#[allow(dead_code, unused_variables)]
fn call_dispatch(state:&State, dispatch:&Fn(Action), action: &Action) {
    if let ShowCompleted = state.visibility_filter{
        dispatch(Visibility(ShowAll));
    }
}


fn main() {
    let mut store = Store::create_store(reducer, State::with_defaults());
    store.subscribe(render).apply_middleware(call_dispatch);

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
                    // Since we prepared so well we just need to call dispatch on our store
                    // With the right action
                    "add" => store.dispatch( Todos(Add( command_parts[1..].join(" ").to_string() ))),
                    "remove" => if let Ok(num) = command_parts[1].parse::<i16>() {
                        store.dispatch( Todos(Remove(num)));
                    },
                    "toggle" => if let Ok(num) = command_parts[1].parse::<i16>() {
                        store.dispatch( Todos(Toggle(num)));
                    },
                    "show" => match command_parts[1] {
                        "all" => store.dispatch( Visibility(ShowAll) ),
                        "active" => store.dispatch( Visibility(ShowActive) ),
                        "completed" => store.dispatch( Visibility(ShowCompleted) ),
                        _ => invalid_command(&command)
                    },
                    _ => invalid_command(&command),
                }
            },
        }
    }
}

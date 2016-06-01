mod redux;
use redux::{ Store, Action };
use redux::Action::*;

#[derive(Clone, Debug)]
struct State {
    todos: Vec<Todo>,
    visibility_filter: VisibilityFilter
}

// By implementing a struct we are creating something very much like
// a class, we can attach methods to it refering to &self` or `&mut self`
impl State {
    // This gives us a quick way to initialize a default state with State::default()
    pub fn default() -> State {
        State {
            todos: Vec::new(),
            visibility_filter: VisibilityFilter::ShowAll,
        }
    }
}

// Same Todo as last time..
#[derive(Clone, Debug)]
struct Todo {
    id: i16,
    title: String,
    completed: bool,
    deleted: bool,
}

// Create a convenient Todo::new(id, title) method
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

// Rust has enums, so the enum type can replace the "type" property of Redux objects
// The enums will replace `action creators` too since Todos(Add("Todo item".to_string()))
// is pretty clear
#[derive(Clone, Debug)]
enum Action {
    Todos(TodoAction),
    Visibility(VisibilityFilter),
}

// mark_done from the previous example becomes Toggle to align with the Redux example
// otherwise functionality is the same
#[derive(Clone, Debug)]
enum TodoAction {
    Add(String),
    Toggle(i16),
    Remove(i16),
}

// Our 3 visibility states
#[derive(Clone, Debug)]
enum VisibilityFilter {
    ShowActive,
    ShowAll,
    ShowCompleted,
}

fn reducer<T>(state: T, action: Action) -> T {
    match action {
        Increment => state + 1,
        Decrement => state - 1,
    }
}

fn render<T>(state: T) {
    println!("State: {:?}", state);
}

pub struct SomeStruct<'a, T: 'a> {
    state: &'a [T]
}

fn main() {
    let mut store = Store::create_store(reducer);
    store.subscribe(render);

    store.dispatch(Increment);
    store.dispatch(Decrement);
    store.dispatch(Increment);
    store.dispatch(Decrement);
    store.dispatch(Increment);
    store.dispatch(Decrement);


}

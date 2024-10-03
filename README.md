# rust-redux
An attempt to implement Redux for Rust at the React Europe hackathon.

## Why
If you like Redux elsewhere you might like it in Rust?

## Current state of the project
This is a barebones Redux implementation, and `examples/todo_list` uses Redux to handle the state of a todo app. Now, middleware is also supported, allowing for enhanced action handling like logging or async operations.

## Running To-Do List Example
```sh
git clone https://github.com/fanderzon/rust-redux.git
cd rust-redux/examples/todo_list
cargo run
```

## Usage
### 1. Creating a Store

```rust
let mut store = Store::new(root_reducer, State::with_defaults())
    .with_middleware(vec![Arc::new(logger_middleware)]);
```
This is a standard store creation. Before creating a store, you will need to create a state model of what the store will be storing. We also need a root reducer function that will return an instance of our Redux state model that calls our other individual reducers. Let's start by building our state model.

### 2. Creating State Model
The `State` type used in the to-do list example contains all parts of our rust-redux state.

```rust
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
```
Your state model does not have to be named "State" or have any specific methods of its own. However, it is useful to have a method similar to `with_defaults` as we'll need to pass some default values into `Store::new()`.

### 3. Creating a Root Reducer
You can think of the root reducer as the rust-redux substitute for `combineReducers` in reduxjs. Our root reducer just needs to return our `State` model where each property in our model is set to the return value of its individual reducer. The root reducer must be of type: `fn(&T, U) -> T`

```rust
fn root_reducer(state: &State, action: Action) -> State {
    State {
        todos: todo_reducer(&state.todos, &action),
        visibility_filter: visibility_reducer(&state.visibility_filter, &action),
    }
}
```

### 4. Creating Actions
Actions can be whatever you want them to be. It is only up to your individual reducers to decide how to handle them. The way we decided to build actions was using Rust enums. They allow us to specify a type and a payload without adding extra syntax. Take a look at the `Action` type created in the to-do example.

```rust
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
```
You will notice that we had to create a generic `Action` wrapper around our other two action types. This isn't totally necessary as we could have stuffed all our actions inside a single type, but this is much cleaner. It is important to note that you can only pass a single action type into `Store.dispatch()`. So, if you wish to have multiple action types, you must wrap them up into another type as we have done above, or you can throw all actions into a single type.

### 5. Creating Reducers
Individual reducers decide how you split up your store's state. Similar to reduxjs, reducers should accept some state and an action. Let's look at an example.

```rust
fn todo_reducer(state: &Vec<Todo>, action: &Action) -> Vec<Todo> {
    let mut new_state: Vec<Todo> = state.clone();

    match *action {
        Todos(ref todo_action) => match *todo_action {
            Add(ref title) => {
                let new_id = new_state.len() as i16 + 1;
                new_state.push(Todo::new(new_id, title.to_string()))
            },
            Toggle(todo_id) => {
                if let Some(todo) = get_mut_todo(&mut new_state, todo_id) {
                    todo.completed = !todo.completed;
                }
            },
            Remove(todo_id) => {
                if let Some(todo) = get_mut_todo(&mut new_state, todo_id) {
                    todo.deleted = true;
                }
            }
        },
        _ => (),
    }
    return new_state;
}
```
Another aspect of reduxjs that we want to adapt is avoiding direct mutation of the store's state. As you can see here, we create a clone of the state that is passed in and mutate the clone's properties instead of the state reference that was passed to the reducer. Mutating the passed-in state in this example isn't actually possible, as our `todo_reducer` does not accept a mutable reference to `Vec<Todo>`.

### 6. Middleware
Middleware allows you to modify or monitor actions before they reach the reducer. In this example, we've added a simple logger middleware that logs every action dispatched and the state after each action.

```rust
fn logger_middleware<S, A>(store: Arc<Store<S, A>>, action: A, next: Arc<dyn Fn(A) + Send + Sync>)
where
    S: Clone + Send + 'static + std::fmt::Debug,
    A: Clone + Send + 'static + std::fmt::Debug,
{
    println!("Dispatching action: {:?}", action);
    next(action);
    println!("New state: {:?}", store.get_state());
}
```
To add middleware to the store, use the `with_middleware` method when creating the store:

```rust
let mut store = Store::new(root_reducer, State::with_defaults())
    .with_middleware(vec![Arc::new(logger_middleware)]);
```
This allows you to see all actions being dispatched and the resulting state, making debugging easier.

### 7. Putting it All Together
Now that we have all of our pieces in place, let's subscribe, dispatch, and get our store's state!

### Dispatching Actions
```rust
use Action::*;
fn main() {
    let mut store = Store::new(reducer, State::with_defaults())
        .with_middleware(vec![Arc::new(logger_middleware)]);
    store.dispatch(Todos(Add("Learn about rust-redux".to_string())));
}
```

### Subscribing
Subscribing allows us to have functions that listen to the store's state directly. Whenever an action is dispatched to the store, these functions are called again with the updated state.

```rust
fn update_with_new_state(state: &State) {
    let visibility = &state.visibility_filter;
    println!("Visibility filter updated to:  {:?}", visibility);
}

fn simple_subscribe(state: &State) {
    println!("Nice dispatch!");
}

fn main() {
    let mut store = Store::new(reducer, State::with_defaults())
        .with_middleware(vec![Arc::new(logger_middleware)]);
    store.subscribe(update_with_new_state);
    store.subscribe(simple_subscribe);
}
```

### Getting State
The store's `get_state` method returns an immutable reference to the current state in the rust-redux store.

```rust
fn main() {
    let mut store = Store::new(reducer, State::with_defaults());
    store.dispatch(Todos(Add("Learn about rust-redux".to_string())));
    let my_current_state = store.get_state();
    println!("Current state: {:?}", my_current_state);
}
```


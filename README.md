# rust-redux
An attempt to implement Redux for Rust at the React Europe hackathon.

## Why
No matter what language you are writing the structured way redux handles state is still a very useful pattern.

## Current state of the project
This is a demo and nothing else at this point. `src/lib.rs` has the barebones Redux implementation and `src/main.rs` uses redux to handle the state of a to-do app.


## Running To-Do List Example
```
git clone https://github.com/fanderzon/rust-redux.git
cd rust-redux
cargo run
```

## Usage
### 1. Creating a Store

`let mut store = Store::create_store(root_reducer,State::with_defaults());`

This is a standard store creation. Before creating a store you will need to create a state model of what our store will be storing. We also need a root reducer function that will return an instance of out redux state model that calls your other individual reducers. Let's start by building our state model.

### 2. Creating State Model
The State type seen below that is used in the to-do list example contains all parts of our rust-redux state.

```
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
```

Your state model does not have to be named "State" or have any specific methods of its own. However, it is useful to have a method similar to with_defaults as we'll need to pass some default values into `Store::create_store()`.


### 3. Creating a Root Reducer
You can think of root reducer as the rust-redux substitute for combineReducers in reduxjs. Our root reducer just needs to return our State model where each property in our model is set to the return value of its individual reducer(We'll talk more about individual state reducers later on). The root reducer must be of type: `fn(&T, U) -> T`
```
fn root_reducer(state: &State, action: Action) -> State {
    State {
        todos: todo_reducer(&state.todos, &action),
        visibility_filter: visibility_reducer(&state.visibility_filter,&action),
    }
}
```

### 4. Creating Actions
Actions can be whatever you want them to be. It is only up to your individual reducers to decide how to handle them. The way we decided to build actions was using rust enums. They allow us to specify a type and a payload without adding all that extra syntax. Take a look at the Action type created in the to-do example.

```
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
You will notice that we had to create a generic Action wrapper around our other two action types. This isn't totally necessary as we could have stuffed all our actions inside a single type, but this is much cleaner. It is important to note that you can only pass a single action type into Store.dispatch(). So, if you wish to have multiple action types you must wrap them up into another type as we have done above, or you can throw all actions into a single type.

### 5. Creating Reducers
Individual reducers will decide how you split up your store's state. Similar to reduxjs reducers should accept some state and an action. Let's look at an example.

```
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
                    if todo.completed { todo.completed = false; } else { todo.completed = true; }
                }
            },
            Remove(todo_id) => {
                if let Some(todo) = get_mut_todo(&mut new_state, todo_id) {
                    todo.deleted = true;
                }
            }
        },
        // If it's not a Todos action change nothing
        _ => (),
    }
    return new_state;
}
```
Another aspect of reduxjs that we want to adapt is avoiding direct mutation of the store's state. As you can see here we create a clone of the state that is passed in and mutate the clone's properties instead of the state reference that was passed to the reducer. Mutating the passed in state in this example isn't actually possible as our todo_reducer does not accept a mutable reference to `Vec<Todo>`.

### 6. Putting it All Together
Now that we have all of our pieces in place, let's subscribe, dispatch, and get our store's state!
### Dispatching Actions
```
use Action::*;
fn main(){
	let mut store = Store::create_store(reducer, State::with_defaults());
	store.dispatch(Todos(Add("Learn about rust-redux"));
}
```

### Subscribing
Subscribing allows us to have functions that listen to the store's state directly. Whenever an action is dispatched to the store, these functions are called again with the udpated state.
```
fn update_with_new_state(state: &State) {
	let visibility = &state.visibility_filter;
	println!("Visibility filter updated to:  {:?}", visibility);
}
fn main(){
	let mut store = Store::create_store(reducer, State::with_defaults());
	store.subscribe(update_with_new_state);
}
```
### Getting State
The store's get_state method returns an immutable reference to the current state in the rust-redux store.
```
fn main(){
	let mut store = Store::create_store(reducer, State::with_defaults());
	store.dispatch(Todos(Add("Learn about rust-redux"));
    let myCurrentState = store.get_state();
}
```

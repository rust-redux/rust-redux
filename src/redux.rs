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

#[allow(dead_code)]
pub struct Store {
    state: State,
    listeners: Vec<fn(State)>,
    reducer: fn(State, Action) -> State,
}

#[allow(dead_code)]
impl Store {
    pub fn create_store(reducer: fn(State, Action) -> State) -> Store {
        Store {
            state: State::with_defaults(),
            listeners: Vec::new(),
            reducer: reducer,
        }
    }
    pub fn subscribe(&mut self, listener: fn(State)) {
        self.listeners.push(listener);
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
    }

    pub fn dispatch(&mut self, action: Action) {
        self.state = (self.reducer)(self.state.clone(), action);
        for i in 0..self.listeners.len() {
            self.listeners[i](self.state.clone());
        }
    }
}

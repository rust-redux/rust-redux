// Redux store implementation
#[allow(dead_code)]
pub struct Store {
    state: i32,
    listeners: Vec<fn(i32)>,
    reducer: fn(i32, Action) -> i32,
}

#[allow(dead_code)]
pub enum Action {
    Increment,
    Decrement
}

#[allow(dead_code)]
impl Store {
    // Takes a reducer function, we skip the initial_state and optional arguments
    // TO keep it simple, State::default() from earlier is our initial_state implementation
    pub fn create_store(reducer: fn(i32, Action) -> i32) -> Store {
        Store {
            state: 0,
            listeners: Vec::new(),
            reducer: reducer,
        }
    }

    // Pushes a listener that will be called for any state change
    pub fn subscribe(&mut self, listener: fn(i32)) {
        self.listeners.push(listener);
    }

    // Simply returns the state
    #[allow(dead_code)]
    pub fn get_state(&self) -> i32 {
        self.state
    }

    // Called for every new action, calls the reducer to update the state
    // and then calls every listener
    pub fn dispatch(&mut self, action: Action) {
        self.state = (self.reducer)(self.state, action);
        for i in 0..self.listeners.len() {
            self.listeners[i](self.state.clone());
        }
    }
}

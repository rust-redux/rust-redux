// Unit struct, basically a generic holder
pub struct State;

#[allow(dead_code)]
pub struct Store<T> {
    state: T,
    listeners: Vec<fn(T)>,
    reducer: fn(T, Action) -> T,
}

#[allow(dead_code)]
impl<T: Clone> Store<T> {
    pub fn create_store(reducer: fn(T, Action) -> T) -> Store<T> {
        let zero: i32 = 0;
        Store {
            state: zero,
            listeners: Vec::new(),
            reducer: reducer,
        }
    }

    // Pushes a listener that will be called for any state change
    pub fn subscribe(&mut self, listener: fn(T)) {
        self.listeners.push(listener);
    }

    // Simply returns the state
    #[allow(dead_code)]
    pub fn get_state(&self) -> T {
        self.state
    }

    // Called for every new action, calls the reducer to update the state
    // and then calls every listener
    pub fn dispatch(&mut self, action: Action) {
        self.state = (self.reducer)(self.state, action);
        for i in 0..self.listeners.len() {
            self.listeners[i](self.state);
        }
    }
}

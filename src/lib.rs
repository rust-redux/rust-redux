use std::clone::Clone;

#[allow(dead_code)]
pub struct Store<T: Clone, U> {
    state: T,
    listeners: Vec<fn(&T)>,
    middlewares: Vec<fn(&T, U)>,
    reducer: fn(&T,U) -> T,
}

#[allow(dead_code)]
impl<T: Clone, U> Store<T, U> {
    pub fn create_store(reducer: fn(&T, U) -> T, initial_state: T) -> Store<T, U> {
        Store {
            state: initial_state,
            listeners: Vec::new(),
            middlewares: Vec::new(),
            reducer: reducer,
        }
    }

    pub fn subscribe(&mut self, listener: fn(&T)) {
        self.listeners.push(listener);
    }

    pub fn apply_middleware(&mut self, middleware: fn(&T, U)) {
        self.middlewares.push(middleware);
    }

    pub fn get_state(&self) -> &T {
        &self.state
    }

    pub fn dispatch(&mut self, action:U) {
        self.state = (self.reducer)(&self.state, action.clone());
        for middleware in &self.middlewares{
            middleware(&self.state, action.clone());
        }
        for listener in &self.listeners {
            listener(&self.state)
        }
    }
}

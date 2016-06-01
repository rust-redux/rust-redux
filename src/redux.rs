use std::clone::Clone;

#[allow(dead_code)]
pub struct Store<T: Clone, U> {
    state: T,
    listeners: Vec<fn(T)>,
    reducer: fn(T,U) -> T,
}

#[allow(dead_code)]
impl<T: Clone, U> Store<T, U> {
    pub fn create_store(reducer: fn(T, U) -> T, initial_state: T) -> Store<T, U> {
        Store {
            state: initial_state,
            listeners: Vec::new(),
            reducer: reducer,
        }
    }
    pub fn subscribe(&mut self, listener: fn(T)) {
        self.listeners.push(listener);
    }

    pub fn get_state(&self) -> T {
        self.state.clone()
    }

    pub fn dispatch(&mut self, action: U) {
        self.state = (self.reducer)(self.state.clone(), action);
        for i in 0..self.listeners.len() {
            self.listeners[i](self.state.clone());
        }
    }
}

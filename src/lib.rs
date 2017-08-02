use std::clone::Clone;
use std::cell::RefCell;

#[allow(dead_code)]
pub struct Store<T: Clone, U> {
    state: RefCell<T>,
    listeners: RefCell<Vec<fn(&T)>>,
    middlewares: RefCell<Vec<fn(&Store<T, U>, &U)>>,
    reducer: fn(&T,&U) -> T,
}

#[allow(dead_code)]
impl<T: Clone, U> Store<T, U> {
    pub fn create_store(reducer: fn(&T, &U) -> T, initial_state: T) -> Store<T, U> {
        Store {
            state: RefCell::new(initial_state),
            listeners: RefCell::new(Vec::new()),
            middlewares: RefCell::new(Vec::new()),
            reducer: reducer
        }
    }
    pub fn subscribe(&self, listener: fn(&T)) -> &Store<T, U> {
        self.listeners.borrow_mut().push(listener);
        self
    }

    pub fn get_state(&self) -> &T {
        self.state.borrow()
    }

    pub fn dispatch(&self, action:U) {
        self.state = (self.reducer)(&self.state.borrow(), &action);

        for middleware in self.middlewares.borrow().iter(){
            middleware(&self.state.borrow(), &action);
        }

        for listener in self.listeners.borrow().iter() {
            listener(&self.state.borrow())
        }
    }
}

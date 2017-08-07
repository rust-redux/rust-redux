use std::clone::Clone;
use std::cell::RefCell;
use std::cell::Ref;
use std::cell::RefMut;

// allows the Store's state to be updated without reducers directly mutating the Store.
pub trait Update_Fields <T> {
    fn update_fields(&mut self, T);
}

#[allow(dead_code)]
pub struct Store<T: Clone + Update_Fields<T>, U> {
    state: RefCell<T>,
    listeners: RefCell<Vec<fn(Ref<T>)>>,
    middlewares: RefCell<Vec<fn(&Store<T,U>, &U)>>,
    reducer: fn(Ref<T>,&U) -> T,
}

#[allow(dead_code)]
impl<T: Clone + Update_Fields<T>, U> Store<T, U> {
    pub fn create_store(reducer: fn(Ref<T>, &U) -> T, initial_state: T) -> Store<T, U> {
        Store {
            state: RefCell::new(initial_state),
            listeners: RefCell::new(Vec::new()),
            middlewares: RefCell::new(Vec::new()),
            reducer: reducer
        }
    }
    pub fn subscribe(&self, listener: fn(Ref<T>)) -> &Store<T, U> {
        self.listeners.borrow_mut().push(listener);
        self
    }

    pub fn apply_middleware(&self, middleware:fn(&Store<T,U>, &U)) -> &Store<T,U> {
        self.middlewares.borrow_mut().push(middleware);
        self
    }

    pub fn get_state(&self) -> Ref<T> {
        self.state.borrow()
    }

    pub fn dispatch(&self, action:U) {
        self.state.borrow().update_fields((self.reducer)(self.state.borrow(), &action));

        for middleware in self.middlewares.borrow().iter(){
            middleware(self, &action);
        }

        for listener in self.listeners.borrow().iter() {
            listener(self.state.borrow())
        }
    }
}

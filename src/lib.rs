use std::clone::Clone;
use std::cell::RefCell;
use std::cell::Ref;
use std::cell::RefMut;

#[allow(dead_code)]
pub struct Store<T: Clone, U> {
    state: RefCell<T>,
    listeners: RefCell<Vec<fn(Ref<T>)>>,
    middlewares: RefCell<Vec<fn(&Store<T,U>, &U)>>,
    reducer: fn(RefMut<T>,&U),
}

#[allow(dead_code)]
impl<T: Clone, U> Store<T, U> {
    pub fn create_store(reducer: fn(RefMut<T>, &U), initial_state: T) -> Store<T, U> {
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

    pub fn get_state(&self) -> T {
        *self.state.borrow()
    }

    pub fn dispatch(&self, action:U) {
        //force the State struct to have an updateFields method
        self.state.borrow_mut().update_fields((self.reducer)(self.state.borrow_mut(), &action));

        for middleware in self.middlewares.borrow().iter(){
            middleware(self, &action);
        }

        for listener in self.listeners.borrow().iter() {
            listener(self.state.borrow())
        }
    }
}

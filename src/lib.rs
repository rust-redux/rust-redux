use std::clone::Clone;
use std::cell::RefCell;

#[derive(Copy, Clone)]
struct StateContainer<T: Clone> {
    inner_state: T
}

impl <T:Clone> StateContainer<T> {
    fn new(state:T) -> StateContainer<T>{
        StateContainer{ inner_state: state }
    }
}

#[allow(dead_code)]
pub struct Store<T: Clone, U> {
    state: RefCell<StateContainer<T>>,
    listeners: Vec<fn(&T)>,
    middlewares: Vec<fn(&T, &Fn(U), &U)>,
    reducer: fn(&T,&U) -> T,
}

#[allow(dead_code)]
impl<T: Clone, U> Store<T, U> {
    pub fn create_store(reducer: fn(&T, &U) -> T, initial_state: T) -> Store<T, U> {
        Store {
            state: RefCell::new(StateContainer::new(initial_state)),
            listeners: Vec::new(),
            middlewares: Vec::new(),
            reducer: reducer
        }
    }

    pub fn subscribe(&mut self, listener: fn(&T)) -> &Store<T, U> {
        self.listeners.push(listener);
        self
    }

    pub fn apply_middleware(&mut self, middleware:fn(&T, &Fn(U), &U)) -> &Store<T,U> {
        self.middlewares.push(middleware);
        self
    }

    pub fn get_state(&self) -> T {
        self.state.borrow().inner_state.clone()
    }

    pub fn dispatch(&self, action:U) {
        let updated_state = (self.reducer)(&self.state.borrow().inner_state, &action);
        self.state.borrow_mut().inner_state = updated_state;

        for middleware in &self.middlewares{
            middleware(&self.state.borrow().inner_state, &|action:U|{
                self.dispatch(action);
            }, &action);
        }

        for listener in &self.listeners {
            listener(&self.state.borrow().inner_state)
        }
    }
}

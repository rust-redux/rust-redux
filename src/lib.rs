use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Store<S, A>
where
    S: Clone,
    A: Clone,
{
    state: Arc<Mutex<S>>,
    reducer: fn(&S, &A) -> S,
    subscribers: Arc<Mutex<Vec<Box<dyn Fn(&S) + Send + Sync>>>>,
    middlewares: Vec<Arc<dyn Fn(&Store<S, A>, &A, &dyn Fn(&A)) + Send + Sync>>,
}

impl<S, A> Store<S, A>
where
    S: Clone + Send + Sync + 'static,
    A: Clone + Send + Sync + 'static,
{
    pub fn new(reducer: fn(&S, &A) -> S, initial_state: S) -> Self {
        Store {
            state: Arc::new(Mutex::new(initial_state)),
            reducer,
            subscribers: Arc::new(Mutex::new(Vec::new())),
            middlewares: Vec::new(),
        }
    }

    pub fn add_middleware(&mut self, middleware: Arc<dyn Fn(&Store<S, A>, &A, &dyn Fn(&A)) + Send + Sync>) {
        self.middlewares.push(middleware);
    }

    pub fn dispatch(&self, action: &A) {
        let base_dispatch = |action: &A| {
            let mut state = self.state.lock().unwrap();
            *state = (self.reducer)(&state, action);
            self.notify_subscribers(&state);
        };

        let dispatch = self.middlewares.iter().rev().fold(
            Box::new(base_dispatch) as Box<dyn Fn(&A)>,
            |next, middleware| {
                Box::new(move |action: &A| {
                    middleware(self, action, &|a| next(a));
                }) as Box<dyn Fn(&A)>
            },
        );

        dispatch(action);
    }

    pub fn subscribe<F>(&self, listener: F)
    where
        F: Fn(&S) + 'static + Send + Sync,
    {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.push(Box::new(listener));
    }

    fn notify_subscribers(&self, state: &S) {
        let subscribers = self.subscribers.lock().unwrap();
        for listener in subscribers.iter() {
            listener(state);
        }
    }

    pub fn get_state(&self) -> S {
        self.state.lock().unwrap().clone()
    }

    pub fn with_middleware(mut self, middlewares: Vec<Arc<dyn Fn(&Store<S, A>, &A, &dyn Fn(&A)) + Send + Sync>>) -> Self {
        self.middlewares = middlewares;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Debug)]
    struct TestState {
        pub counter: i32,
    }

    #[derive(Clone, Debug)]
    enum TestAction {
        Increment,
        Decrement,
    }

    fn test_reducer(state: &TestState, action: &TestAction) -> TestState {
        match action {
            TestAction::Increment => TestState {
                counter: state.counter + 1,
            },
            TestAction::Decrement => TestState {
                counter: state.counter - 1,
            },
        }
    }

    fn logger_middleware<S, A>(
        store: &Store<S, A>,
        action: &A,
        next: &dyn Fn(&A),
    ) where
        S: Clone + Send + Sync + 'static + std::fmt::Debug,
        A: Clone + Send + Sync + 'static + std::fmt::Debug,
    {
        println!("Dispatching action: {:?}", action);
        next(action);
        println!("New state: {:?}", store.get_state());
    }

    #[test]
    fn test_store() {
        let initial_state = TestState { counter: 0 };
        let store = Store::new(test_reducer as fn(&TestState, &TestAction) -> TestState, initial_state)
            .with_middleware(vec![Arc::new(logger_middleware)]);

        store.dispatch(&TestAction::Increment);
        assert_eq!(store.get_state().counter, 1);

        store.dispatch(&TestAction::Decrement);
        assert_eq!(store.get_state().counter, 0);
    }

    #[test]
    fn test_subscribe() {
        let initial_state = TestState { counter: 0 };
        let store = Store::new(test_reducer as fn(&TestState, &TestAction) -> TestState, initial_state)
            .with_middleware(vec![Arc::new(logger_middleware)]);

        let observed_states = Arc::new(Mutex::new(Vec::new()));
        let observed_states_clone = Arc::clone(&observed_states);
        store.subscribe(move |state: &TestState| {
            let mut states = observed_states_clone.lock().unwrap();
            states.push(state.counter);
        });

        store.dispatch(&TestAction::Increment);
        store.dispatch(&TestAction::Increment);
        store.dispatch(&TestAction::Decrement);

        let observed_states = observed_states.lock().unwrap();
        assert_eq!(*observed_states, vec![1, 2, 1]);
    }
}

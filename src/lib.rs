use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Store<S, A>
where
    S: Clone,
    A: Clone,
{
    state: Arc<Mutex<S>>,
    reducer: fn(&S, A) -> S,
    subscribers: Arc<Mutex<Vec<Box<dyn FnMut(&S) + Send>>>>,
    middlewares: Vec<Arc<dyn Fn(Arc<Store<S, A>>, A, Arc<dyn Fn(A) + Send + Sync>) + Send + Sync>>,
}

impl<S, A> Store<S, A>
where
    S: Clone + Send + 'static,
    A: Clone + Send + 'static,
{
    pub fn new(reducer: fn(&S, A) -> S, initial_state: S) -> Self {
        Store {
            state: Arc::new(Mutex::new(initial_state)),
            reducer,
            subscribers: Arc::new(Mutex::new(Vec::new())),
            middlewares: Vec::new(),
        }
    }

    pub fn with_middleware(
        mut self,
        middlewares: Vec<
            Arc<dyn Fn(Arc<Store<S, A>>, A, Arc<dyn Fn(A) + Send + Sync>) + Send + Sync>,
        >,
    ) -> Self {
        self.middlewares = middlewares;
        self
    }

    pub fn dispatch(&mut self, action: A) {
        if self.middlewares.is_empty() {
            self.inner_dispatch(action);
        } else {
            let store = Arc::new(self.clone());
            let dispatch_chain = Self::create_dispatch_chain(store);
            dispatch_chain(action);
        }
    }

    fn create_dispatch_chain(store: Arc<Store<S, A>>) -> Arc<dyn Fn(A) + Send + Sync + 'static> {
        let middlewares = store.middlewares.clone();
        let mut dispatch = Arc::new({
            let store = Arc::clone(&store);
            move |action: A| {
                store.inner_dispatch(action);
            }
        }) as Arc<dyn Fn(A) + Send + Sync + 'static>;

        for middleware in middlewares.into_iter().rev() {
            let next = Arc::clone(&dispatch);
            let store_clone = Arc::clone(&store);
            dispatch = Arc::new(move |action: A| {
                middleware(Arc::clone(&store_clone), action, Arc::clone(&next));
            });
        }

        dispatch
    }

    fn inner_dispatch(&self, action: A) {
        let mut state = self.state.lock().unwrap();
        *state = (self.reducer)(&state, action);
        self.notify_subscribers(&state);
    }

    pub fn subscribe<F>(&mut self, listener: F)
    where
        F: FnMut(&S) + 'static + Send,
    {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.push(Box::new(listener));
    }

    fn notify_subscribers(&self, state: &S) {
        let mut subscribers = self.subscribers.lock().unwrap();
        for listener in subscribers.iter_mut() {
            listener(state);
        }
    }

    pub fn get_state(&self) -> S {
        self.state.lock().unwrap().clone()
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

    fn test_reducer(state: &TestState, action: TestAction) -> TestState {
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
        store: Arc<Store<S, A>>,
        action: A,
        next: Arc<dyn Fn(A) + Send + Sync>,
    ) where
        S: Clone + Send + 'static + std::fmt::Debug,
        A: Clone + Send + 'static + std::fmt::Debug,
    {
        println!("Dispatching action: {:?}", action);
        next(action);
        println!("New state: {:?}", store.get_state());
    }

    #[test]
    fn test_store() {
        let initial_state = TestState { counter: 0 };
        let mut store = Store::new(test_reducer, initial_state)
            .with_middleware(vec![Arc::new(logger_middleware)]);

        store.dispatch(TestAction::Increment);
        assert_eq!(store.get_state().counter, 1);

        store.dispatch(TestAction::Decrement);
        assert_eq!(store.get_state().counter, 0);
    }

    #[test]
    fn test_subscribe() {
        let initial_state = TestState { counter: 0 };
        let mut store = Store::new(test_reducer, initial_state)
            .with_middleware(vec![Arc::new(logger_middleware)]);

        let observed_states = Arc::new(Mutex::new(Vec::new()));
        let observed_states_clone = Arc::clone(&observed_states);
        store.subscribe(move |state: &TestState| {
            let mut states = observed_states_clone.lock().unwrap();
            states.push(state.counter);
        });

        store.dispatch(TestAction::Increment);
        store.dispatch(TestAction::Increment);
        store.dispatch(TestAction::Decrement);

        let observed_states = observed_states.lock().unwrap();
        assert_eq!(*observed_states, vec![1, 2, 1]);
    }
}

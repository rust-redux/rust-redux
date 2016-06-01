mod redux;
use redux::{ Store, Action };
use redux::Action::*;

fn reducer(state: i32, action: Action) -> i32 {
    match action {
        Increment => state + 1,
        Decrement => state - 1,
    }
}

fn render(state: i32) {
    println!("State: {}", state);
}

fn main() {
    let mut store = Store::create_store(reducer);
    store.subscribe(render);

    store.dispatch(Increment);
    store.dispatch(Decrement);
    store.dispatch(Increment);
    store.dispatch(Decrement);
    store.dispatch(Increment);
    store.dispatch(Decrement);
}

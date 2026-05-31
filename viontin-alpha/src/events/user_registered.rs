use viontin::fw::events::{Event, Listener, Subscriber, EventDispatcher};

#[derive(Debug)]
pub struct UserRegistered {
    pub name: String,
    pub email: String,
}

impl Event for UserRegistered {}

#[derive(Debug)]
pub struct UserEventSubscriber;

impl Subscriber for UserEventSubscriber {
    fn subscribe(&self, dispatcher: &mut EventDispatcher) {
        dispatcher.listen::<UserRegistered, UserRegisteredHandler>(UserRegisteredHandler);
    }
}

#[derive(Debug)]
pub struct UserRegisteredHandler;

impl Listener<UserRegistered> for UserRegisteredHandler {
    fn handle(&self, event: &UserRegistered) {
        println!("  [event] User registered — {} <{}>", event.name, event.email);
    }
}

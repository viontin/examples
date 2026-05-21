use viontin::fw::events::{Event, Subscribable, Subscriber};

#[derive(Debug)]
pub struct UserRegistered {
    pub name: String,
    pub email: String,
}

impl Event for UserRegistered {}

#[derive(Debug)]
pub struct UserEventSubscriber;

impl Subscriber for UserEventSubscriber {
    fn subscribe(&self, dispatcher: &mut dyn Subscribable) {
        dispatcher.listen(
            "viontin_alpha::events::user_registered::UserRegistered",
            Box::new(UserRegisteredHandler),
        );
    }
}

#[derive(Debug)]
pub struct UserRegisteredHandler;

impl viontin::fw::events::Listener for UserRegisteredHandler {
    fn listens(&self) -> Vec<&'static str> {
        vec!["viontin_alpha::events::user_registered::UserRegistered"]
    }

    fn handle(&self, _event: &dyn Event) {
        println!("  [event] User registered — notification sent");
    }
}

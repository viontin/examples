pub mod user_registered;

use viontin::fw::events::EventDispatcher;

pub fn register_listeners(dispatcher: &mut EventDispatcher) {
    dispatcher.listen::<user_registered::UserRegistered, user_registered::UserRegisteredHandler>(
        user_registered::UserRegisteredHandler,
    );
}

pub fn dispatch_user_registered(dispatcher: &EventDispatcher, name: &str, email: &str) {
    let event = user_registered::UserRegistered {
        name: name.to_string(),
        email: email.to_string(),
    };
    dispatcher.dispatch(&event);
}

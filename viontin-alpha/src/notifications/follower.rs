use viontin::fw::notification::{Notification, Notifiable};

#[derive(Debug, Clone)]
pub struct NewFollower {
    pub from_user: String,
}

impl Notification for NewFollower {
    fn channels(&self) -> Vec<&'static str> {
        vec!["mail"]
    }

    fn to_mail(&self, _notifiable: &dyn Notifiable) -> Option<String> {
        Some(format!("<h1>New Follower</h1><p>{} started following you!</p>", self.from_user))
    }
}

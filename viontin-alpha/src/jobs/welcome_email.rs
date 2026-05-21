use viontin::fw::queue::Job;

#[derive(Debug)]
pub struct SendWelcomeEmail {
    pub user_email: String,
    pub user_name: String,
}

impl Job for SendWelcomeEmail {
    fn handle(self: Box<Self>) -> Result<(), String> {
        println!("  [job] Sending welcome email to {} <{}>", self.user_name, self.user_email);
        Ok(())
    }

    fn name(&self) -> &str {
        "welcome_email"
    }
}

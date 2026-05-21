use viontin::fw::mail::Envelope;

pub fn build(to: &str) -> Envelope {
    Envelope {
        from: Some("noreply@viontin.dev".into()),
        to: vec![to.into()],
        cc: vec![],
        bcc: vec![],
        subject: "Welcome to Viontin!".into(),
        html_body: Some("<h1>Welcome!</h1><p>Thanks for joining.</p>".into()),
        text_body: Some("Welcome! Thanks for joining.".into()),
        attachments: vec![],
    }
}

// Rust Factory Method Pattern Example (single-file project)
// You can place this into src/main.rs of a Cargo project

trait Notification {
    fn send(&self);
}

struct Email;
struct Sms;
struct Push;

impl Notification for Email {
    fn send(&self) {
        println!("Sending EMAIL notification");
    }
}

impl Notification for Sms {
    fn send(&self) {
        println!("Sending SMS notification");
    }
}

impl Notification for Push {
    fn send(&self) {
        println!("Sending PUSH notification");
    }
}

enum NotificationType {
    Email,
    Sms,
    Push,
}

// Factory function (central creation logic)
fn notification_factory(t: NotificationType) -> Box<dyn Notification> {
    match t {
        NotificationType::Email => Box::new(Email),
        NotificationType::Sms => Box::new(Sms),
        NotificationType::Push => Box::new(Push),
    }
}

fn main() {
    let notifications = vec![
        NotificationType::Email,
        NotificationType::Sms,
        NotificationType::Push,
    ];

    for n in notifications {
        let obj = notification_factory(n);
        obj.send();
    }

    // Example of decoupling: client never constructs concrete types directly
    let special = notification_factory(NotificationType::Email);
    special.send();
}

// Sending EMAIL notification
// Sending SMS notification
// Sending PUSH notification
// Sending EMAIL notification
#![allow(unused)]

mod mult_owner_mutable;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage = self.value as f64 / self.max as f64;
        let to_send = match percentage {
            p if p >= 1.0 => Some("Error: you are over your quota!"),
            p if p >= 0.9 => Some("Urgent warning: You've used up over 90% of your quota!"),
            p if p >= 0.75 => Some("Warning: You've used up over 75% of your quota!"),
            _ => None,
        };
        if let Some(msg) = to_send {
            self.messenger.send(msg);
        }
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messages: Vec::new(),
                sent_messages: RefCell::new(Vec::new()),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // self.sent_messages.push(String::from(msg)); // can not borrow it as mutable
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

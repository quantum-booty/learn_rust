pub fn interior_mutability() {}

pub trait Messenger {
    fn send(&self, message: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> Self {
        Self {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> Self {
            Self {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // the interface specifies that &self is an immutable reference
            // but we want to push to its internal cache so it can be tested
            // we need a way to mutate its value without it being a mutable borrow
            // i.e. we want interior mutability
            // we can wrap sent_messages in RecCell, and mutable borrow its value
            // RefCell allows us to have one mutable borrow, and many immutable borrow
            // but this is enforced during runtime
            self.sent_messages.borrow_mut().push(String::from(message));
            // self.sent_messages.borrow_mut().push(String::from(message));
            // second mutable borrow gives run-time error
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        // multiple immutable borrow is allowed
    }
}

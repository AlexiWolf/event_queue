use std::sync::mpsc::*;

use crate::*;

/// Provides an [`EventQueue`] implementation based on [`std::sync::mpsc`].
///
/// This type is used entirely through the [`EventQueue`] trait interfaces.
pub struct MpscEventQueue<E> {
    receiver: Receiver<E>,
}

impl<E> MpscEventQueue<E> {
    /// Creates a new event queue.
    pub fn new() -> (MpscEventSender<E>, Self) {
        let (sender, receiver) = channel();
        let sender = MpscEventSender::from(sender);
        let receiver = Self { receiver };
        (sender, receiver)
    }
}

impl<E: 'static> EventQueue<E> for MpscEventQueue<E> {
    fn next_event(&mut self) -> Option<E> {
        self.receiver.try_recv().ok()
    }
}

pub struct MpscEventSender<E> {
    inner: Sender<E>,
}

unsafe impl<E> Send for MpscEventSender<E> {}
unsafe impl<E> Sync for MpscEventSender<E> {}

impl<E> From<Sender<E>> for MpscEventSender<E> {
    fn from(sender: Sender<E>) -> Self {
        Self { inner: sender }
    }
}

impl<E> EventSender<E> for MpscEventSender<E> {
    fn send_event(&self, event: E) -> Result<(), String> {
        match self.inner.send(event) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string()),
        }
    }
}

#[cfg(test)]
mod event_queue_tests {
    use std::thread;

    pub use super::*;

    #[test]
    pub fn should_send_and_receive_events() {
        let (mut event_queue, event_sender) = MpscEventQueue::new();

        event_sender.send_event(0).unwrap();

        assert_eq!(event_queue.next_event().expect("No event in the queue"), 0);
    }

    #[test]
    pub fn should_send_events_and_receive_events_across_threads() {
        let (mut event_queue, event_sender) = MpscEventQueue::new();

        event_sender.send_event(0).unwrap();
        let thread_sender = event_sender.clone();
        thread::spawn(move || {
            thread_sender.send_event(1).unwrap();
        })
        .join()
        .unwrap();
        event_sender.send_event(2).unwrap();

        assert_eq!(event_queue.next_event().expect("No event in the queue."), 0);
        assert_eq!(event_queue.next_event().expect("No event in the queue."), 1);
        assert_eq!(event_queue.next_event().expect("No event in the queue."), 2);
    }

    #[test]
    pub fn should_flush_empty_list_if_there_are_no_events() {
        let (mut event_queue, _) = MpscEventQueue::<i32>::new();

        assert!(event_queue.next_event().is_none());
    }
}

/// A generic, FIFO, MPSC event queue.
pub trait EventQueue<E> {
    /// Returns the next event in the queue.
    fn next_event(&mut self) -> Option<E>;
}

/// The event-sending half of an [`EventQueue`].
///
/// Event Senders can be freely, and safely cloned, given away, and even sent across threads.
pub trait EventSender<E>: Send + Sync {
    fn send_event(&self, event: E) -> Result<(), String>;
}

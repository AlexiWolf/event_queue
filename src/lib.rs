//! Provides a generic Event-Queue API.
//!
//! This module provides a [FIFO](https://en.wikipedia.org/wiki/FIFO_(computing_and_electronics))
//! (First-in, First-out), MPSC (Multi-Producer, Single-Consumer) event system based on the
//! sender / receiver model found in [std::sync::mpsc] (actually, [MpscEventReceiver] is
//! built on the mpsc API.) This module provides traits which wrap up the channel-like
//! functionality into a nicer API.
//!
//! # Examples
//!
//! All event queues use the same API, so the following examples should work for any type
//! implementing the [`EventQueue`] traits.  
//!
//! ## Create an Event Queue
//!
//! ```
//! # use generic_event_queue::*;
//! # enum EventType { Event };
//! #
//! let (event_sender, event_receiver) = mpsc_event_queue();
//! #
//! # event_sender.send_event(123);
//! ```
//!
//! You can use any custom event-type, or data you'd like when creating an Event Queue.
//! For example, numbers!
//!
//! ```
//! # use generic_event_queue::*;
//! #
//! let (event_sender, event_receiver) = mpsc_event_queue();
//! #
//! # event_sender.send_event(123);
//! ```
//!
//! ## Handling Events
//!
//! An [`EventQueue`] will collect incoming events, and store them until they are ready to be
//! processed.  The order of incoming events is always preserved, and they come out in the same
//! order they came in.  (FIFO, remember?)
//!
//! Queued events are queried in a loop.  Querying events requires you have mutable access to the
//! Event Queue, as the Single-Consumer model can only have *one* event consumer.  By requiring
//! mutable access, we can use Rust's type system better enforce this restriction
//!
//! ```
//! # use generic_event_queue::*;
//! # enum EventType { Event };
//! #
//! # let (event_sender, mut event_receiver) = mpsc_event_queue::<EventType>();
//! #
//! while let Some(event) = event_receiver.next_event() {
//!     match event {
//!         EventType::Event => (), // Handle the event.
//!     }
//! }
//! ```
//!
//! ## Sending Events
//!
//! When we want to send an event to an [`EventQueue`], we use an [`EventSender`].  An event
//! sender is like a tunnel, through which you can send data, and it will pop out on the other
//! side.  
//!
//! ```
//! # use generic_event_queue::*;
//! # enum EventType { Event };
//! # let (event_sender, event_receiver) = mpsc_event_queue(); 
//! #
//! event_sender.send_event(EventType::Event); // Event is sent back to the EventQueue.
//! ```
//!
//! ### Cloning, and Transferring Ownership of an `EventSender`
//!
//! Event Senders are extremely useful because they can be freely, and safely cloned, and their
//! ownership moved to other code that needs to send events.  This enables sending events from
//! code that otherwise does not have access to the Event Queue.
//!
//! ```
//! # use generic_event_queue::*;
//! # #[derive(Clone)]
//! # enum EventType { Event };
//! #
//! # struct SomeOtherType {
//! #     pub event_sender: MpscEventSender<EventType>,
//! # }
//! #
//! # impl SomeOtherType {
//! #   fn new(event_sender: MpscEventSender<EventType>) -> Self {
//! #       Self { event_sender }
//! #   }
//! # }
//! #
//! # fn some_other_function(event_sender: &MpscEventSender<EventType>) {}
//! #
//! let (event_sender, event_receiver) = mpsc_event_queue();
//!
//! // The EventSender can be cloned, and freely passed around.
//! let other_type = SomeOtherType::new(event_sender.clone());
//! some_other_function(&event_sender);
//!
//! // The original EventSender is unaffected.
//! event_sender.send_event(EventType::Event);
//! ```
//!
//! ### Sending an `EventSender` to Another Thread
//!
//! Event Senders can be safely sent across thread boundaries, even when the Event Queue cannot.
//!
//! ```
//! # use generic_event_queue::*;
//! # enum EventType { Event };
//! # let (event_sender, event_receiver) = mpsc_event_queue();
//! #
//! // This EventSender stays on the main thread with the EventQueue.
//! event_sender.send_event(EventType::Event);
//!
//! // The clone is moved to the other thread.
//! let thread_sender = event_sender.clone();
//! std::thread::spawn(move || {
//!     thread_sender.send_event(EventType::Event);
//! }).join();
//! ```

mod event_queue;
pub use event_queue::*;
mod mpsc_event_queue;
pub use mpsc_event_queue::*;

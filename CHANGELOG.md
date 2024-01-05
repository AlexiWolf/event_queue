# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2] - 2024-01-05

- Removed `HasEventSender` trait.
- Removed `Default` impl for `MpscEventQueue`.
- Removed `MpscEventQueue::event_sender()`
- Removed `MpscEventQueue::new()`.

- Renamed `EventQueue` to `EventReceiver`.
- Renamed `MpscEventQueue` to `MpscEventReciever`.

- Added `MpscEventSender`.
- Added `mpsc` module.
- Added `mpsc::event_queue()` fn.
- Added `ReceiverDropppedError`.

- Changed return type of `EventSender::send_event()` to `Result<(), ReceiverDroppedError>`.

## [0.1] - 2023-12-08

- Added `EventQueue` trait.
- Added `EventSender` trait.
- Added `HasEventSender` trait.
- Added `MpscEventQueue` struct.

- All code lovingly stolen from Wolf Engine.

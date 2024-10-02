//! # Rust Simulation Framework
//!
//! This crate provides a flexible framework for simulating discrete-event systems (DES).
//! It allows users to schedule, manage, and execute events over time, making it suitable for
//! simulations of various systems such as queueing networks, resource allocation systems, and more.
//!
//! The core components of the framework are:
//! 
//! - [`Event`]: A struct representing a single event in the simulation, holding its scheduled time, an action, and context.
//! - [`EventScheduler`]: A struct that manages the execution of events, prioritizing those scheduled to run earlier.
//!
//! ## Key Features
//!
//! - **Event Scheduling:** Schedule events at specific times or after delays.
//! - **Event Logging:** Keep a log of all events executed and their outcomes for later analysis.
//! - **Flexible Execution:** Run the scheduler until a certain condition is met, such as reaching a max time.
//! - **Contextual Information:** Attach metadata (context) to each event for richer event processing.
//! 
//! ## Usage Example
//!
//! Below is a simple example demonstrating how to create an event, schedule it in the `EventScheduler`, and run the simulation.
//!
//! ```rust
//! use std::collections::HashMap;
//! use rust_simulation_framework::{Event, EventScheduler};
//!
//! fn main() {
//!     let mut scheduler = EventScheduler::new();
//!     let action = Box::new(|| {
//!         println!("Event executed!");
//!         Some("Event completed.".to_string())
//!     });
//!
//!     let event = Event::new(5.0, Some(action), None);
//!     scheduler.schedule(event);
//!     scheduler.run_until_max_time(10.0);
//! }
//! ```
//!
//! ## Core Structs
//! - [`Event`]: Defines the core event object used to represent scheduled actions.
//! - [`EventScheduler`]: Manages the execution of events over simulated time.
//!
//! ## Customization
//! You can extend the framework by adding custom event types or adjusting how events are scheduled.
//!
//! ## Design Goals
//! This framework is designed to be:
//! 
//! - **Simple to use:** By providing straightforward methods to schedule and run events.
//! - **Flexible:** Allowing users to define custom event behaviors.
//! - **Efficient:** Using a priority queue to ensure events are executed in the correct order.
//!
//! ## Future Directions
//!
//! Planned features include:
//! - **Advanced Scheduling Policies:** Adding support for different event scheduling strategies.
//! - **Performance Optimizations:** Improving efficiency for larger simulations.
//!
//! ## Crate Overview
//! This crate provides all necessary components for event-driven simulations in Rust.
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

/// Represents an event in the simulation.
///
/// Each event has a scheduled time (`time`), an associated action (`action`) 
/// that will be executed when the event occurs, and a `context` for storing 
/// key-value pairs of additional information about the event.
///
/// # Fields
/// - `time`: The time at which the event is scheduled to run.
/// - `action`: A closure that represents the task to be performed when the event is triggered.
///   It returns an `Option<String>` to optionally pass a result when executed.
/// - `context`: A map containing any extra contextual information as key-value pairs (both as `String`).
/// - `active`: A boolean indicating if the event is active. If false, the event will not run.
pub struct Event {
    pub time: f64,
    pub action: Box<dyn FnMut() -> Option<String>>,
    pub context: HashMap<String, String>,
    pub active: bool,
}

// Implement Clone manually for Event
impl Clone for Event {
    /// Creates a clone of the event.
    ///
    /// **Note**: The action closure is not cloned, since closures cannot be cloned. A placeholder
    /// action that returns `None` is used in the cloned event. The `context` and other fields are 
    /// copied as usual.
    fn clone(&self) -> Self {
        Event {
            time: self.time,
            action: Box::new(|| None),  // Placeholder action for clone
            context: self.context.clone(),
            active: self.active,
        }
    }
}

// Implement Event methods
impl Event {
    /// Creates a new `Event` with the given time, action, and context.
    ///
    /// # Parameters
    /// - `time`: The time when the event should be executed.
    /// - `action`: An optional closure representing the event's task. Defaults to a no-op (returns `None`).
    /// - `context`: An optional `HashMap` of context information. Defaults to an empty map.
    ///
    /// # Returns
    /// A new `Event` instance.
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let event = Event::new(5.0, None, None);
    /// assert_eq!(event.time, 5.0);
    /// ```
    pub fn new(time: f64, action: Option<Box<dyn FnMut() -> Option<String>>>, context: Option<HashMap<String, String>>) -> Self {
        Event {
            time,
            action: action.unwrap_or_else(|| Box::new(|| None)),
            context: context.unwrap_or_default(),
            active: true,
        }
    }

    /// Executes the action of the event if it is active.
    ///
    /// # Returns
    /// - `Some(String)`: The result of the action if the event is active and the action produces a result.
    /// - `None`: If the event is inactive or the action produces no result.
    ///
    /// # Example
    /// ```
    /// let mut event = Event::new(0.0, Some(Box::new(|| Some("Executed".to_string()))), None);
    /// assert_eq!(event.run(), Some("Executed".to_string()));
    /// ```
    pub fn run(&mut self) -> Option<String> {
        if self.active {
            (self.action)()
        } else {
            None
        }
    }
}

// Implement ordering traits for Event to use in BinaryHeap
impl PartialEq for Event {
    /// Checks if two events are equal based on their scheduled time.
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    /// Compares two events based on their time, in reverse order, for use in a max-heap.
    ///
    /// This allows events with earlier times to be processed first.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    /// Defines the ordering between two events.
    ///
    /// The event with the earlier time has higher priority, enabling
    /// the `BinaryHeap` to act as a priority queue.
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.partial_cmp(&self.time).unwrap()
    }
}

/// Manages and schedules events using a priority queue.
///
/// The `EventScheduler` executes events based on their scheduled time, maintaining an event log
/// and allowing for conditional execution (e.g., stop after a certain time or when certain criteria are met).
///
/// # Fields
/// - `current_time`: The current time in the simulation, updated as events are processed.
/// - `event_queue`: A binary heap used as a priority queue for storing scheduled events.
/// - `event_log`: A log that stores all events executed and their results.
pub struct EventScheduler {
    pub current_time: f64,
    event_queue: BinaryHeap<Event>,
    pub event_log: Vec<(Event, Option<String>)>,
}

// Implement EventScheduler methods
impl EventScheduler {
    /// Creates a new `EventScheduler` with an empty event queue.
    ///
    /// # Returns
    /// A new `EventScheduler` instance.
    ///
    /// # Example
    /// ```
    /// let scheduler = EventScheduler::new();
    /// assert_eq!(scheduler.current_time, 0.0);
    /// ```
    pub fn new() -> Self {
        EventScheduler {
            current_time: 0.0,
            event_queue: BinaryHeap::new(),
            event_log: Vec::new(),
        }
    }

    /// Schedules a new event by adding it to the event queue.
    ///
    /// # Parameters
    /// - `event`: The event to be scheduled.
    ///
    /// # Example
    /// ```
    /// let mut scheduler = EventScheduler::new();
    /// let event = Event::new(5.0, None, None);
    /// scheduler.schedule(event);
    /// ```
    pub fn schedule(&mut self, event: Event) {
        self.event_queue.push(event);
    }

    /// Schedules a timeout event to be executed after a specified delay.
    ///
    /// # Parameters
    /// - `delay`: The amount of time after which the event should occur.
    /// - `action`: The action to be executed (optional).
    /// - `context`: Additional context for the event (optional).
    ///
    /// # Example
    /// ```
    /// let mut scheduler = EventScheduler::new();
    /// scheduler.timeout(10.0, Some(Box::new(|| Some("Timeout event".to_string()))), None);
    /// ```
    pub fn timeout(&mut self, delay: f64, action: Option<Box<dyn FnMut() -> Option<String>>>, context: Option<HashMap<String, String>>) {
        let event = Event::new(self.current_time + delay, action, context);
        self.schedule(event);
    }

    /// Runs the event scheduler until a stop condition is met.
    ///
    /// # Parameters
    /// - `stop`: A closure that takes a reference to the scheduler and returns `true` when the scheduler should stop.
    /// - `log_filter`: An optional closure that determines whether to log an event. Defaults to logging all events.
    ///
    /// # Returns
    /// A vector of executed events along with their results.
    ///
    /// # Example
    /// ```
    /// let mut scheduler = EventScheduler::new();
    /// scheduler.timeout(5.0, Some(Box::new(|| Some("Event executed".to_string()))), None);
    /// let stop_fn = Box::new(|s: &EventScheduler| s.current_time >= 10.0);
    /// scheduler.run(stop_fn, None);
    /// ```
    pub fn run(&mut self, stop: Box<dyn Fn(&Self) -> bool>, log_filter: Option<Box<dyn Fn(&Event, &Option<String>) -> bool>>) -> Vec<(Event, Option<String>)> {
        let log_filter = log_filter.unwrap_or_else(|| Box::new(|_, _| true));
        while !stop(self) {
            if let Some(mut event) = self.event_queue.pop() {
                self.current_time = event.time;
                let event_result = event.run();
                if log_filter(&event, &event_result) {
                    self.event_log.push((event, event_result));
                }
            } else {
                break;
            }
        }
        self.event_log.clone()
    }

    /// Runs the event scheduler until a specified maximum time is reached.
    ///
    /// This is a convenience method that calls `run` with a predefined stop condition based on `max_time`.
    ///
    /// # Parameters
    /// - `max_time`: The maximum simulation time.
    ///
    /// # Returns
    /// A vector of executed events along with their results.
    ///
    /// # Example
    /// ```
    /// let mut scheduler = EventScheduler::new();
    /// scheduler.timeout(5.0, Some(Box::new(|| Some("Timeout event".to_string()))), None);
    /// scheduler.run_until_max_time(10.0);
    /// ```
    pub fn run_until_max_time(&mut self, max_time: f64) -> Vec<(Event, Option<String>)> {
        self.run(Box::new(stop_at_max_time_factory(max_time)), None)
    }
}

// Stop function to halt the simulation at a maximum time
/// A factory function to create a stop condition that halts the simulation after a maximum time.
///
/// # Parameters
/// - `max_time`: The maximum simulation time.
///
/// # Returns
/// A closure that returns `true` when the scheduler's current time
fn stop_at_max_time_factory(max_time: f64) -> Box<dyn Fn(&EventScheduler) -> bool> {
    Box::new(move |scheduler: &EventScheduler| {
        scheduler.current_time >= max_time
            || scheduler.event_queue.peek().map_or(true, |event| event.time >= max_time)
    })
}


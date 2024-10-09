# Overview

DESRu (pronounced "dez-ruh") is a discrete event simulation package for Rust.

This Rust crate provides a flexible framework for simulating Discrete Event Systems (DES). It allows users to schedule, manage, and execute events over time, making it suitable for simulating systems such as queueing networks, resource allocation systems, and other event-driven models.

# Key Features

- **Event Scheduling**: Schedule events to occur at specific times or after delays.
- **Event Logging**: Keep a log of all executed events and their outcomes for post-simulation analysis.
- **Flexible Execution**: Run simulations for a specific duration or until a custom stopping condition is met.
- **Contextual Information**: Attach metadata to events for richer simulation context and behavior customization.

# Getting Started

## Installation

```bash
$ cargo add desru
```

## Examples of Usage

### Scheduling an Event

```rust
use desru::{Event, EventScheduler};

fn main() {
    let mut scheduler = EventScheduler::new();
    let event = Event::new(
        0.0, 
        Some(Box::new(|scheduler| Some("Executed".to_string()))), 
        None
    );
    
    scheduler.schedule(event);
    scheduler.run_until_max_time(10.0);
}
```

### Simple Car Process

This example replicates the classic SimPy Car simulation, where a car alternates between parking and driving.

```rust
use desru::{Event, EventScheduler};

const PARK_DURATION: f64 = 5.0;
const DRIVE_DURATION: f64 = 2.0;

fn car(scheduler: &mut EventScheduler) {
    park(scheduler);
}

fn park(scheduler: &mut EventScheduler) {
    println!("Start parking at {}", scheduler.current_time);
    scheduler.schedule(Event::new(
        scheduler.current_time + PARK_DURATION,
        Some(Box::new(move |scheduler: &mut EventScheduler| {
            drive(scheduler);
            None
        })),
        None,
    ));
}

fn drive(scheduler: &mut EventScheduler) {
    println!("Start driving at {}", scheduler.current_time);
    scheduler.schedule(Event::new(
        scheduler.current_time + DRIVE_DURATION,
        Some(Box::new(move |scheduler: &mut EventScheduler| {
            park(scheduler);
            None
        })),
        None,
    ));
}

fn main() {
    let mut scheduler = EventScheduler::new();
    car(&mut scheduler);
    scheduler.run_until_max_time(15.0);
}
```

### Object-Oriented Car Process

This example uses a more object-oriented approach to simulate a car alternating between charging and driving.

```rust
use desru::{Event, EventScheduler};

const CHARGE_DURATION: f64 = 5.0;
const TRIP_DURATION: f64 = 2.0;

struct Car<'a> {
    scheduler: &'a mut EventScheduler,
}

impl<'a> Car<'a> {
    fn new(scheduler: &'a mut EventScheduler) -> Self {
        let mut car = Car { scheduler };
        car.charge();
        car
    }

    fn charge(&mut self) {
        println!("Start charging at {}", self.scheduler.current_time);
        self.scheduler.schedule(Event::new(
            self.scheduler.current_time + CHARGE_DURATION,
            Some(Box::new(move |scheduler: &mut EventScheduler| {
                let mut car_instance = Car { scheduler };
                car_instance.drive();
                None
            })),
            None,
        ));
    }

    fn drive(&mut self) {
        println!("Start driving at {}", self.scheduler.current_time);
        self.scheduler.schedule(Event::new(
            self.scheduler.current_time + TRIP_DURATION,
            Some(Box::new(move |scheduler: &mut EventScheduler| {
                let mut car_instance = Car { scheduler };
                car_instance.charge();
                None
            })),
            None,
        ));
    }
}

fn main() {
    let mut scheduler = EventScheduler::new();
    let _car = Car::new(&mut scheduler);
    scheduler.run_until_max_time(15.0);
}
```

# Core Components

The Event struct represents a discrete event in the simulation. Each event has:

- A scheduled time.
- A closure (the action) to be executed when the event is triggered.
- Contextual metadata for storing key-value pairs.

The `EventScheduler` manages the execution of events. It processes events in order of their scheduled times, executing them and then advancing the simulation time.

# Design Philosophy

- Keep it simple.
- Keep it performant.
- Keep it to the fundamentals.


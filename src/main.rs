// NOTE:
// Final Design Goals:
// * State Machine with multiple paths (not just linear)
// * Simple interface for running through states
// * Includes an ErrorState (that can be entered from any other state)
// * Each state reads inputs and verifies that against a target
// * Each state may hold values unique to it
// * Valid state transitions are verified at compile time

trait StateMachine {
    fn run(&self);
    fn end_condition_met(&self) -> bool;
}

trait NextState<S>: Sized
where
    GlorifiedKettle<S>: From<Self>,
    GlorifiedKettle<SafetyCutOff>: From<Self>,
{
    fn next(self) -> GlorifiedKettle<S> {
        self.into()
    }

    fn error(self) -> GlorifiedKettle<SafetyCutOff> {
        self.into()
    }
}

#[derive(Clone, Copy)]
struct GlorifiedKettle<S> {
    state: S,
}

impl GlorifiedKettle<Filling> {
    fn new() -> Self {
        GlorifiedKettle {
            state: (Filling {}),
        }
    }
}

#[derive(Clone, Copy)]
struct Filling {
    // end_level: u8,
}

struct Heating {
    end_temperature: u8,
    actual_temperature: f32,
}

struct Emptying {
    // end_level: u8,
}

struct SafetyCutOff {
    // end_level: u8,
}

impl From<GlorifiedKettle<Filling>> for GlorifiedKettle<Heating> {
    fn from(_val: GlorifiedKettle<Filling>) -> GlorifiedKettle<Heating> {
        GlorifiedKettle {
            state: Heating {
                end_temperature: MAX_TEMPERATURE,
                actual_temperature: 0.0,
            },
        }
    }
}

impl From<GlorifiedKettle<Heating>> for GlorifiedKettle<Emptying> {
    fn from(_val: GlorifiedKettle<Heating>) -> GlorifiedKettle<Emptying> {
        GlorifiedKettle { state: Emptying {} }
    }
}

impl From<GlorifiedKettle<Emptying>> for GlorifiedKettle<Filling> {
    fn from(_val: GlorifiedKettle<Emptying>) -> GlorifiedKettle<Filling> {
        GlorifiedKettle { state: Filling {} }
    }
}

impl From<GlorifiedKettle<Filling>> for GlorifiedKettle<SafetyCutOff> {
    fn from(_val: GlorifiedKettle<Filling>) -> GlorifiedKettle<SafetyCutOff> {
        GlorifiedKettle {
            state: SafetyCutOff {},
        }
    }
}

impl From<GlorifiedKettle<Heating>> for GlorifiedKettle<SafetyCutOff> {
    fn from(_val: GlorifiedKettle<Heating>) -> GlorifiedKettle<SafetyCutOff> {
        GlorifiedKettle {
            state: SafetyCutOff {},
        }
    }
}

impl From<GlorifiedKettle<Emptying>> for GlorifiedKettle<SafetyCutOff> {
    fn from(_val: GlorifiedKettle<Emptying>) -> GlorifiedKettle<SafetyCutOff> {
        GlorifiedKettle {
            state: SafetyCutOff {},
        }
    }
}

impl StateMachine for GlorifiedKettle<Filling> {
    fn run(&self) {
        if !self.end_condition_met() {
            fill();
        }
    }

    fn end_condition_met(&self) -> bool {
        read_level() == FULL
    }
}

impl StateMachine for GlorifiedKettle<Heating> {
    fn run(&self) {
        if !self.end_condition_met() {
            heat();
        }
    }

    fn end_condition_met(&self) -> bool {
        read_temperature() == self.state.end_temperature
    }
}

impl NextState<Heating> for GlorifiedKettle<Filling> {}
impl NextState<Emptying> for GlorifiedKettle<Heating> {}
impl NextState<Filling> for GlorifiedKettle<Emptying> {}

impl StateMachine for Emptying {
    fn run(&self) {
        if !self.end_condition_met() {
            empty();
        }
    }

    fn end_condition_met(&self) -> bool {
        read_level() == EMPTY
    }
}

// Filler Content for now
fn fill() {}
fn read_level() -> u8 {
    50
}
fn heat() {}
fn read_temperature() -> u8 {
    60
}
fn empty() {}

const EMPTY: u8 = 0;
const MAX_TEMPERATURE: u8 = 90;
const FULL: u8 = 100;
// ----------------------

fn main() {
    let state = GlorifiedKettle::new();
    loop {
        state.run();

        if !state.end_condition_met() {
            continue;
        }

        let state = state.next();
    }
}

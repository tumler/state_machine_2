use std::{thread, time};

enum States {
    Filling(GlorifiedKettle<Filling>),
    Heating(GlorifiedKettle<Heating>),
    Emptying(GlorifiedKettle<Emptying>),
    SafetyCutOff(GlorifiedKettle<SafetyCutOff>),
}

impl States {
    fn step(self) -> States {
        println!("Changing States");
        match self {
            States::Filling(value) => States::Heating(value.into()),
            States::Heating(value) => States::Emptying(value.into()),
            States::Emptying(value) => States::Filling(value.into()),
            _ => States::SafetyCutOff(GlorifiedKettle {
                current_level: 0,
                current_temperature: 0,
                state: SafetyCutOff {},
            }),
        }
    }

    fn error(self) -> States {
        States::SafetyCutOff(GlorifiedKettle {
            current_level: 0,
            current_temperature: 0,
            state: SafetyCutOff {},
        })
    }

    fn run(&mut self) -> Result<(), &'static str> {
        println!("Running State");
        match self {
            States::Filling(value) => value.run(),
            States::Heating(value) => value.run(),
            States::Emptying(value) => value.run(),
            _ => Err("Not in state"),
        }
    }

    fn is_end_condition_met(&self) -> bool {
        println!("Running State");
        match self {
            States::Filling(value) => value.is_end_condition_met(),
            States::Heating(value) => value.is_end_condition_met(),
            States::Emptying(value) => value.is_end_condition_met(),
            _ => false,
        }
    }
}

trait StateMachine {
    fn run(&mut self) -> Result<(), &'static str>;
    fn is_end_condition_met(&self) -> bool;
}

struct GlorifiedKettle<S> {
    current_level: u8,
    current_temperature: u8,
    state: S,
}

impl<S> GlorifiedKettle<S> {
    fn read_inputs(&mut self) {}
}

impl GlorifiedKettle<Filling> {
    fn new() -> Self {
        GlorifiedKettle {
            current_level: 0,
            current_temperature: BASE_TEMPERATURE,
            state: (Filling { end_level: FULL }),
        }
    }

    fn fill(&mut self) {
        self.current_level += STEP_LEVEL;
    }
}

impl GlorifiedKettle<Heating> {
    fn heat(&mut self) {
        self.current_temperature += STEP_TEMPERATURE;
    }
}

impl GlorifiedKettle<Emptying> {
    fn empty(&mut self) {
        self.current_level -= STEP_LEVEL;
    }
}

struct Filling {
    end_level: u8,
}

struct Heating {
    end_temperature: u8,
}

struct Emptying {
    end_level: u8,
}

struct SafetyCutOff {
    // end_level: u8,
}

impl From<GlorifiedKettle<Filling>> for GlorifiedKettle<Heating> {
    fn from(val: GlorifiedKettle<Filling>) -> GlorifiedKettle<Heating> {
        GlorifiedKettle {
            current_level: val.current_level,
            current_temperature: val.current_temperature,
            state: Heating {
                end_temperature: MAX_TEMPERATURE,
            },
        }
    }
}

impl From<GlorifiedKettle<Heating>> for GlorifiedKettle<Emptying> {
    fn from(val: GlorifiedKettle<Heating>) -> GlorifiedKettle<Emptying> {
        GlorifiedKettle {
            current_level: val.current_level,
            current_temperature: val.current_temperature,
            state: Emptying { end_level: EMPTY },
        }
    }
}

impl From<GlorifiedKettle<Emptying>> for GlorifiedKettle<Filling> {
    fn from(val: GlorifiedKettle<Emptying>) -> GlorifiedKettle<Filling> {
        GlorifiedKettle {
            current_level: val.current_level,
            current_temperature: BASE_TEMPERATURE, //TODO: Update this -> This should not be reset here...
            state: Filling { end_level: FULL },
        }
    }
}

impl From<GlorifiedKettle<Filling>> for GlorifiedKettle<SafetyCutOff> {
    fn from(val: GlorifiedKettle<Filling>) -> GlorifiedKettle<SafetyCutOff> {
        GlorifiedKettle {
            current_level: val.current_level,
            current_temperature: val.current_temperature,
            state: SafetyCutOff {},
        }
    }
}

impl From<GlorifiedKettle<Heating>> for GlorifiedKettle<SafetyCutOff> {
    fn from(val: GlorifiedKettle<Heating>) -> GlorifiedKettle<SafetyCutOff> {
        GlorifiedKettle {
            current_level: val.current_level,
            current_temperature: val.current_temperature,
            state: SafetyCutOff {},
        }
    }
}

impl From<GlorifiedKettle<Emptying>> for GlorifiedKettle<SafetyCutOff> {
    fn from(val: GlorifiedKettle<Emptying>) -> GlorifiedKettle<SafetyCutOff> {
        GlorifiedKettle {
            current_level: val.current_level,
            current_temperature: val.current_temperature,
            state: SafetyCutOff {},
        }
    }
}


impl StateMachine for GlorifiedKettle<Filling> {
    fn run(&mut self) -> Result<(), &'static str> {
        self.read_inputs();

        // TODO: Add check if values are too high / if there is an error?
        println!("Filling");
        if !self.is_end_condition_met() {
            self.fill();
            println!("Level: {}", self.current_level);
        }
        Ok(())
    }

    fn is_end_condition_met(&self) -> bool {
        self.current_level >= self.state.end_level
    }
}

impl StateMachine for GlorifiedKettle<Heating> {
    fn run(&mut self) -> Result<(), &'static str> {
        self.read_inputs();

        println!("Heating");
        if !self.is_end_condition_met() {
            self.heat();
            println!("Heating: {}", self.current_temperature);
        }
        Ok(())
    }

    fn is_end_condition_met(&self) -> bool {
        self.current_temperature >= self.state.end_temperature
    }
}

impl StateMachine for GlorifiedKettle<Emptying> {
    fn run(&mut self) -> Result<(), &'static str> {
        self.read_inputs();

        println!("Emptying");
        if !self.is_end_condition_met() {
            self.empty();
            println!("Level: {}", self.current_level);
        }
        Ok(())
    }

    fn is_end_condition_met(&self) -> bool {
        self.current_level <= self.state.end_level
    }
}

// Filler Content
const BASE_TEMPERATURE: u8 = 25;
const STEP_TEMPERATURE: u8 = 5;
const STEP_LEVEL: u8 = 10;
// ----

const EMPTY: u8 = 0;
const MAX_TEMPERATURE: u8 = 80;
const FULL: u8 = 100;

fn main() {
    let mut state = States::Filling(GlorifiedKettle::new());
    loop {
        if state.run().is_err() {
            state = state.error();
        }

        if state.is_end_condition_met() {
            state = state.step();
        }

        println!("Current State:");
        match state {
            States::Filling(_) => println!("Filling"),
            States::Heating(_) => println!("Heating"),
            States::Emptying(_) => println!("Emptying"),
            States::SafetyCutOff(_) => println!("SafetyCutOff"),
        }

        thread::sleep(time::Duration::from_secs(2));
    }
}

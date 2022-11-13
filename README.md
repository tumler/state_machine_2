# State Machine

Initially based off [State_Machine](https://github.com/tumler/state_machine) which is inspired by the [Pretty State Machine Patterns in Rust](https://hoverbear.org/blog/rust-state-machine-pattern/) Blog Post.

## Design Goals

* Transitions are checked at compile time
* States wrapped in an enum for flexibility
* Clear and easy to use interfaces
* Linear State Machine with an Error State (Safety Cut Out State - accessible from any other state)
    * Error State exists but logic needs to be added for checking values considered errors (for a real implementation the Error State can be tranisitioned to from within an ISR - example: initiated from a button)

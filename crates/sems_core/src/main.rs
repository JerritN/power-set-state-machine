use sems_core::{Id, StateMachine, Truth};


fn main() {
    let mut state_machine = StateMachine::new();
    println!("{:?}", state_machine.can_run(t_a));
    println!("{:?}", state_machine.can_run(t_b));
    println!("{:?}", state_machine.run(t_a));
    println!("{:?}", state_machine.can_run(t_b));
}

#[derive(Debug)]
struct A();

impl Truth for A {
    fn id() -> Id {
        "A"
    }
}

fn t_a(_:()) -> A {
    A()
}

fn t_b(a: A) {
    println!("{:?}", a);
}
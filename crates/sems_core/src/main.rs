use sems_core::{StateMachine, Truth, transition::IntoTransitionMut};
use sems_macro::Truth;

fn main() {
    let mut state_machine = StateMachine::new();
    state_machine.set_truth(A(5));

    let mut vec = Vec::new();
    let insert_a = |a: A| _ = &mut vec.push(a.0);
    state_machine.run(insert_a).unwrap();

    println!("{:?}", vec);
}

#[derive(Debug,Truth)]
struct A(i32);

#[derive(Debug,Truth)]
struct B();
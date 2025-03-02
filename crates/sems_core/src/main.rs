use std::any::TypeId;

use sems_core::{Id, StateMachine, Truth};
use sems_macro::Truth;

fn main() {
    let mut state_machine = StateMachine::new();

    state_machine.set_truth(A());
    state_machine.run(t_d).unwrap();
}

#[derive(Debug,Truth)]
struct A();

#[derive(Debug,Truth)]
struct B();

fn t_a() -> A {
    A()
}

fn t_b(a: A) {
    println!("{:?}", a);
}

fn t_c(a: Option<A>) {
    println!("{:?}", a);
}

fn t_d(a: A, b: B) {
    println!("{:?},{:?}", a,b);
}

fn t_e(a: (A,B), b: B) {

}
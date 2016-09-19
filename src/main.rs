#![feature(specialization)]

extern crate child;

use child::Child;

struct Alice;

impl Child for Alice {
    type Parent = ();
}

struct Bob;

impl Child for Bob {
    type Parent = Alice;
}

trait FooChild { }

impl<T> FooChild for T
    where T: Child, T::Parent: Foo
{
}



trait Foo { fn foo(&self); }

impl Foo for Alice {
    fn foo(&self) { println!("Alice foo!"); }
}

// Implement for all children with parents that implement `Foo`
impl<T> Foo for T
    where T: FooChild
{
    default fn foo(&self) { println!("Descendant foo!"); }
}



fn main() {
    Alice.foo();
    Bob.foo();
}

#![feature(specialization)]

extern crate marker;

use marker::Marker;

trait Foo { fn foo(&self); }

struct Fizz;

impl Marker for Fizz {
    type Mark = ();
}

impl Foo for Fizz {
    fn foo(&self) { println!("Fizz!"); }
}

trait FooMarker { }

impl<T> FooMarker for T
    where T: Marker, T::Mark: Foo
{

}

impl<T> Foo for T
    where T: FooMarker
{
    default fn foo(&self) { println!("Has Foo marker!"); }
}

struct Buzz;

impl Marker for Buzz {
    type Mark = Fizz;
}

fn main() {
    Fizz.foo();
    Buzz.foo();
}

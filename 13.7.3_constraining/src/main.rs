use std::fmt::Display;

struct C<T>(T);

trait t {
    type A;

    fn inner(&self) -> Self::A;
}

impl<T: Clone> t for C<T> {
    type A = T;

    fn inner(&self) -> Self::A {
        self.0.clone()
    }
}

fn printer<C>(c: &C) where
C: t,
C::A: Display {
    println!("{}", c.inner());
}

fn p<C>(c: &C) where
C: t<A = i32> {
    println!("{}", c.inner());
}

fn main() {
    let c = C(4i32);

    println!("{}", c.inner());

    printer(&c);
    p(&c);
}

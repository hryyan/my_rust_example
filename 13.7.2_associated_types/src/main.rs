struct Pair( i32, i32 );

trait t {
    type A;
    type B;

    fn contains(&self, &Self::A, &Self::B) -> bool;
    fn first(&self) -> i32;
    fn second(&self) -> i32;
}

impl t for Pair {
    type A = i32;
    type B = i32;

    fn contains(&self, a: &i32, b: &i32) -> bool {
        (&self.0 == a) && (&self.1 == b)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn second(&self) -> i32 {
        self.1
    }
}

fn different<C: t>(c: C) -> i32 {
    c.second() - c.first()
}

fn main() {
    let a = 3;
    let b = 10;

    let p = Pair(a, b);

    println!("The different is {}", different(p));
}

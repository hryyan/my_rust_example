struct Pair( i32, i32 );

trait Contains<A, B> {
    fn contains(&self, &A, &B) -> bool;
    fn first(&self) -> i32;
    fn second(&self) -> i32;
}

impl Contains<i32, i32> for Pair {
    fn contains(&self, f: &i32, s: &i32) -> bool {
        (&self.0 == f) && (&self.1 == s)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn second(&self) -> i32 {
        self.1
    }
}

fn different<A, B, C>(contains: &C) -> i32 where
C: Contains<A, B>{
    contains.first() - contains.second()
}

fn main() {
    let number = 3;
    let digit = 10;

    let container = Pair(number, digit);
    println!("The different is: {}", different(&container));
}

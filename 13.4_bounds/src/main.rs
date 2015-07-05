use std::fmt::Debug;

trait a_trait {
    fn a_trait_function(&self) -> f64;
}

impl a_trait for R {
    fn a_trait_function(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct R { length: f64, height: f64 }
#[allow(dead_code)]
struct T { length: f64, height: f64 }

fn print_debug <T: Debug> (t: &T) {
    println!("{:?}", t);
}

fn a_trait_function<T: a_trait> (t: &T) {
    t.a_trait_function();
}

fn main() {
    let r = R { length:3.0, height:2.0};
    let t = T { length:5.0, height:4.0};

    print_debug(&r);
}

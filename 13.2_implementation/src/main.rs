struct Val(f64,);
struct GenericVal<T>(T,);

impl Val {
    fn value(&self) -> &f64 {
        &self.0
    }
}

impl <T> GenericVal<T> {
    fn value(&self) -> &T {
        &self.0
    }
}

fn main() {
    let a = Val(3.0);
    let b = GenericVal(5i32);

    println!("{}, {}", a.value(), b.value());
}

struct Empty;
struct Null;

trait trait_test<T> {
    fn test(&self, _: T);
}

impl <T, U> trait_test<T> for U {
    fn test(&self, _: T) {
        println!("trait test");
    }
}

fn main() {
    let empty = Empty;
    let null = Null;

    empty.test(null);

    empty;
}

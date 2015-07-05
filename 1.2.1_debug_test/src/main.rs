#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Deep(DebugPrintable);

fn main() {
    println!("{:?}", DebugPrintable(3));
    println!("{:?}", Deep(DebugPrintable(3)));
}


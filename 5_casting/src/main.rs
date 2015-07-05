fn get() -> i32{
    1
}
fn main() {
    let decimal = 65.4321_f32;

    let integer: u8 = decimal as u8;
    let character = integer as char;
    println!("{}, {}", integer, character);
    println!("{}", get());
}

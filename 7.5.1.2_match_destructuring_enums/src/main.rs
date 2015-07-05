#[allow(dead_code)]
#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::CMY(122, 16, 40);

    println!("What color is it?");

    match color {
        Color::Red => println!("This is red"),
        Color::Blue => println!("This is blue"),
        Color::Green => println!("This is green"),
        Color::RGB(r, g, b) => println!("This is RGB: r{}, g{}, b{}", r, g, b),
        Color::HSV(h, s, v) => println!("This is HSV: h{}, s{}, v{}", h, s, v),
        Color::HSL(h, s, l) => println!("This is HSL: h{}, s{}, l{}", h, s, l),
        Color::CMY(c, m, y) => println!("This is CMY: c{}, m{}, y{}", c, m, y),
        Color::CMYK(c, m, y, k) => println!("This is CMYK: c{}, m{}, y{}, k{}", c, m, y, k),
    }
}

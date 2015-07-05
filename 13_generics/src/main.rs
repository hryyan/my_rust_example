struct SingleGenerics<T>(T);

fn main() {
    let _char: SingleGenerics<char> = SingleGenerics('a');
    let _u8: SingleGenerics<u8> = SingleGenerics(1);

    //println!("{}, {}", _char, _u8);
}

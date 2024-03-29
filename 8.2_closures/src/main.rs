fn main() {
    fn function(i: i32) -> i32 { i + 1 };
    
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     | i + 1 ;

    let i = 1;

    println!("function: {}", function(i));
    println!("annotated closure: {}", closure_annotated(i));
    println!("inferred closure: {}", closure_inferred(i));

    let one = || 1;
    println!("closure return one: {}", one());

    let professor_x = "Professor";

    let print = || println!("Processor X's name is: {}", professor_x);
    print();
}

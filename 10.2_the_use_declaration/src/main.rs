use deeply::nested::function as another_function;

fn function() {
    println!("Call function");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("Call deeply::nested::function");
        }
    }
}

fn main() {
    function();
    another_function();
}

fn function() {
    println!("Call function");
}

mod my {
    #[allow(dead_code)]
    pub fn function() {
        println!("Call my::function");
    }

    mod nest {
        fn function() {
            println!("Call my::nest::function");
        }
    }

    pub mod another_nest {
        pub fn function() {
            println!("Call my::another_nest::function");
        }
    }
}

fn main() {
    function();

    my::function();
    //my::nest::function();
    my::another_nest::function();
}

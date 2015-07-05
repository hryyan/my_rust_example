fn function() {
    println!("Call outter function");
}

mod my {
    pub fn indirect_all() {
        {
            use cool::function as one_function;
            one_function();
        }

        {
            use self::cool::function as two_function;
            two_function();
        }

        {
            use super::function as three_function;
            three_function();
        }
    }

    mod cool {
        pub fn function() {
            println!("Call my::cool::function");
        }
    }
}

mod cool {
    pub fn function() {
        println!("Call cool:function");
    }
}

fn main() {
    my::indirect_all();
}

fn main() {
    let reference = &4;

    match reference {
        &val => println!("Got a value via desturturing: {:?}", val),
    }

    match reference {
        val => println!("Test, {}", val),
    }

    match *reference {
        val => println!("Got a value via dereference: {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 4;

    match _is_a_reference {
        &val => println!("Got a value {}", val),
    }

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match value {
        r => println!("Ex: {:?}", r),
    }

    match mut_value {
        mut r => {
            r += 1;    
            println!("Got a reference to a value: {:?}", r);
        }
    }
}

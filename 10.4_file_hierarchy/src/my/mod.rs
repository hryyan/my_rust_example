mod the_mod_in_same_directory;
pub mod the_mod_in_same_directory_but_can_be_accessed;

pub fn function() {
    println!("Call my::function");

    private_function();
}

fn private_function() {
    println!("Call my::private_function");
}

pub fn call_others() {
    the_mod_in_same_directory::function();
}

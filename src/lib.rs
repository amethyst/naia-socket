extern crate gaia_data_transport;

fn private_function() {
    println!("called gaia's `private_function()`");

    gaia_data_transport::indirect_access();
}

pub fn indirect_access() {
    print!("called gaia's `indirect_access()`, that\n> ");

    private_function();
}
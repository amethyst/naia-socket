pub fn public_function() {
    println!("called gaia's `public_function()`");
}

fn private_function() {
    println!("called gaia's `private_function()`");
}

pub fn indirect_access() {
    print!("called gaia's `indirect_access()`, that\n> ");

    private_function();
}
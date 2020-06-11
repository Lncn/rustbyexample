mod guessing_game;
mod hello_world;
mod primitives;

fn main() {
    /* Chapter 01: Hello World */
    println!("************************************");
    println!("***** Chapter 01: Hello World! *****");
    println!("************************************");
    hello_world::run();
    println!("  *****************************");
    println!("  *** Extra: Guessing Game! ***");
    println!("  *****************************");
    guessing_game::run();
    println!("************************************");
    println!("***** Chapter 02: Primitives! ******");
    println!("************************************");
    primitives::run();
}

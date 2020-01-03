mod hello_world;
mod guessing_game;

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
}

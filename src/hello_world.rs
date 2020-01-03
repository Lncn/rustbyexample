use std::fmt;

pub fn run() {
    /* I'm still not entirely sure what the exclamation syntax is here for Rust "macros",
     * but it's clear it's probably more complicated than normal C macros or something... */
    println!("Hello, Lincoln!");
    println!("I'm a Rustacean!\n");

    // Here's a trivial case for block comments:
    let x = 5 + /* 90 + */ 5;
    println!("Is 'x' 10 or 100? x = {}\n", x);

    // We have lots of options for formatting print statements...
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Bob", "Alice");
    println!(
        "{subject} {verb} {object}\n",
        subject = "Archie",
        verb = "drank",
        object = "milk"
    );
    // You can specify special formatting within the curlies with a ':'
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );
    println!("{number:>width$}", number = 1, width = 6);
    // Zero pad your right alignment:
    println!("{number:>0width$}\n", number = 1, width = 6);

    // You can specify float resolution using more '{:}' syntax
    let pi = 3.1415926535;
    println!("Pi is roughly {:.4}", pi);

    // To print out more than just primitives, you need a little more...
    #[derive(Debug)]
    struct Structure(i32);
    // Custom structures need to implement the Display trait, but Rust also provides a default
    // #[derive(Debug)] attribute which is generally sufficient and recommended for custom structs
    println!(
        "This struct, {:?}, wouldn't print without the '#[derive(Debug)]'\n",
        Structure(3)
    );

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    };
    let name = "Ruby";
    let age = 0;
    let ruby = Person { name, age };
    println!("You can pretty print structs with '{{:#?}}'\n{:#?}", ruby);
}

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

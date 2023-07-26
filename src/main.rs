//https://doc.rust-lang.org/rust-by-example/hello.html
fn hello_world(){
    println!("Hello, world!");
    println!("I'm a Rustacean!");
}

//https://doc.rust-lang.org/rust-by-example/hello/print.html
fn formatted_print() {
    format!("How does format work?");
    print!("{} is 2", 2);
    println!("{0} is {1} but also {1}", "multiple", "arguements");
    println!("{subject} {verb} {agreement}", subject="verb", verb="subject", agreement="agreement");
    eprint!("ERROR!");
    eprintln!("ERROR NEW LINE!");
}

fn formatted_print_activity(){
     // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);
    println!("{padding:>10}", padding="lmao");

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number=1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1); // 10000

    println!("{number:p>5}", number=5);

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    println!("{number:<>width$}", number=1, width=3);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
    
    /*
    Add a println! macro call that prints: Pi is roughly 3.142 by controlling the 
    number of decimal places shown. For the purposes of this exercise, use let pi = 3.141592 
    as an estimate for pi. (Hint: you may need to check the std::fmt documentation for setting the number of decimals to display)
    */
    let pi: f64 = std::f64::consts::PI;
    println!("pi is = {pi:.3}");

}

fn debug() {
    struct UnPrintable(i32);

    #[derive(Debug)]
    struct DebugPrintable(i32);

    // Derive the `fmt::Debug` implementation for `Structure`. `Structure`
    // is a structure which contains a single `i32`.
    #[derive(Debug)]
    struct Structure(i32);

    // Put a `Structure` inside of the structure `Deep`. Make it printable
    // also.
    #[derive(Debug)]
    struct Deep(Structure);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name: &str = "Peter";
    let age: u8 = 27;
    let peter: Person<'_>= Person { name, age};

    println!("{:#?}", peter);
}

fn debug_self_challenge() {
    //before moving onto 1.2.2 display
    //I wanna challenge myself with a simple problem.

    //Challenge: Get euler's number and print it out in 5 different ways.

    #[derive(Debug)]
    struct StoredEulersNumber(f64);
    
    let euluers_number = std::f64::consts::E;

    println!("First way: Printing Euluer's number from a struct: {:?}", StoredEulersNumber(euluers_number));
    println!("Second way: Printing Euluer's number as is: {}", euluers_number);
    let euluer_as_bits = euluers_number.to_bits();
    
    println!("Third way: Printing Euluers number in hexadecimal: {:X?}", euluer_as_bits);
    println!("Fourth way: Printing Euluers number by padding zeroes: {:0>15.5}", euluers_number);

    let num_bits: usize = (f64::MANTISSA_DIGITS + 1) as usize;
    let result_macro = format!("{:0>width$b}", euluer_as_bits, width = num_bits);
    println!("Fifth way: Printing Euluers number by padding 5 zeroes and turning it into binary {}", result_macro);
}

fn main() {
    hello_world();
    formatted_print();
    formatted_print_activity();
    debug();
    debug_self_challenge();
}


//https://doc.rust-lang.org/rust-by-example/hello/comment.html
//This is a comment

/* This is block comment */

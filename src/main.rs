//https://doc.rust-lang.org/rust-by-example/hello.html
fn hello_world() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");
}

//https://doc.rust-lang.org/rust-by-example/hello/print.html
fn formatted_print() {
    format!("How does format work?");
    print!("{} is 2", 2);
    println!("{0} is {1} but also {1}", "multiple", "arguements");
    println!(
        "{subject} {verb} {agreement}",
        subject = "verb",
        verb = "subject",
        agreement = "agreement"
    );
    eprint!("ERROR!");
    eprintln!("ERROR NEW LINE!");
}

fn formatted_print_activity() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number = 1);
    println!("{padding:>10}", padding = "lmao");

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number = 1); // 00001
                                          // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number = 1); // 10000

    println!("{number:p>5}", number = 5);

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number = 1, width = 5);

    println!("{number:<>width$}", number = 1, width = 3);

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
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name: &str = "Peter";
    let age: u8 = 27;
    let peter: Person<'_> = Person { name, age };

    println!("{:#?}", peter);
}

fn debug_self_challenge() {
    //before moving onto 1.2.2 display
    //I wanna challenge myself with a simple problem.

    //Challenge: Get euler's number and print it out in 5 different ways.

    #[derive(Debug)]
    struct StoredEulersNumber(f64);

    let euluers_number = std::f64::consts::E;

    println!(
        "First way: Printing Euluer's number from a struct: {:?}",
        StoredEulersNumber(euluers_number)
    );
    println!(
        "Second way: Printing Euluer's number as is: {}",
        euluers_number
    );
    let euluer_as_bits = euluers_number.to_bits();

    println!(
        "Third way: Printing Euluers number in hexadecimal: {:X?}",
        euluer_as_bits
    );
    println!(
        "Fourth way: Printing Euluers number by padding zeroes: {:0>15.5}",
        euluers_number
    );

    let num_bits: usize = (f64::MANTISSA_DIGITS + 1) as usize;
    let result_macro = format!("{:0>width$b}", euluer_as_bits, width = num_bits);
    println!(
        "Fifth way: Printing Euluers number by padding 5 zeroes and turning it into binary {}",
        result_macro
    );
}

use std::{fmt::{self, Formatter, Display}, mem};

fn display() {
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    let structure = Structure(2);
    println!("Display: {}", structure);

    //fmt::Display is not implemented for Vec<T> or for any other generic containers. fmt::Debug must then be used for these generic cases.
    //This is not a problem though because for any new container type which is not generic, fmt::Display can be implemented.
}

fn display_self_challenge() {
    #[derive(Debug)]
    struct Translation(f64, f64, f64);

    impl fmt::Display for Translation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{},{},{}", self.0, self.1, self.2)
        }
    }

    let translation = Translation(1.1, 2.0, 4.0);
    println!("Translation {}", translation);
    println!("Translation Debug: {:?}", translation);
}

fn display_complex_example() {
    #[derive(Debug)]
    struct MinMax(i64, i64);

    // Implement `Display` for `MinMax`.
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Use `self.number` to refer to each positional data point.
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    // Define a structure where the fields are nameable for comparison.
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    // Similarly, implement `Display` for `Point2D`.
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point: Point2D = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}

fn display_activity() {
    /*
        After checking the output of the above example, use the Point2D struct as a guide to add a Complex struct to the example. When printed in the same way, the output should be:
        Display: 3.3 + 7.2i
        Debug: Complex { real: 3.3, imag: 7.2 }
    */

    #[derive(Debug)]
    struct Point2D{
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} + {}", self.x, self.y)
        }
    }

    let point = Point2D { x: 20.0, y: 20.0};
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}

fn testcase_list(){
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate(){
                if count != 0 {
                    write! (f, ", ")?;
                }
                write!(f, "{}: {}", count,v)?;
            }

            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

fn formatting(){
    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl Display for City {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
            
            write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    for city in [
        City { name: "Dublin", lat: 54.23123, lon: 23.21235},
        City { name: "Dublin", lat: 09.985123, lon: 72.123123},
        City { name: "Dublin", lat: 24.28456, lon: 86.432332},
    ] {
        println!("{}", city);
    }
}

fn formatting_activity(){
    //Add an implementation of the fmt::Display trait for the Color struct above so that the output displays as:
    struct Color { 
        red: u8,
        green: u8,
        blue: u8,
    }

    impl Display for Color {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "RGB ({}, {}, {}) 0x{:X}{:X}{:X}", self.red, self.green, self.blue, self.red, self.green, self.blue)
        }
        
    }

    for color in [
        Color { red: 255, green: 255, blue: 255},
    ] {
        println!("{}", color);
    }

}

fn primitives() {
    //i8, i16, i32, i64, i128, and isize
    //u8, u16, u32, u64, u128, and usize
    //f32, f64
    //char unicode scalar 'a', 'α', '∞'
    //bool true and false
    //(), empty tuple

    //Integers default to i32 and floats to f64. Note that Rust can also infer types from context.

    let logical = true;

    let a_float = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0;
    let default_int = 7;

    let mut interred_type= 12;
    interred_type = 5323424;

    let mut mutable = 12;
    mutable =21;

    // mutable = true;

    let mutable = true;

}

fn literals_and_operators() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3f64);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
    
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x08 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as: {}", 1_000_000u32);
}

fn tuples(){
    // This is a tuple
    // Tuples can be used as function arguments and as return values.
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        // `let` can be used to bind the members of a tuple to variables.
        let (int_param, bool_param) = pair;

        (bool_param, int_param)
    }

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);
    
    let tuples_in_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("Tuple in tuple first-first value: {}", tuples_in_tuples.0.0);

    println!("tuple of tuples: {:?}", tuples_in_tuples);
    
    let pair = (1, true);

    println!("Pair is {:?}", pair);

    println!("Reverse pair: {:?}", reverse(pair));

    //One element tuple needs a comma
    println!("One Element tuple: {:?}", (5u32,));
    println!("This is just an i32 {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?},{:?},{:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

}


fn tuples_activity(){
    struct Matrix(f32, f32, f32, f32);

    fn transpose(matrix: Matrix) -> Matrix{
        let new_matrix = matrix;

        Matrix(new_matrix.0, new_matrix.2, new_matrix.1, new_matrix.3)
    }

    impl Display for Matrix{
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            return write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3);
        }
    }

    let matrix = Matrix(2.0, 5.0, 8.0, 9.0);
    println!("{}", matrix);
    
    println!("{}", transpose(matrix));
}


// This function borrows a slice.
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn arrays(){
    // fixed array size with values
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    
    // All elemetns are initialized in 0
    let ys: [i32; 500] = [0; 500];

    println!("First element of xs: {}", xs[0]);
    println!("last element of ys: {}", ys[499]);

    println!("length of ys: {}", ys.len());

    println!("array occupies {} bytes", mem::size_of_val(&ys));

    //Borrowing a silice.
    analyze_slice(&xs);

     // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}

#[derive(Debug)]
struct SomeStruct(i32);

fn main() {
    hello_world();
    formatted_print();
    formatted_print_activity();
    debug();
    debug_self_challenge();
    display();
    display_self_challenge();
    display_complex_example();
    display_activity();
    testcase_list();
    formatting();
    formatting_activity();
    primitives();
    literals_and_operators();
    tuples();
    tuples_activity();
    arrays();
}
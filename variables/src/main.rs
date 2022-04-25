fn main() {
    /*
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6; // Compile-time issue, x isn't mutable so it can't be written to.
    println!("The value of x is: {}", x);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // Since x is now mutable, this won't lead to compile-time issue
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // How constants are declared
                                                    // Can take in SOME expressions but not all
                                                    // Is always immutable, must declare type

    let x = 5;
    let x = x + 1; // Declaring same var is ALLOWED
                   // This is called SHADOWING
                   // Creates new variable on top of old one.
                   // Allows you to change immutables BUT ALSO, can change type of variable
                   // Essentially, makes new variable on top of old one.
    {
        let x = x + 2;
        println!("The value of x is: {}", x);
    }

    println!("The value of x is: {}", x);
    */

    // Rust has two types of values, Scalar and Compound
        // 4 types of Scalar:
            // Integer sizes:
                // 8 bit -> Signed i8 Unsigned u8
                // 16 bit -> Signed i16 Unsigned u16
                // 32 bit -> Signed i32 Unsigned u32
                // 64 bit -> Signed i64 Unsigned u64
                // architecture of computer -> Signed isize Unsigned usize
                // Architecture of computer is based off of if pc is 32 bit or 64 bit
                // Integer literals can be written in:
                    // Decimal 10_000
                    // Hex 0xff
                    // Octal 0o77
                    // Binary 0b1111_0000
                    // Byte (u8 only) b'A'
            // Floating-point numbers:
                // Supports 32 bit and 64 bit
                // f32 and f64 respectively
                // Same speed in modern arch but 64 more accurate
            // Bools:
                // One byte in size
                // Either T or F
            // Character:
                // Indicated by single-quotes ('')
                // Can store any UTF-8 character (emojis!)
        // 2 primitive Compound types:
            // Tuple
                // Fixed length, cannot grow or shrink in size once declared.
                // Example declaration:
                    // let tup: (i32, f64, u8) = (500, 6.4, 1);
                // Can use pattern matching to destructure a tuple value:
                    // let (x, y, z) = tup;
                // Process of breaking tuple into parts is called DESTRUCTURING
                // Can also access desired value as follows:
                    // let five_hundred = tup.0;
                    // let six_point_four = tup.1;
                    // let one = tup.2;
                // Empty tuple is a special type with only one value ()
                    // Called the unit type, the value called the unit value
                    // Expressions implicitly return the unit value if they don't return any other value
            // Array
                // All elements of an array must have the same type.
                // Arrays have a fixed length and cannot be shrunk or grown
                // Write arrays as follows:
                    // let a = [1, 2, 3, 4, 5];
                // STORED IN STACK, NOT HEAP
                // Ensures you always have a fixed number of elements
                // To write an array's type, do as follows:
                    // let a: [i32; 5] = [1, 2, 3, 4, 5];
                // Can also have an array set to same value for each element as follows:
                    // let a = [3;5]
                    // This is the same as let a = [3, 3, 3, 3, 3];
                // Takes single chunk of memory of known, fixed size, can be allocated on stack
                // To access, use
                    //array[0]
                // Rust checks accessing out of bounds in Runtime
                // When you access out of bounds, program will panic and shut down.
                // Safety and reliability feature
}

//Small section, didnt finish Common programming Concepts
// So far, this section is pretty standard.
// Cool to know about tuples, hope to learn more about stack vs heap.
// Kinda know the difference, but would love being more active about thinking about it.
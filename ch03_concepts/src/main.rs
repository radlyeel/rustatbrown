fn main() {
    // In this file, there will be pairs of slightly differnt lines;
    // uncomment one or the other to see the effect
    // 3.1 Variables and Mutability
    // By default, "variables" are immutable
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    //  -- Always immutable
    //  -- Possible in global scope
    //  -- Require type annotation
    //  -- Value must be knowable at compile time
    //  -- Conventionally, names are UPPER_SNAKE_CASE
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // This line prevents unused warning
    println!("Seconds in three hours: {THREE_HOURS_IN_SECONDS}");


    // Shadowing
    let x = 5;

    // This 'x' is actually a different 'x'; the first is now unavailable
    let x = x + 1;

    {
        // This shadows the outer block's 'x'
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    // The inner block's 'x' is no longer in scope
    println!("The value of x is: {x}");

    // 3.2 Data Types
    let num_decimal : u32 = 98_222;
    let num_hex : u8 = 0xff;
    let num_oct : u8 = 0o77;
    let num_bin : u8 = 0b1111_0000;
    let num_byte : u8 = b'A';
    println!("Numbers: {num_decimal}, {num_hex}, {num_oct}, {num_bin}, {num_byte}");

    // Floats
    let x = 2.0;      // f64 by default
    let y: f32 = 3.0; // f32
    println!("{x}, {y}");

    // Numeric operations are not novel. Questions? See
    // https://rust-book.cs.brown.edu/ch03-02-data-types.html#numeric-operations

    // Booleans
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("{t}, {f}");

    // Char
    let z : char = 'z';

    // Emojis just work
    let heart_eyed_cat = '😻';
    println! ("{z}  {heart_eyed_cat}");

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("1. tup = {a}, {b}, {c}");
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    println!("2. tup = {a}, {b}, {c}");
    // Cannot use . notation in println!()
    // println!("tup = {tup.0}, {b}, {c}");


    // Arrays
    let a = [1, 2, 3, 4, 5];
    // special type annotation for array size
    //let a: [i32; 5] = [1, 2, 3, 4, 5];
    let five_threes = [3;5];
    // println! chokes on arrays
    // println!("{a}, {five_threes}");
    // println!("{a[0]}, {five_threes[4]}");
    let first_a = a[0];
    let last_3 = five_threes[4];
    println!("{first_a}, {last_3}");

    //  Panic on invalid indexing
    let index : usize = 4;
    let a_i = a[index];
    println!("a[{index}] = {a_i}");

    // 3.3 Functions can be defined almost anywhere, including after they're
    // called (see below, print_labeled_measurement
    fn function() {
        println!("Inside function()");
    }

    println!("Calling fuction()");
    function();

    fn function2(x:i32) {
        println!("Inside function({x})");
    }

    println!("Calling fuction2(5)");
    function2(5);


    print_labeled_measurement(5, 'h');


    // Multiple parameters
    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value}{unit_label}");
    }

    // Returning a value from a function
    fn five() -> i32 {
        5
    }

    // Aside: cannot call a function from println! placeholder
    // println!("Five = {five()}");
    // Ignore warnings about snake case names
    let Five = five();
    println!("Five = {Five}");

    // In and out
    fn plus_one(x: i32) -> i32 {
        x + 1
    }


    let Six = plus_one(five());
    println!("Six = {Six}");

    // 3.5. Control Flow
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let-if:
    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Loops
    // Unconditional loop
    // Can return a value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
        
    // Labels for breaking from outer loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // Conditional Loops with while
    let a = [10, 20, 30, 40, 50];
    let mut sum : i32 = 0;
    let mut index = 0;

    while index < 5 {
        sum += a[index];
        index += 1;
    }
    assert!(sum == 150, "problem with first summation");
    println!("All good");

    // iterator
    sum = 0;
    for a_i in a {
        sum += a_i;
    }
    assert!(sum == 150, "problem with second summation");
    println!("All good");

    // Reverse indexing
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let mut x = 0;
    'a: loop {
        x += 1;
        println!("x = {x}");
        'b: loop {
            if x > 10 {
                continue 'a;
            } else {
                break 'b;
            }      
        }
        break;       
    }
}

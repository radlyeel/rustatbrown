fn main() {

    fn read(y: bool) {
        if y {
            println!("y is true!");
        }
    }

    // The call tp read() should not happen until x exists
    let x = true;
    read(x); // oh no! x isn't defined!
    //let x = true;

    // Boxes live on the heap
    // let a = Box::new([0; 1_000_000]);
    // a is a pointer to the heap data
    // Rust does not allow me to 'free' heap memory
    //   instead, boxes are freed when their OWNER goes out of scope,  
    //   and there maye be EXACTLY ONE owner.
    // let b = a;   This MOVES ownership to b and a is no longer valid
    // Cloning avoids moves:
    // let b = a.clone()
    // But it entails copying the underlying data, so use it sparingly
    // 

    // 4.2 References and Borrowing
    // Variables have _ prefix to stifle 'unused' warning
    let mut x: Box<i32> = Box::new(1);
    let _a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value,
                             //     so x points to the value 2

    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let _b: i32 = **r1;       // two dereferences get us to the heap value

    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let _c: i32 = *r2;        // so only one dereference is needed to read it

    // Rust Avoids Simultaneous Aliasing and Mutation

    // Permissions (only at compile time, used by Borrow Checker
    // Read (R): data can be copied to another location.
    // Write (W): data can be mutated in-place.
    // Own (O): data can be moved or dropped.
    // 
    // let x = 7;        x has RO
    // let mut y = 42;   y has ROW
    // TODO: Studty https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html#references-change-permissions-on-paths
    //
}

fn main() {
    // Variables

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    // Types

    // Integers
    let _int_a: u32 = "42".parse().expect("Not a number");
    let _int_b: u32 = 42;
    let _int_c: i32 = -42;
    // Hexidecimal
    let _int_d = 0xFFFFFF;
    // Binary
    let _int_e = 0b1001;

    // Floating points
    // 64-bit double precision floating point: Aka Doubles
    let _f_x = 2.0;
    // Single precision
    let _f_y: f32 = 2.0;

    // Basic arithmetic

    // Addition
    let _sum = 5 + 5;

    // Subtraction
    let _subtraction = 5 - 5;

    // Multiplication
    let _multiply = 5 * 5;

    // Division
    let _divide = 5 / 5;

    // Remainder
    let _remainder = 5 % 5;

    // Booleans
    let _is_cool: bool = true;

    // Characters
    let _emoji = '🐢';

    // Arrays
    let _arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Functions
    fn greeting() {
        println!("Wazzap!");
    };

    greeting();

    // Closures
    let str = "Wazzap closure!";
    let cool_closure = || {
        // Can access variables outside of block scope
        println!("{}", str);
    };
    let cool_small_closure = || println!("{} You're so tiny!", str);
    cool_closure();
    cool_small_closure();
}

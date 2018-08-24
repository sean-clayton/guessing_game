fn main() {
    // Variables

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    // Types

    // Integers
    let int_a: u32 = "42".parse().expect("Not a number");
    let int_b: u32 = 42;
    let int_c: i32 = -42;
    // Hexidecimal
    let int_d = 0xFFFFFF;
    // Binary
    let int_e = 0b1001;

    // Floating points
    // 64-bit double precision floating point: Aka Doubles
    let f_x = 2.0;
    // Single precision
    let f_y: f32 = 2.0;

    // Basic arithmetic

    // Addition
    let sum = 5 + 5;

    // Subtraction
    let subtraction = 5 - 5;

    // Multiplication
    let multiply = 5 * 5;

    // Division
    let divide = 5 / 5;

    // Remainder
    let remainder = 5 % 5;

    // Booleans
    let is_cool: bool = true;

    // Characters
    let emoji = '🐢';
}

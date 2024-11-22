fn main() {
    // Scalar Types: integers, floating-point numbers, Booleans, and characters.

    // Integers
    let i1: i8 = -8;
    let ui1: u8 = 8;
    let i2: i16 = -16;
    let ui2: u8 = 16;
    let i3 = -32; // the default type of an integer is i32
    let ui3: u32 = 32;
    let i4: i64 = -64;
    let ui4: u64 = 64;
    let i5: i128 = -128;
    let ui5: i128 = 128;
    let i6: isize = -999;
    let ui6: usize = 999;
    println!("signed integers: {i1}, {i2}, {i3}, {i4}, {i5}, {i6}");
    println!("unsigned integers: {ui1}, {ui2}, {ui3}, {ui4}, {ui5}, {ui6}");
    // Integers literals
    let il1 = 98_222; // Decimal
    let il2 = 0xff; // Hex
    let il3 = 0o77; // Octal
    let il4 = 0b1111_0000_0; // Binary
    let il5 = b'a'; // Byte(u8 only)
    println!("Integers literals: {il1}, {il2}, {il3}, {il4}, {il5}");

    // floating-point
    let f1 = 2.0; // f64
    let f2: f32 = 2.0; // f32
    println!("floating-point: {f1}, {f2}");

    // booleans
    let b1 = false;
    let b2 = true;
    println!("booleans: {b1}, {b2}");

    // characters: occupy 4 bytes
    let c1 = 'a';
    let c2 = 'B';
    let c3 = '😻';
    println!("characters: {c1}, {c2}, {c3}");

    // ===================================================================

    // Compound Types: tuples and arrays.

    // Tuple
    let tup = (500, true, "hello");
    let (x, y, z) = tup;
    println!("x: {x}, y: {y}, z:{z}");
    // arrays
    let arr1 = [1, 2, 3, 4, 5];
    let weeks = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    println!("test array: {arr1:?}, weeks: {weeks:?}");
}

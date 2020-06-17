use std::mem;

#[allow(dead_code)]
pub fn two_point_zero(){
    let logical: bool = true;
    println!("{}", logical);

    let a_float: f64 = 1.0;
    let an_integer = 5i32;
    println!("{}", a_float);
    println!("{}", an_integer);

    let default_float = 3.0; // f64
    let default_integer = 7; // i32
    println!("{}", default_float);
    println!("{}", default_integer);

    let mut inferred_type = 12; // Type i64 is inferred from another line
    println!("{}", inferred_type);
    inferred_type = 4294967296i64;
    println!("{}", inferred_type);

    let mut mutable = 12; // Mutable i32
    println!("{}", mutable);
    mutable = 21;
    println!("{}", mutable);

    // Error because the type of a variable cannot be changed
    // mutable = true;
    let mutable = true;
    println!("{}", mutable);
}

#[allow(dead_code)]
pub fn two_point_one(){
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    return (boolean, integer);
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

#[allow(dead_code)]
pub fn two_point_two(){
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tupes: {:?}", tuple_of_tuples);

    // Too long tuples can't be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    println!("One element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;

    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}

fn analyze_slice(slice: &[i32]) {
    println!("first name element :{}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

#[allow(dead_code)]
pub fn two_point_three(){
    let xs: [i32; 5] = [1,2,3,4,5];

    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("array size: {}", xs.len());
    println!("array occupies {} bytes",)
}

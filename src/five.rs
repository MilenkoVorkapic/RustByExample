#![allow(dead_code)]
#![allow(overflowing_literals)]

// 5.1
pub fn five_point_one(){
    let decimal = 65.4321_f32;

    // Nope, not the right type
    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    // Nope, cannot cast this type
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is : {}", 1000 as u16);

    println!("1000 as a u8 is : {}", 1000 as u8);
    println!("-1 as a u8 is : {}", (-1i8) as u8);

    println!("1000 mod 256 is : {}", 1000 % 256);

    println!("128 as a i16 is: {}", 128 as i16);
    println!("128 as a i8 is: {}", 128 as i8);

    println!("1000 as a u8 is : {}", 1000 as u8);
    println!("232 as a i8 is : {}", 232 as i8);
}

// 5.2
pub fn five_point_two(){
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

// 5.3
pub fn five_point_three(){
    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec)
}

// 5.4
type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

pub fn five_point_four(){
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches)
}
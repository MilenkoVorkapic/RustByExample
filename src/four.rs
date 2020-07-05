#![allow(dead_code)]

// 4.0
pub fn four_point_zero(){
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();
   
    let copied_integer = an_integer;
    println!("A copied integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;
    let _noisy_unused_variable = 2u32;
}

// 4.1
pub fn four_point_one(){
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Nope, not mutable
    // _immutable_binding += 1;
}

// 4.2
pub fn four_point_two(){
    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }
    // Nope, not in this scope
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}

// 4.3
pub fn four_point_three(){
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Nope, not initialized
    // println!("another binding: {}", another_binding);

    another_binding = 1;
    println!("another binding: {}", another_binding);
}

// 4.4
pub fn four_point_four(){
    let mut _mutable_integer = 7i32;
    {
        let _mutable_integer = _mutable_integer;
        
        // Nope, here it's "frozen"
        // _mutable_integer = 50;
    }

    // Here the variable can be modified, because the local one above is now out of scope
    _mutable_integer = 3;
    println!("mutable integer: {}", _mutable_integer);
}
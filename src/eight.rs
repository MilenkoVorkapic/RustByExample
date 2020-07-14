#![allow(dead_code)]
#![allow(unreachable_code)]

pub fn eight_point_one(){
 let n = 5;
 if n < 0 {
     print!("{} is negative", n);
 } else if n > 0 {
     print!("{} is positive", n);
 } else {
     print!("{} is zero", n);
 }

 let big_n = 
    if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        n/2
    };
    println!("{} -> {}", n, big_n)
}

pub fn eight_two_zero(){
    let mut count = 0u32;
    
    println!("Let's count until infinity");

    loop {
        count +=1;

        if count == 16 {
            break;
        }

        if count % 3 == 0 && count % 5 == 0 {
            println!("fizzbuzz");
            continue;
        }

        if count % 3 == 0 {
            println!("three");
            continue;
        }

        if count % 5 == 0 {
            println!("five");
            continue;
        }
    }
}

pub fn eight_two_one(){
 'outer: loop {
     println!("Entered the outer loop");

    loop {
         println!("Entered the inner loop");

         // this would break the inner loop
         // break

         break 'outer;
     }

     println!("This point will never be reached");
 }

 println!("Exited the outer loop");
}

pub fn eight_two_two(){
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20)
}

pub fn eight_three(){
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
}

pub fn eight_four(){
    // or written: 
    // for n in 1..=100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello {}", name)
        }
    }

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello {}", name)
        }
    }

    let mut names2 = vec!["Bob", "Frank", "Ferris"];
    for name in names2.iter_mut() {
        *name = match name {
            &mut "Ferris" => "THere is a rustacean among us",
            _ => "Hello",
        }
    }
}

pub fn eight_five(){
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary)
}

pub fn eight_five_one_one(){
    let pair = (0, -2);
    println!("Tell me about {:?}", pair);

    match pair {
        (0, y) => println!("First is `0` and y is `{:?}`", y),
        (x, 0) => println!("`x` is {:?} and last is `0`", x),
        _ => println!("It doesn't matter what they are")
    }
}


enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

pub fn eight_five_one_two(){
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");
    match color {
        Color::Red => println!("The color is red"),
        Color::Blue => println!("The color is blue"),
        Color::Green => println!("The color is green"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, blue: {}", r,g,b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
    }
}

pub fn eight_five_one_three(){
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We aded 10. `mut_value`: {:?}", m);
        }
    }
}

pub fn eight_five_one_four(){
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo {x: (1, 2), y: 3};

    match foo {
        Foo {x: (1, b), y} => println!("First of x is 1, b = {}, y = {}", b, y),
        Foo {y:2, x: i} => println!("y is 2, i = {:?}", i),
        Foo {y, .. } => println!("y = {}, we don't care about x", y)
    }
}

pub fn eight_five_two(){
    let pair = (2, -2);
    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation...")
    }
}

pub fn eight_five_three(){
    fn age() -> u32 {
        15
    }

    println!("Telle me what type of person you are");

    match age(){
        0 => println!("I'm not born yet I guess"),
        n @ 1 ..=12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n)
    }

    fn some_number() -> Option<u32> {
        Some(42)
    }

    match some_number(){
        Some(n @ 42) => println!("The answer: {}", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => ()
    }
}

enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

enum Test{
    Bar
}

pub fn eight_six(){
    let optional  = Some(7);
    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i)
        },
        _ => {},
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}", i)
    }

    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number. Let's go with a letter");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters, Let's go with an emoticon :)!");
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(_value @ 100) = c {
        println!("c is one hundred")
    }

   
    
}

pub fn eight_seven(){
    let mut optional = Some(0);

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit");
                    optional = None;
                } else {
                    println!("`i` is {:?}. Try again.", i);
                    optional = Some(i + 1);
                }
            },
            _ => {break}
        }
    }

    let mut optional2 = Some(0);

    while let Some(i) = optional2 {
        if i > 9 {
            println!("Greater than 9, quit");
            optional2 = None;
        } else {
            println!("`i` is {:?}. Try again.", i);
            optional2 = Some(i + 1);
        }
    }
}
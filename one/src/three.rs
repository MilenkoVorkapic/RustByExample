#![allow(dead_code)]
use crate::three::List::*;

// 3.1
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32, 
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

#[allow(dead_code)]
pub fn three_point_one(){
    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};
    println!("{:?}", peter);

    let point: Point = Point {x: 10.3, y: 0.4};
    println!("Point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point {x: 5.2, ..point};
    println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {x: top_edge, y: left_edge} = point;
    let _rectangle = Rectangle {
        top_left: Point {x: left_edge, y: top_edge},
        bottom_right: bottom_right
    };

    let _unit = Unit;
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}


// 3.2

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x: i64, y: i64},
}

fn inspect(event: WebEvent){
    match event {
        WebEvent::PageLoad => println!("page loaded!"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("Pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click {x, y} => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

pub fn three_point_two(){
    // 3.2
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click {x: 20, y:80};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let add = Operations::Add;
    let sub = Operations::Subtract;
    println!("{}", add.run(5, 4));
    println!("{}", sub.run(5, 4));

    // 3.2.1
    three_two_one();
    three_two_two();
    three_two_three();
}

// 3.2.1
enum Status{
    Rich, 
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn three_two_one(){
    use crate::three::Status::{Poor, Rich};
    use crate::three::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money..."),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}

// 3.2.2

enum Number {
    Zero,
    One,
    Two
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn three_two_two(){
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

// 3.2.3
enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
       return Nil;
    }

    fn prepend(self, elem: u32) -> List {
       return Cons(elem, Box::new(self));
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                return format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                return format!("Nil")
            }
        }
    }
}

fn three_two_three(){
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked has length: {}", list.len());
    println!("{}", list.stringify());
}

// 3.3
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    return n > THRESHOLD;
}

pub fn three_point_three(){
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big"} else { "small"})

    // Cannot modify a const, this crashed
    // THRESHOLD = 5;
}
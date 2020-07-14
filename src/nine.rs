#![allow(dead_code)]
// 9.0
pub fn nine_zero(){
    fizzbuzz_to(100);
}

fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz")
    } else if is_divisible_by(n, 3) {
        println!("fizz")
    } else if is_divisible_by(n, 5) {
        println!("buzz")
    } else {
        println!("{}", n)
    }
}

fn is_divisible_by(number: u32, divider: u32) -> bool {
    if divider == 0 {
        return false;
    }
    number % divider == 0
}

// 9.1
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point {x: 0.0, y: 0.0}
    }

    fn new(x: f64, y: f64) -> Point {
        Point {x, y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;

        1.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }
    fn translate(&mut self, x: f64, y: f64){
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self){
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}

pub fn nine_one(){
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", Rectangle::perimeter(&rectangle));
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle{
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();
}

//9.2
pub fn nine_two(){
    fn function(n: i32) -> i32 {n + 1};

    let i = 1;

    
    let closure_annotated = || -> i32 {
        i + 1
    };

    let closure_inferred = |n| {
        println!("{}", i);
        n + 1
    };


    println!("function: {}", function(i));
    println!("annotated: {}", closure_annotated());
    println!("inferred: {}", closure_inferred(i));

    let one  = |n| n;
    println!("closure returnin one: {}", one(i));
}

//9.2.1
pub fn nine_two_one(){
    use std::mem;

    let color = "green";

    let print = || println!("`color`: {}", color);
    print();

    let _reborrow = &color;
    print();
    let _color_moved = color;

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    // let _reborrow = &count;

    inc();

    let _count_reborrowed = &count;
    // inc();

    let movable = Box::new(3);

    let consume = || {
        println!("movables: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    // consume();
    
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("{}", haystack.len());
}

// 9.2.2

fn apply<F>(f: F) where F: FnOnce(){f()}

fn apply_to_3<F>(f: F) -> i32 where F:Fn(i32) -> i32 {
    f(3)
}

pub fn nine_two_two(){
    use std::mem;

    let greeting = "hello";

    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}", greeting);

        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);
        println!("Now I can sleep. zzz");

        mem::drop(farewell);
    };

    apply(diary);
    
    let double = |x| 2 * x;

    println!("3 doubles: {}", apply_to_3(double))
}

// 9.2.3
fn applysecond<F>(f:F) where
    F: Fn(){
        f();
    }

pub fn nine_two_three(){
    let x = 7;

    let print = || println!("{}", x);

    applysecond(print);
}

// 9.2.4
fn call_me<F: Fn()>(f: F) {
    f();
}

fn function(){
    println!("I'm a function!");
}

pub fn nine_two_four(){
    let closure = || println!("I'm a closure");

    call_me(closure);
    call_me(function)
}

// 9.2.5
fn create_fn() -> impl Fn(){
    let text = "Fn".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut(){
    let text = "FnMut".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce(){
    let text = "FnOnce".to_owned();
    move || println!("This is a: {}", text)
}

pub fn nine_two_five(){
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}

// 9.2.6.1
pub trait Iterator {
    type Item;

    fn any<F>(&mut self, f: F) -> bool where
        F: FnMut(Self::Item);

    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        P: FnMut(Self::Item);
}

pub fn nine_two_six_one(){
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));
    
    let array1 = [1, 2, 3];
    // let array2 = [4, 5, 6];

    println!("2 in array1: {}", array1.iter().any(|&x| x ==2));
    // println!("2 in array2: {}", array2.into_iter().any(|&x| x ==2));
}

// 9.2.6.2 

pub fn nine_two_six_two(){
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`
    println!("Find 2 in array2: {:?}", array2.iter().find(|&&x| x == 2));

    let vec = vec![1, 9, 3, 3, 13, 2];
    let index_of_first_even_number = vec.iter().position(|x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    let index_of_first_neg_number = vec.iter().position(|x| x < &0);
    assert_eq!(index_of_first_neg_number, None);
}

// 9.3
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}
pub fn nine_three(){
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared > upper {
            break;
        }
        if is_odd(n_squared) {
            acc += n_squared
        }
    }

    println!("Imperative style: {}", acc);

    let sum_of_squared_odd_numbers = 
        (0..)
          .map(|n| n * n)
          .take_while(|&n_squared| n_squared < upper)
          .filter(|&n_squared| is_odd(n_squared))
          .fold(0, |acc, n_squared| acc + n_squared);

    println!("functionnal style: {}", sum_of_squared_odd_numbers)
}

// 9.4
fn foo() -> ! {
    panic!("Thix call never returns");
}

fn some_fn(){
    ()
}

pub fn nine_four(){
    /*
    let a: () = some_fn();
    println!("this function returns and you can see this line");

    let x: ! = panic!("This call never returns");
    println!("You will never see this line");
    */

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i%2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }

    println!("The sum of odd numbers up to 9 (exl): {}", sum_odd_numbers(9));
}
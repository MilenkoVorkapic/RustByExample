#![allow(dead_code)]

use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

// 5.1
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        return Number {value: item};
    }
}

pub fn six_point_one(){
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("converted string is : {}", my_string);

    let num = Number::from(30);
    println!("My number is :{:?}", num);

    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num)
}

// 5.2
#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            return Ok(EvenNumber(value));
        } else {
            return Err(());
        }
    }
}

pub fn six_point_two(){
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

// 5.3
struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Circle of radius {}", self.radius);
    }
}

pub fn six_point_three(){
    let circle = Circle {radius: 6};
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
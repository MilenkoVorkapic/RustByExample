#![allow(dead_code)]
struct A;

struct Single(A);

struct SingleGen<T>(T);

pub fn zero(){
    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
}

// 1
fn reg_fn(_s: Single){}

fn gen_spec_t(_s: SingleGen<A>){}

fn gen_spec_i32(_s: SingleGen<i32>){}

fn generic<T>(_s: SingleGen<T>){}

pub fn one(){
    reg_fn(Single(A));
    gen_spec_t(SingleGen(A));
    gen_spec_i32(SingleGen(6));

    generic::<char>(SingleGen('a'));
    generic(SingleGen('c'));
}

// 2

struct S;
struct GenericVal<T>(T);

impl GenericVal<f32>{}
impl GenericVal<S>{}

impl<T> GenericVal<T>{}

struct Val {
    val: f64
}

struct GenVal<T> {
    gen_val: T
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

pub fn two(){
    let x = Val {val: 3.0};
    let y = GenVal {gen_val : 3i32};

    println!("{}, {}", x.value(), y.value());
}

// 3
struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T){}
}

pub fn three(){
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
    // empty;
    // null;
}

// 4

/* fn printer<T: Display>(t: T) {
    println!("{}", t);
} */

// struct S<T: Display>(T);

// let s = S(vec![1]);

use std::fmt::{Debug, Display};

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {self.length * self.height}
}

#[derive(Debug)]
struct Rectangle {length: f64, height: f64}
struct Triangle {length: f64, height: f64}

fn print_debug<T: Debug>(t: &T){
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {t.area()}

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str {"red"}
fn blue<T: Blue>(_: &T) -> &'static str {"blue"}

pub fn four(){
    let rectangle = Rectangle {length: 3.0, height: 4.0};
    let _triangle = Triangle {length: 3.0, height: 4.0};

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // println!("A turkey is {}", red(&_turkey));
}

// 5
fn compare_prints<T: Debug + Display>(t: &T){
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U){
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}

pub fn five(){
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);

    compare_types(&array, &vec);
}

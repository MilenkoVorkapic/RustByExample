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
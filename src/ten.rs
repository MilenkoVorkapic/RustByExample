#![allow(dead_code)]

mod my_mod {
    fn private_function(){
        println!("called `my_mod::private_function()`");
    }

    pub fn function(){
        println!("`called my_mod::function()`");
    }

    pub fn indirect_access(){
        print!("called `my_mod::indirect_access()`, that/n> ");
        private_function();
    }

    pub mod nested {
        pub fn function(){
            println!("called `my_mod::nested::function()`");
        }
        fn private_function(){
            println!("called `my_mod::nested::private_function()`");
        }
        pub(in crate::ten::my_mod) fn public_function_in_my_mod(){
            println!("called `my_mod::nested::public_function_in_my_mod()`");
        }
        pub(self) fn public_function_in_nested(){
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        pub(super) fn public_function_in_super_mod(){
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod(){
        println!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    pub(crate) fn public_function_in_crate(){
        println!("my_mod::public_function_in_crate()");
    }

    mod private_nested{
        pub fn function(){
            println!("called `my_mod::private_nested::function()`")
        }

        pub(crate) fn restricted_function(){
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function(){
    println!("called `function()`");
}

pub fn ten_one(){
    function();
    my_mod::function();

    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    my_mod::public_function_in_crate();
}

// 10.2
mod my{
    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox{
                contents
            }
        }

        pub fn get_content(self) -> T {
            self.contents
        }
    }
}

pub fn ten_two(){
    let open_box = my::OpenBox {contents: "public information"};
    println!("The open box contains: {}", open_box.contents);

    // let closed_box = my::ClosedBox {contents: "Classified informations"};
    let closed_box = my::ClosedBox::new("Classified informations");
    println!("closed box contents: {}", closed_box.get_content())
}





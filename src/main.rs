#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_imports)]
#[allow(dead_code)]

use std::io;

// fn main() {
//     let mut input = String::new();
//     println!("Say Something");
//     match io::stdin().read_line(&mut input){
//         Ok(_)=>{
//             print!("You Said: {}", input);
//         }
//         Err(e) =>{
//             print!("Something went wrong: {}", e);
//         }
//     }
// }

/*
fn main(){
    println!("Hello, world!");
    println!("My name is {} and I'm {} years old", "Alex", 29);
    println!("a + b = {}", 3 + 9);
    println!("{0} has a {2} and {0} has a {1}", "Alex", "cat", "dog");
    println!("{name} {surname}", surname="Smith", name="Alex");
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 50, 50, 50);
    DEBUG:
    println!("array: {:?}", [1, 2, 3]);
}
*/


/// Variables

// fn main() {
//     let name = "Alex";
//     let mut age = 32;
//
//     let amount: i64 = 9387592743847;
//     println!("{}", age);
//
//     age = 43;
//
//     println!("{}", age);
//
//     let color = "blue";
//     let color = 86;
//
//     println!("{}", color);
//
//     let (a, b, c) = (43, 85, "red");
// }

/// Scalar data types
// fn main() {
//     let pi: f32 = 4.0;
//     let million = 1_000_000;
//     println!("{}", million);
//     let is_day = true;
//     let is_night = false;
//     println!("{}", is_day);
//     let char1 = 'A';
//     let char1 = '\u{1F601}';
//     println!("{}", char1);
// }

/// Strings

// fn main() {
//     let cat: &'static str = "Fluffy";
//     println!("{}", cat);
//
//     let mut dog = String::new();
//     let mut dog = String::from("Max");
//     println!("{}", dog);
//     let owner = format!("Hi I'm {} the owner of {}", "Mark", dog);
//     println!("{}", owner);
//
//     println!("{}", dog.len());
//     dog.push(' ');
//     dog.push_str("the dog");
//     println!("{}", dog);
//     let new_dog = dog.replace("the", "is my");
//     println!("{}", new_dog);
// }

/// Constants

const URL: &str = "google.com";

fn constants_demo() {
    println!("{}", URL);
}


/// Operators

fn operators_demo() {
    let a = 4 + 8;
    let b = 10 / 3;
    let c = 10 % 3;
    println!("a={}, b={}, c={}", a, b, c);
    println!("{}", a >= b);
    println!("{}", a >= b && b <= c);
}


/// Functions

// fn main() {
//     let mut name = "Aditya";
//     let greeting = say_hello(&mut name);
//     println!("{}", greeting);
// }
//
//
// fn say_hello(name: &mut &str) -> String {
//     let greeting = format!("Hello {}", name);
//     greeting
// }
mod player;
use crate::archive::arch::arch_file;

mod archive;

fn main(){
    player::play_mov ie("snatch.mp4");
    player::play_audio("rhcp.mp4");
    clean::perform_clean();
}

// Create a module with mod
mod clean{
    pub fn perform_clean(){
        println!("Cleaning")
    }
}
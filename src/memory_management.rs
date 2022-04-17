#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let i = 5;
    let j = i;
    println!("{}", j);
    println!("{}", i);

    let v = vec![1, 2, 3, 4, 5];
    // let w = v;
    // println!("{:?}", w);
    // println!("{:?}", v);

    // Borrowing Ownership
    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("Vector used in foo");
        v
    };
    let v = foo(v);
    println!("{:?}", v);
}

// Memory Borrowing

fn borrowing_demo() {
    let mut a = 6;
    {
        let b = &mut a;
        println!("{}", *b);
        *b += 2;
    }
    println!("{}", a);

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{}", i);
        v.push(6);
    }
}

// Lifetime

#[derive(Debug)]
struct Person {
    name: String
}

#[derive(Debug)]
struct Dog<'l> {
    name: String,
    owner: &'l Person
}

impl Person {
    // fn get_name(&self) -> &String {
    fn get_name<'l> (&'l self) -> &'l String {
        &self.name
    }
}

fn lifetime_demo() {
    println!("{}", get_str());

    let p1 = Person { name: String::from("John") };
    let d1 = Dog { name: String::from("Max"), owner: &p1};
    println!("{:?}", d1);

    let mut a: &String;
    {
        let p2 = Person { name: String::from("Mary")};
        // a = p2.get_name();
        a = p1.get_name();
    }
    println!("{}", a);
}

fn get_str() -> &'static str {
    "Hello"
}

// Reference Counted Variable
use std::rc::Rc;

struct Car {
    brand: Rc<String>
}

impl Car {
    fn new(brand: Rc<String>) -> Car { Car { brand: brand} }
    fn drive(&self) {
        println!("{} is driving", &self.brand);
    }
}

fn rc_demo() {
    let brand = Rc::new(String::from("BMW"));
    println!("pointers: {}", Rc::strong_count(&brand));
    {
        let car = Car::new(brand.clone());
        car.drive();
        println!("pointers: {}", Rc::strong_count(&brand));
    }
    println!("My car is a {}", brand);
    println!("pointers: {}", Rc::strong_count(&brand));
}
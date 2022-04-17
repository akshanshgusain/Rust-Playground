#[allow(unused_variables)]
#[allow(unused_assignments)]

fn array_demo(){
    let primes = [2, 3, 5, 7, 11];
    let doubles: [f64; 4] = [2.0, 4.0, 6.0, 8.0];
    println!("{:?}", primes);
    println!("{:?}", doubles);

    let numbers = [0;15];
    println!("{:?}", numbers);

    const DEFAULT: i32 = 3;
    let mut numbers = [DEFAULT; 15];
    println!("{:?}", numbers);
    println!("{:?}", numbers[3]);

    numbers[3] = 5;
    println!("{:?}", numbers);

    for number in numbers.iter() {
        println!("{}", number);
    }
}

fn vector_demo() {
    let primes: Vec<i32> = Vec::new();
    let mut primes = vec![2, 3, 5];
    println!("{:?}", primes);

    primes.push(7);
    println!("{:?}", primes);
    primes.remove(2);
    println!("{:?}", primes);

    let mut numbers = vec![2;10];
    println!("{:?}", numbers);

    const DEFAULT: bool = true;
    let values = vec![DEFAULT;8];
    println!("{:?}", values);

    numbers[5] = 8;
    println!("{:?}", numbers);

    for number in numbers.iter() {
        println!("{}", number * number);
    }
}


fn slice_demo() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("{:?}", slice);

    let mut colors = ["red", "green", "blue", "pink"];
    println!("{:?}", colors);
    update_colors(&mut colors[2..4]);
    println!("{:?}", colors);
}

fn update_colors(colors_slice: &mut [&str]) {
    colors_slice[0] = "yellow";
    colors_slice[1] = "orange";
}


fn tuple_demo() {
    let mut person: (&str, i64, bool) = ("John", 27, true);
    println!("{:?}", person);
    println!("{:?}", person.0);
    person.0 = "Mike";
    println!("{:?}", person.0);
    let (name, age, employment) = person;
    println!("name: {}, age: {}, employed: {}", name, age, employment);
}


fn struct_demo() {
    let emp = Employee {
        name: String::from("John"),
        company: String::from("Google"),
        age: 35
    };

    println!("{:?}", emp);
    println!("{}", emp.name);
    println!("{}", emp.fn_details());
    println!("{}", Employee::static_fn_detail());
}

#[derive(Debug)]
struct Employee {
    name: String,
    company: String,
    age: u32
}

impl Employee {
    fn fn_details(&self) -> String {
        format!("name: {}, age: {}, company: {} ", &self.name, &self.age, &self.company)
    }

    fn static_fn_detail() -> String {
        String::from("Details of a person")
    }
}

fn enum_demo() {
    let my_color = Colors::Red;
    println!("{:?}", my_color);
    let my_color = Red;
    println!("{:?}", my_color);

    let person = Name(String::from("Alex"));
    println!("{:?}", person);
}

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue
}

#[derive(Debug)]
enum Person {
    Name(String),
    Surname(String),
    Age(u32)
}



fn generics_demo() {
    let p1: Point<i32> = Point {X: 6, Y: 8};
    let p2: Point<f64> = Point {X: 3.25, Y: 8.63};
    println!("{:?}", p1);
    println!("{:?}", p2);

    let c1 = Red("#f00");
    let c2 = Red(255);
    println!("{:?}", c1);
    println!("{:?}", c2);

    let p3: Point2<i32, f64> = Point2 {x: 34, y: 8.5};
    println!("{:?}", p3);
}

#[derive(Debug)]
struct Point<T> {
    X: T,
    Y: T
}


#[derive(Debug)]
struct Point2<T, V> {
    x: T,
    y: V
}
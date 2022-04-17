#[allow(unused_variables)]
#[allow(unused_assignments)]

static mut R: i32 = 0;

fn scope_demo() {
    {
        let a = 3;
        println!("{}", a);
    }
    // println!("{}", a);

    unsafe {
        R = 4;
        println!("R = {}", R);
    }

    unsafe {
        println!("R = {}", R);
    }
}


// Lambda - Closures

fn closures_demo() {
    let a = |a: i32| a+1;
    println!("{}", a(6));
    let b = |b: i32| {
        let c = b + 1;
        c
    };
    println!("{}", b(4));

    let gen = |x| println!("{}", x);
    gen(3);
    // gen(true);
}

// Higher Order Functions

fn hof_demo() {
    let square = |a: i32| a * a;
    apply(square, 6);

    // Calculate the sum of all the squares less than 500
    // only for even numbers
    let limit = 500;
    let mut sum = 0;
    for i in 0.. {
        let isq = i * i;
        if isq > limit { break; }
        else {
            if is_even(isq) {
                sum += isq;
            }
        }
    }
    println!("The sum is {}", sum);

    //With HOFs
    let sum2 =
        (0..).map(|x| x * x)
            .take_while(|&x| x <= limit)
            .filter(|x| is_even(*x))
            .fold(0, |sum, x| sum + x);
    println!("The sum using HOFs is {}", sum2);
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn apply(f: fn(i32) -> i32, a: i32) {
    println!("Result {}", f(a));
}

// Macro


// () => (println!("First macro"))
// }

// macro_rules! name {
//     ($name: expr) => { println!("Hey {}", $name)}
// }

macro_rules! name {
    ($($name: expr),*) => ( $(println!("Hey {}", $name);)* )
}

macro_rules! xy {
    (x => $e: expr) => (println!("X is {}", $e));
    (y => $e: expr) => (println!("Y is {}", $e));
}

macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("{:?} was called", stringify!($fn_name));
        }
    }
}

fn macro_demo() {
    my_macro!();
    name!("John");
    name!("Alex", "Mary", "Carol");
    xy!(x => 5);
    xy!(y => 3 * 9);
    build_fn!(hey);
    hey();
}
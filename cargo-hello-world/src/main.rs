macro_rules! log_error {
    () => {
        println!("This is an error log {:?}", message);
    };
}

fn main() {
    scalar_types();
    println!("return value is: {}", last_char(String::from("Hello")));
    println!("result is {}", is_over_20(40).to_string());
    println!("{}", is_over_20(10));
    println!("{}", is_over_20(10000));
    greet(12);
    greet(23);
    greet(100);
    ownership_and_borrowing();
    log_error!();
}

fn scalar_types() {
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is {}", x);

    const NAME_CONST: &str = "Adekunle";
    println!("The value of NAME_CONST is {}", NAME_CONST);

    x = 6;
    println!("The value of x is {}", x);
}

fn last_char(arg: String) -> char {
    if arg.is_empty() {
        return 'f';
    }

    arg.chars().next_back().unwrap()
}

fn is_over_20(param: u32) -> bool {
    let result = param > 20;
    result
}

/// greet people based on the
fn greet(time: u8) {
    return match time {
        0..=11 => println!("Good Morning"),
        12..=17 => println!("Good Aftenoon"),
        18..=23 => println!("Good Evening"),
        _ => println!("Greet yourself"),
    };
}

fn ownership_and_borrowing() {
    let name = String::from("adekunle");
    let name1 = &name;
    println!("{} - {}", name, name1);
    // drop(name); this is not possible because of the borrowing happening above
    println!("{}", name1);
}

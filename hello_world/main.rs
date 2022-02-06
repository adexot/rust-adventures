fn main() {
    let num = 3;
    let mut sum = 0;
    let arr = [1, 1, 3, 5, 44, 5, 8, 5];

    for i in 0..=5 {
        sum += i
    }
    println!("The square of {} is {}", num, square(num));
    println!("The sum is => {}", sum);
    println!("Now try to access something that is not there => {}", num);

    let i = 10;
    let res1 = by_ref(&i);
    mut_ref(&mut sum);
    println!("res1 => {} sum => {} i => {}", res1, sum, i);
    println!("arr => {:?}", &arr[0..5]);
    println!("actual arr => {:?}", arr);
    println!("actual arr => {:?}", arr);

    let tup = ("ade", 2, true);
    let (first, _second, _third) = tup;
    println!("turple value is {}", first);

    let _b = &i;

    if i == res1 {
        println!("Equal");
    } else {
        println!("Not Equal");
    }

    let str = "kunle";
    let mut str_two = String::from("ade");

    makes_copy(str);
    makes_copy(&str_two);
    change_value(&mut str_two);
    change_value(&mut str_two);

    println!("Trying to access the value of => {} {}", str, str_two,);

    test_example();
}

fn square(x: u32) -> u32 {
    x * x
}

fn by_ref(x: &i32) -> i32 {
    *x + 1
}

fn mut_ref(x: &mut i32) {
    *x = 1000
}

fn makes_copy(same_string: &str) {
    println!("makes copy of the string => {}", same_string);
}

fn change_value(str: &mut String) {
    str.push_str(", the new word is added");
}

fn test_example() {
    let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

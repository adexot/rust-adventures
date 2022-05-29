use std::{str::FromStr, env::args, process::exit};

fn main() {
    let values_list = get_env_values();

    println!("Sum values from the cmd");
    let mut env_sum = values_list[0];

    for m in &values_list[1..]{
        env_sum = sum(env_sum, *m);
    }

    println!("Sum of {:?} is {}", values_list, env_sum);
}

fn sum(n: u64, m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    n + m
}

fn get_env_values() -> Vec<u64>{
    let mut env_vars = Vec::new();
    let cmd_inputs = args().skip(1);

    if cmd_inputs.len() == 0 {
        println!("Please provide integer values");
        exit(1);
    }

    for arg in cmd_inputs{
        env_vars.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    env_vars
}

#[test]
fn test_sum(){
    assert_eq!(sum(2,1), 3)
}

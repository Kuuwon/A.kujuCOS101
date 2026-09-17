// Project II: Rust program to determine an employee's annual incentive

use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experienced = input1.trim().to_lowercase();

    println!("Enter the employee's age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if experienced == "yes"
    {
        if age >= 40
        {
            println!("Annual incentive: N1,560,000");
        }
        else if age >= 30
        {
            println!("Annual incentive: N1,480,000");
        }
        else if age < 28
        {
            println!("Annual incentive: N1,300,000");
        }
        else
        {
            println!("No incentive band defined for this age");
        }
    }
    else
    {
        println!("Annual incentive: N100,000");
    }
}

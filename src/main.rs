use std::io;

mod moving_ball;
mod spining_balls;

fn main() {
    let choice=menu();
    match choice {
        1=> moving_ball::run(),
        2=> spining_balls::run(),
        _=> println!("Wrong choice! "),
    };
}

fn menu()->u32{
    println!("--- MENU ---");
    println!("1. Moving Ball");
    println!("2. Spining Balls");
    print!("Enter your choice: ");
    
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Failed to read from stdin!");

    let trimmed=input.trim();
    match trimmed.parse::<u32>(){
        Ok(i)=>return i,
        Err(..)=>println!("This is not an integer: {}",trimmed),
    };
    return 0;
}

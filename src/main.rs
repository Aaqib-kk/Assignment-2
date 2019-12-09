#[allow(dead_code)]
use std::f32;
use std::io;
fn main() {
    loop {

    println!("Please Enter you numbers\nEnter First Number");
    let mut x = taking_input();
    let num1 = convert_f32(&mut x);

    //editing
    if num1 == 0.0 {
        break;
    }
    //

    println!("Enter Second Number");
    let mut y = taking_input();
    let num2 = convert_f32(&mut y);

    println!(
        "\nEnter the function you want to perform \n1) Addition\n2) Subtraction\n3) Multiplication\n4) Division\n5) Exponent\n6) Quit your Choice"
    );
    let choice = taking_input();
    let choice: u32 = choice.trim().parse().expect("error");

    match choice {
        1 => println!("Addition result is {}", num1 + num2),
        2 => println!("Subtraction result is {}", num1 - num2),
        3 => println!("Division result is {}", num1 / num2),
        4 => println!("Multiplication result is {}", num1 * num2),
        5 => println!("Exponent result is {}", num1.powf(num2)),
        _ => println!("Invalid character is entered"),
    }
    }
}

fn taking_input() -> String {
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("failed");
    num1
}

fn convert_f32(x: &mut String) -> f32 {
    
    // let x: f32 = x.trim().parse().expect("errsadasdor");
    // x

    match x.trim().parse::<f32>()
        {
        Ok(T) =>  { return T }
        Err(T) => {  println!("You have entered invalid number!, Please Try Agin");
                    return 1.0            

                    }
    }
}
/*
use std::io;

fn read_user_input_character () -> f32 {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input);
    match user_input.trim().parse::<f32>()
        {
        Ok(T) =>  { return T }
        Err(_) => { return 0.0; }
    }
}

fn main()
{

    loop 
    {
    println!("\nPlease enter a letter to guess:");
    let user_char = read_user_input_character();

    /* Exit if user enters an asterisk ('*') */
    if user_char == 0.0 {
        break;
    }
    println!("input character is {}", user_char);
    }
    
    
}

*/



/*

Q1.Project Calculator Write a calculator program. A minimal calculator will support the following
functions:
•numbers with decimals (not just integers)
•addition (1 + 2 is 3) •subtraction (12 -4 is 8)
•multiplication (33 * 2 is 66) •division (3 /8 is 0.375)
•exponents (2 ^ 3 is 8)
•error messages when you do something wrong Your calculator should keep on running until explicitly
told to quit. I suggest typing a zero as the first operand to cause it to quit, i.e. Program Console
Sample:>2 + 3 5 >4 * 9 36 >0+2
ByeHint: Well, if you read in everything as a String, then you can convert to other things.
What to avoid:Any program, which presents me with a screen like the following, will notreceive
a very good score.
Program Console Sample:
Enter the function you wish to perform.
1)addition
2)subtraction
3)multiplication
4)division
5) quit Your choice:
Also, the same fate applies to any program that ever presents me with the following message:
Would you like to calculate again? (y/n) Finally, you may use the built-in function in order
to compute powers, but those that write their own will receive a much higher score.

*///



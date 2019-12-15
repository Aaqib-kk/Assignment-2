/*
Q1. Project Calculator
Write a calculator program. A minimal calculator will support the following functions:
// */

// #[allow(dead_code)]
// use std::f32;
// use std::io;
// fn main() {
    
//     loop { 
//     println!( "\n\t\t*** Simple Calculator ***\n\nEnter the Function you want to perform \n1) Addition\n2) Subtraction\n3) Multiplication\n4) Division\n5) Exponent\n6) Enter 0 to Quit");
//     let mut choice = taking_input();
//     let choice = convert_f32(&mut choice);

//     if choice != 10.0 && choice != 0.0
//     {
//         println!("Please Enter you numbers\nEnter First Number ");
//         let mut x = taking_input();
//         let num1 = convert_f32(&mut x);
//         if num1 == 10.0 {
//             println!("\n*** PLEASE ENTER NUMBER AGAIN ***\n");
//             continue;
//         }

//         println!("Enter Second Number");
//         let mut y = taking_input();
//         let num2 = convert_f32(&mut y);
//         if num2 == 10.0 {
//             println!("\n** PLEASE ENTER NUMBER AGAIN ***\n");
//             continue;
//         }
        
//         match choice {
//             1.0 => println!("Addition result is {}", num1 + num2),
//             2.0 => println!("Subtraction result is {}", num1 - num2),
//             3.0 => println!("Division result is {}", num1 / num2),
//             4.0 => println!("Multiplication result is {}", num1 * num2),
//             5.0 => println!("Exponent result is {}", num1.powf(num2)),
//             _   => println!("Invalid number is added"),
//         }
//     }
//     else if choice == 10.0 {
//         continue;
//         }
//     else if choice == 0.0 {
//         break;
//         } 
//     }
// }

// fn taking_input() -> String {
//     let mut num1 = String::new();
//     io::stdin().read_line(&mut num1).expect("failed");
//     num1
// }

// fn convert_f32(x: &mut String) -> f32 {
    
//     match x.trim().parse::<f32>()
//         {
//         Ok(t) =>  { return t }
//         Err(_t) => {  println!("You have entered invalid number!, Please Try Agin");
//                     return 10.0 }
//     }
// }

/* Q3.We learned to use closures as arguments to other functions. 
Can you use a closure as a value of field of a struct?
Read Listing 13-10: The caching logic ofCacherto learn about it
and then create your own example. Be creative */

// use std::ops::{Add, Sub};

// #[derive(Debug, PartialEq)]
// struct Numbers <T>
// {
//     x: T,
//     y: T,
// }

// impl <T> Add for Numbers <T>
// where T: Add<Output = T> 
// {
//     type Output = Self;
//     fn add(self, other: Self) -> Self::Output
//     {
//         Self
//         {
//             // self.x + self.y
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// fn main() 
// {
//     let number_list = Numbers
//     {
//         x: 5,
//         y: 10,
//     };
//     println!("{:?}", number_list.x + number_list.y);
// }
/*
Q4. Mr. Asim wants to adopt children from Yateem khana.
He wants to adopt those children only who have the primary 
education and are at least bilingual. Mr. Allah Bakhsh is
the head of Yateem Khana. He is helping Mr Asim to make 
his visit successful. He is showing him 2 children at a 
time by passing those 2 children(struct) in a function named 
adopt(). If both of the children have both of the above mentioned
qualities(traits), then Mr. Asim happily brings them to their new 
home sweet home. How can you help Mr. Allah Bakhsh / Mr.Asim?*/
// #[derive(Debug)]
// struct Childern {

//     child1: String,
//     child2: String,
// }
// pub trait Qualities {
//     fn adopt(&self) -> String {
//         format!("Childern are not educated nor bilingual")
//     }
// }
// impl Qualities for Childern {
//     fn adopt(&self) -> String {
//         format!("Both child are educated")
//     }
// }
// fn main() {

//     let childer_qualities = Childern {
//         child1: String::from("Mukhtiar"),
//         child2: String::from("Sarkar"),
//     };
//     println!("{:?}", childer_qualities.adopt());
// }
/*
Q5. What are the 4 qualities of closure?
Hint: Sir Naufil highlighted each of them with different
highlighters in documentation
*/

// 1 - Making a clouser with no arguments
// use std::io;
// fn main()
// {
    // let x = || println!("Clouser with no argument");
    // x();    
// }

// 2 -  capturing values from enviroment
// fn main()
// {
// let mut number = String::new();
//     io::stdin().read_line(&mut number).expect("error");
//     let number: u32 = number.trim().parse().expect("error");
//     println!("Printing Table of {}", number);
//     let mut answer:u32 = 1;
//     for i in 1..11
//     {
//         let mut _f = || {
//             answer = number * i; 
//             println!("{} * {} = {}", number, i, answer );
//         };
//         _f();
//     }
// }
    

// 3 - passing a closure as an argument to a function
// fn hello<T:Fn()>(x:T){
//     x();
// }
// fn main() {
//     let x = || println!("Hey this is MAK = Muhammad Aqib Khan");
//     hello(x);
// }

// 4 - A closure expecting some argument and returning argument to a function
// fn call<T:Fn(u32)->u32>(x:T)->u32{
//     x(2)
// }
// fn main() {
//     let x = |y| y + 1;
//     println!("{}", call(x));
// }
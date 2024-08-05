#![allow(unused)]
fn main() {
    // println!("Hello, world!");

    /* call function without arguments */
    // another_function();

    /* call function with argument */
    // another_function(5);

    // print_labeled_measurement(5, 'h');

    /* __________ Statements and expressions __________ */
    // let x = 5; // this line is an statement and 5 is expression
    // let x = let y = 5; // this lines provide an error becasue let y = 5; is an statement and statement doesn't return any value
    // calling a function is and express and macros are expression
    // new scope block created by curly brackets is also expression
    // like:
    // let y = {
    //     let x = 5;
    //     x + 1
    // };

    //println!("{y}"); // println macro is also and expression

    /* __________ Functions with Return Values __________ */
    // fn five() -> i32 {
    //     5
    // }

    // let x = five();

    // println!("The value of x is : {}", x);

    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    let x = plus_one(5);

    println!("The value of x is : {}", x);
}

/* function body for function without parameter  */
// fn another_function() {
//     println!("Another function.");
// }

/* function body for function with parameter */
// fn another_function(x: i32) {
//     println!("The value of x is {x}");
// }

// fn print_labeled_measurement(value: i32, unit_lable: char) {
//     println!("The measurement is: {value}{unit_lable}")
// }

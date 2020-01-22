/* By default, Rust brings only a few types into the scope of every program in the prelude.
*  If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly
*  with a use statement.
*  Using the std::io library provides you with a number of useful features,
*  including the ability to accept user input.
*/
use std::io;

/*
* The fn syntax declares a new function, the parentheses, (),
* indicate there are no parameters, and the curly bracket, {, starts the body of the function.
*/
fn main() {
    println!("Hello, world!");

    println!("Guess the number!");

    println!("Please enter your guess!");
    /*
    *   The 'let' keyword is used for variable declarationwhile
    *  the 'mut' keyword denotes a variable that is mutable since recall that all variables are
    *  immutable by default in rust.
    */
    let mut guess = String::new();
    /* The :: syntax in the ::new line indicates that new is an associated function of the String type.
    *  An associated function is implemented on a type,
    *  in this case String, rather than on a particular instance of a String.
    *  Some languages call this a static method.
    */


    /*
    * The Result types are enumerations, often referred to as enums.
    * An enumeration is a type that can have a fixed set of values, and those values are called the enum’s variants.
    *
    *
    * For Result, the variants are Ok or Err. The Ok variant indicates the operation was successful,
    * and inside Ok is the successfully generated value. The Err variant means the operation failed,
    * and Err contains information about how or why the operation failed.
    *
    *
    * The purpose of these Result types is to encode error-handling information.
    * Values of the Result type, like values of any type, have methods defined on them.
    * An instance of io::Result has an expect method that you can call. If this instance of io::Result is an Err value,
    * expect will cause the program to crash and display the message that you passed as an argument to expect.
    * If the read_line method returns an Err, it would likely be the result of an error coming from the underlying
    * operating system. If this instance of io::Result is an Ok value, expect will
    * take the return value that Ok is holding and return just that value to you so you can use it.
    * In this case, that value is the number of bytes in what the user entered into standard input.
    */
    io::stdin().read_line(&mut guess) // Returns a result type
        .expect("Failed to read given input!"); // lines seperated for clarity

    /* This line prints the string we saved the user’s input in. The set of curly brackets, {},
    *  is a placeholder: think of {} as little crab pincers that hold a value in place.
    *  You can print more than one value using curly brackets: the first set of curly brackets
    *  holds the first value listed after the format string, the second set holds the second value, and so on
    */
    println!("You guessed: {}", guess);
}

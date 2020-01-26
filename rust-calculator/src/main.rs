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

    println!("Please enter your first number!");
    /*
    *  The 'let' keyword is used for variable declaration, while
    *  the 'mut' keyword denotes a variable that is mutable since recall that all variables are
    *  immutable by default in rust.
    *
    *  The :: syntax in the ::new line indicates that new is an associated function of the String type.
    *  An associated function is implemented on a type,
    *  in this case String, rather than on a particular instance of a String.
    *  Some languages call this a static method.
    *
    */
    let mut number1 = String::new();



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
    io::stdin().read_line(&mut number1) // Returns a result type
        .expect("Failed to read given input!"); // lines seperated for clarity

    while !verify_numeric_input(number1.as_mut_str()){
        println!("Please enter your first number! Ensure it is a numberic value");

        number1.clear();

        io::stdin().read_line(&mut number1)
            .expect("Failed to read given input!");
    }

    println!("Please enter your second number!");

    let mut number2 = String::new();

    io::stdin().read_line(&mut number2) // Returns a result type
        .expect("Failed to read given input!"); // lines seperated for clarity

    while !verify_numeric_input(number2.as_mut_str()){
        println!("Please enter your second number! Ensure it is a numberic value");

        number2.clear();

        io::stdin().read_line(&mut number2)
            .expect("Failed to read given input!");
    }

    let mut operation = String::new();

    /*
    * In Rust the type of an array does not have to be explicitly stated if since they have to share the same type.
    * If one were to explicitly state the array declarion it would be [&str; 5] where [TYPE; LENGTH].
    * If this varible were to be assigned as a const (constant), then the type would have to be
    * declared before hand.
    */
    let arr = ["Addition", "Subtraction", "Multiplication", "Division", "Modulo"];

    /*
    * Again, Rust allows us to declare the type of the varible before use or initialization.
    * In this specifc case, it is very redunant becuase the String::with_capacity method creates
    * and returns a new string, so the type of words could be inferred but never the less it is
    * good to know.
    */
    let mut words : String = String::with_capacity(40);
    /*
    * A basic for loop in rust. It takes an iterator type prodcued by the array and
    * bonds that value to the scoped variable i.
    *
    */
    for i in arr.iter() {
        /*
        * push_str in Rust is very similar to the .append method in Java (from the StringBuffer class)
        */
        words.push_str(i);
        words.push_str(" ");
    }
    println!("Please enter your intedend operation! Your choices are {}", words);


    io::stdin().read_line(&mut operation) // Returns a result type
        .expect("Failed to read given input!"); // lines seperated for clarity

    while !verify_operation(&operation) {
        println!("Please enter your intedend operation again! Your choices are {}", words);

        operation.clear();

        io::stdin().read_line(&mut operation) // Returns a result type
            .expect("Failed to read given input!"); // lines seperated for clarity
    }

    /*
    * Gotta be honest didn't think functional composition would work in this way but it does,
    * the thing is if this were a 'real' program we would want to transform the String struct to a
    * i64 number immediately in order to save space.
    */
    let final_result = calculator(operation, transform_input(number1.as_mut_str()),
                                                    transform_input(number2.as_mut_str()));

    /* This line prints the string we saved the user’s input in. The set of curly brackets, {},
    *  is a placeholder: think of {} as little crab pincers that hold a value in place.
    *  You can print more than one value using curly brackets: the first set of curly brackets
    *  holds the first value listed after the format string, the second set holds the second value, and so on
    */
    println!("Your result is {}", final_result);
}

/*
* We want to verify that the given user operation is actually a viable one.
*
*/
fn verify_operation(input : &str) -> bool {
    let match_pattern = input.trim().to_ascii_uppercase();
    return match match_pattern.as_str() {
        "ADDITION" | "SUBTRACTION" | "MULTIPLICATION" | "DIVISION" | "MODULO" => true,
        _ => false
    };
}

/*
* Verifies that the given string can be converted into a integer primitive values.
* Returns true if the value can be parsed into an integer value, false otherwise.
*
* @parameters input a string slice
*/
fn verify_numeric_input(input : &str) -> bool {
    return match input.trim().parse::<i64>() {
        Ok(_n) => true,
        Err(_e) => false
    };
}

/*
* Turns a string value into an integer primitive value,
* if the string cannot be parsed 0 is returned.
*
* @parameters input a string slice
*/
fn transform_input (input : &str) -> i64 {
    return match input.trim().parse::<i64>() {
        Ok(n) => n,
        Err(_e) => 0
    };
}

/*
*
*
*/
fn calculator(operation : String, number1 : i64, number2: i64) -> i64 {
    let oper = operation.trim().to_ascii_uppercase();
    return match oper.as_str() {
        "ADDITION" => number1 + number2,
        "MULITIPLICATION" => number1 * number2,
        "SUBTRACTION" => number1 - number2,
        "DIVISION" => number1 / number2,
        "MODULO" => number1 % number2,
        _ => 0
    };
}

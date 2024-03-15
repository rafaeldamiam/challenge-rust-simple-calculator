// TO RUN THIS CODE USE: cargo watch -x run

use std::io;

fn main() {
    let number_one = ask_number();
    let operator = ask_operator();
    let number_two = ask_number();
    let mut result: String = " ".to_string();
    
    match operator.as_str(){
        "+"=> result = addition(number_one,number_two),
        "-"=> result = subtraction(number_one,number_two),
        "x"=> result = multiplication(number_one,number_two),
        "/"=> result = division(number_one,number_two),
        _=> println!("Enter a valid operator!"),
    }

    if result != " ".to_string(){
        //macro need ! to be used
        println!("{number_one} {operator} {number_two} = {result}");
    }
}

//TO_CALC FUNCTIONS
fn addition(n1: f64,n2: f64) -> String{
    let sum = (n1 + n2).to_string();

    return sum;
}
fn subtraction(n1: f64,n2: f64) -> String{
    let minus = (n1 - n2).to_string();

    return minus;
}
fn multiplication(n1: f64,n2: f64) -> String{
    let times = (n1 * n2).to_string();

    return times;
}
fn division(n1: f64,n2: f64) -> String{
    let divide = (n1 / n2).to_string();

    return divide;
}


//ASK FUNCTIONS
fn ask_operator() -> String {
    let mut input = String::new();
    println!("Please, enter the operator (+,-,x,/): ");
    io::stdin().read_line(&mut input).unwrap();

    let oper = input.trim().to_string();

    return oper;
}

fn ask_number() -> f64 {
    let mut input = String::new();
    println!("Please, enter the number: ");
    io::stdin().read_line(&mut input).unwrap();
    let number: f64 = input.trim().parse().unwrap();

    return number;
}
use std::io;
use rand::Rng;

pub fn run(question: u32){
    match question {
        1 => hello(),
        2 => personalized_greeting(),
        3 => temp_convert(),
        4 => {
            let num = 5;
            println!("The factorial of {} is {}", num, factorial(num));
        },
        5 => odd_or_even(5),
        6 => sum_range(20),
        7 => fizz_buzz(),
        8 => {
            let s = String::from("maham");
            println!("{} is a palindrome: {}", s, is_palindrome(&s));
        },
        9 => guess_game(),
        10 => {
            let num1 = -5;
            let num2 = 2;
            println!("The sum of {} and {} is {}", num1, num2, calculate(num1, num2, '+'));
            println!("{} subtracted by {} is {}", num1, num2, calculate(num1, num2, '-'));
            println!("The product of {} and {} is {}", num1, num2, calculate(num1, num2, '*'));
            println!("{} divided by {} is {}", num1, num2, calculate(num1, num2, '/'));
        },
        11 => {
            let arr: [i32; 5] = [-1, 23, -98, 100, 2];
            println!("The maximum value in array is {}", find_maximum(&arr));
        },
        12 => {
            let s = String::from("hello");
            println!("{} reversed is {}", s, reverse_string(&s));
        },
        13 => generate_fibonacci(8),
        14 => get_all_prime(100),
        15 => {
            let mut str: String = String::from("hello");
            let str: String = demo_ownership(str);
            println!("{}", str);
        },
        16 => todo!(),
        17 => {
            let s = String::from("hello");
            print_first_and_last(s);
        }
        18 => {
            let s = String::from("holaxholaxholaxyz");
            let sub = String::from("z");
            println!("The string {} contains {}, {} times.", s, sub, contains_substring(&s,&sub));
        }
        _ => println!("Not implemented yet!")
    }
}

// Question 1 -> Hello, Rust

fn hello(){
    println!("Hello, Rust");
}

// Question 2 -> personalized greeting

fn personalized_greeting(){
    let mut name = String::new();
    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("failed to read line");
    println!("Hello, {}", name);
}

// Question 3 -> celsius to farenhiet
fn temp_convert(){
    let mut temp: f64;
    let mut inp_str = String::new();

    println!("Enter Temperature: ");
    io::stdin().read_line(&mut inp_str).expect("failed to read line");
    temp = inp_str.trim().parse().expect("not a valid number");
    inp_str = String::from("");
    println!("Enter scale: ");
    io::stdin().read_line(&mut inp_str).expect("failed to read line");

    print!("inp str = {}",inp_str);

    if inp_str.trim() == String::from("C") || inp_str.trim() == String::from("c") {
        inp_str = String::from("F");
        temp = temp * 9.0 / 5.0 + 32.0;
    } else {
        inp_str =String::from("C");
        temp = (temp - 32.0) * 5.0 / 9.0;
    }
    
    println!("Converted temperature is {:.2}{}", temp, inp_str);
}

// Question 4 -> factorial

fn factorial(num: u32) -> u32{
    if num <= 1 {
        return 1;
    }
    num * factorial(num - 1)
}

// Question 5 -> odd or even

fn odd_or_even(num: u32){
    if num % 2 == 0 {
        println!("{} is Even", num);
    } else {
        println!("{} is Odd", num);
    }
}

// Question 6 -> sum of n integers

fn sum_range(n: u32){
    let mut sum = 0;
    for i in 1..n+1{
        sum += i;
    }
    println!("The sum of 1 to {} is {}.", n, sum);
}

// Question 7 -> Fizz Buzz
fn fizz_buzz(){
    for i in 1..101{
        if is_divisible(i, 3) && is_divisible(i, 5){
            print!("FizzBuzz ");
        } else if is_divisible(i, 3) {
            print!("Fizz ");
        } else if is_divisible(i, 5) {
            print!("Buzz ");
        } else {
            print!("{} ", i);
        }
    }
    println!("");
}

// Question 8 -> palindrome
fn is_palindrome(s: &String) -> bool{
    let s_vec: Vec<char> = s.chars().collect();
    let mut lo = 0;
    let mut hi = s.len() - 1;
    while lo <= hi{
        if s_vec.get(lo) != s_vec.get(hi) {return false;}
        lo+=1;
        hi-=1;
    }
    true
}

// Question 9 -> guess random number 
fn guess_game(){
    let guess_num = rand::thread_rng().gen_range(1..=100);
    
    
    loop {
        let mut guess = String::new();
        println!("Enter your guess: ");
        io::stdin().read_line(& mut guess).expect("failed to readline");
        let guess: u32 = guess.trim().parse().expect("not a number");

        if guess > guess_num{
            println!("Too high!");
        } else if guess < guess_num {
            println!("Too low");
        } else {
            println!("Congrats! You guessed right!");
            break;
        }
    }
}

// Question 10 -> Basic Calculator

fn calculate(num1: i32, num2: i32, op: char) -> i32 {
    match op {
        '+' => add(num1, num2),
        '-' => sub(num1, num2),
        '/' => div(num1, num2),
        '*' => mul(num1, num2),
        _ => todo!(),
    }
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn sub(num1: i32, num2: i32) -> i32 {
    num1 - num2
}

fn mul(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn div(num1: i32, num2: i32) -> i32 {
    num1 / num2
}

// Question 11 -> Find Maximum

fn find_maximum(arr: &[i32]) -> i32{
    let mut max: i32 = -2147483648;
    for i in 0..arr.len(){
        if arr[i] > max {
            max = arr[i];
        }
    }
    return max;
}

// Question 12 -> Reverse String

fn reverse_string(s: &String) -> String{
    let mut rs = String::new();
    for char in s.chars(){
        rs = char.to_string() + &rs;
    }
    return rs;
}

// Quesion 13 -> Fibonacci Series upto N number of terms

fn generate_fibonacci(n: u32){
    let mut num1 = 0;
    let mut num2 = 1;
    println!("Fibonacci Series: ");
    print!("{} + ", num1);
    for _i in 2..n+1 {
        print!("{} + ", num2);
        let temp = num1;
        num1 = num2;
        num2 = temp + num2;
    }
    println!("...");
}

// Question 14 -> Print all prime numbers upto a given range, N

fn is_divisible(dividend: u32, divisor: u32) -> bool {
    dividend % divisor == 0
}

fn is_prime(suspect: u32) -> bool {
    for i in 2..suspect/2 {
        if is_divisible(suspect, i) { 
            return false;
        }
    }
    true
}

fn get_all_prime(n: u32){
    println!("Prime Numbers: ");
    for i in 2..n+1 {
        if is_prime(i){
            print!("{} ", i);
        }
    }
    println!("");
}

// Question 15 -> demonstrate ownership

fn demo_ownership(mut str: String) -> String {
    str.push_str(", world!");
    str
}

// Question 17 -> print first and last of String
fn print_first_and_last(s: String) {
    let size = s.len();
    if size > 0 {
        println!("First: {}", &s[0..1]);
        println!("Last: {}", &s[size - 1..size]);
    }
}

// qUESTION 18 -> count substring
fn contains_substring(s: &String, sub: &String) -> u32 {
    let mut i = 0;
    let mut count = 0;
    while i <= s.len() - sub.len() {
        if s[i..i+sub.len()] == sub[0..sub.len()] {
            i += sub.len();
            count += 1;
        } else {
            i += 1;
        }
    }
    count
}


/**
 * 猜数字
 * 2024.03.23 by dralee
 */
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let mut ver = String::new();
    println!("input the guess version:(1~3)");
    io::stdin()
        .read_line(&mut ver)
        .expect("failed to read line");
    let ver: u32 = ver.trim().parse().expect("please type a number");
    if ver.cmp(&1).is_eq() {
        guess_1();
    } else if ver.cmp(&2).is_eq() {
        guess_2();
    } else if ver.cmp(&3).is_eq() {
        guess_3();
    } else {
        println!("the guess version is from 1 to 3!");
    }
}

fn guess_1() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = guess.trim().parse().expect("please type a number");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

fn guess_2() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = guess.trim().parse().expect("please type a number");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn guess_3() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){ // 输入非数字，而不报错，直接跳过
            Ok(num)=> num,
            Err(_) => continue,  // continue for the loop
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // break for the loop
            }
        }
    }
}
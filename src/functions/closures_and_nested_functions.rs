use chrono::{DateTime, Utc};   
use std::time::Duration;
use std::thread::sleep;

fn closure_no_params() {
    let get_timestamp = || -> DateTime<Utc> { Utc::now() };
    println!("Timestamp:   {}", get_timestamp());//.format("%T"));
}

fn closure_one_param() {
    let reciprocal = |n: f64| -> f64 { if n == 0.0 {0.0} else {1.0 / n} };
    println!("Reciprocal:  {}", reciprocal(5.0));
}

fn closure_many_params() {
    let prod = |a, b| -> i32 { a * b };  // type can be inferred from the context
    println!("Product:     {}", prod(20, 5));
}

fn closure_multiple_statements() {
    let get_timestamp_after_delay = |seconds: u64| -> DateTime<Utc> {
        sleep(Duration::new(seconds, 0));
        Utc::now()
    };  
    println!("Timestamp:   {}", get_timestamp_after_delay(5).format("%T"));
}


fn capture_immutable_reference() {

    let b1 = String::from("┌─────────────────┐");
    let b2 = String::from("└─────────────────┘");

    let display_heading = |s| {
        println!("{}", b1);
        println!("│ {:<15} │", s);
        println!("{}", b2);
    };

    display_heading(String::from("hello"));
    display_heading(String::from("goodbye!"));

    println!("b1: {}\nb2: {}\n", b1, b2);
}

fn capture_mutable_reference() {

    let mut b1 = String::from("┌─────────────────┐");
    let mut b2 = String::from("└─────────────────┘");

    let mut display_heading = |s| {
        b1.push_str("✅");
        b2.push_str("✅");
        println!("{}", b1);
        println!("│ {:<15} │", s);
        println!("{}\n", b2);
    };

    display_heading(String::from("hello"));
    display_heading(String::from("goodbye!"));

    println!("b1: {}\nb2: {}\n", b1, b2);
}

fn capture_value_automatically() {
    let message = String::from("hello");

    println!("Message initially:  {}", message);

    // consume_message moves 'message' into closure here.
    let consume_message = || {
        println!("Message in closure: {}", message);
        std::mem::drop(message);
    };

    // Can't use 'message' here, it's owned by the closure.
    // println!("{}", message);  // Nope!

    // We can call consume_message() once.
    consume_message();

    // But we can't call consume_message() again.
    // consume_message();        // Nope!
}

fn capture_value_forcibly() {

    let message = String::from("HELLO");

    println!("Start of method...");

    // We must forcibly move captured values into closure (otherwise it won't compile).
    std::thread::spawn(move || {
        println!("Message at start of closure: {}", message);
        std::thread::sleep(Duration::new(5, 0));
        println!("Message at end of closure:   {}", message);
    });

    println!("End of method...");
}

pub fn nested_function() {
    let a = 5;
    fn sqr(i: i32) -> i32 {
        i * i + a
    }

    println!("Square of 5 is {}", sqr(5));
    println!("Square of 7 is {}", sqr(7));
}

pub fn nested_functions() {
    let some_value = 5;
    // nested functions can't access external variables e.g. some_value
    fn sqr(i: i32) -> i32 {
        i * i
    }

    println!("Square of 5 is {}", sqr(5));
    println!("Square of 7 is {}", sqr(7));
}
use std::{alloc::{alloc, Layout}, any::{type_name_of_val, Any}, mem::{size_of, transmute}};
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};

/**
 * 元数据实现的反射
 * 2024.04.22 by dralee
 */
fn main() {
    types();
    is_type();
    any_types();
    dyn_call();
    serialize();
    dyn_create();
}

fn types(){
    println!("------------- types -----------");
    let a = 1;
    let b = "well";
    let c = true;
    let d = 1.1;
    let e = "what";

    println!("type a: {:?}, a type name: {}", a.type_id(), type_name_of_val(&a));
    println!("type b: {:?}, b type name: {}", b.type_id(), type_name_of_val(&b));
    println!("type c: {:?}, c type name: {}", c.type_id(), type_name_of_val(&c));
    println!("type d: {:?}, d type name: {}", d.type_id(), type_name_of_val(&d));
    println!("type e: {:?}, e type name: {}", e.type_id(), type_name_of_val(&e));
/*
------------- types -----------
type a: TypeId { t: 115387559057565692143404304070439989267 }
type b: TypeId { t: 94774005424224989474030617523037649708 }
type c: TypeId { t: 324675245860759320943513204350442872190 }
type d: TypeId { t: 8711759054683223602271599665973969982 }
type e: TypeId { t: 94774005424224989474030617523037649708 }
*/
}

fn is_type() {
    println!("------------- is type -----------");
    is_string(&"Hello".to_string());
    is_string(&"Hello");
    is_string(&1);    
    is_string(&true);
}
fn is_string(s: &dyn Any) { // Any类型
    if s.is::<String>() {
        println!("{:?} is a string.", s);
    } else {
        println!("{:?} is not a string.", s);
    }
}

fn any_types(){
    println!("------------- any type -----------");
    let x = vec![1,2,3];
    let y = vec!['a','b','c'];
    let z = vec!["a","b","c"];

    print_type(&x);
    print_type(&y);
    print_type(&z);
}
fn print_type<T: Any>(val: &T) {
    let v_any = val as &dyn Any;
    if let Some(i) = v_any.downcast_ref::<Vec<i32>>() {
        println!("the type of {:?} is Vect<i32>", i);
    } else if let Some(c) = v_any.downcast_ref::<Vec<char>>() {
        println!("the type of {:?} is Vect<char>", c);
    } else if let Some(s) = v_any.downcast_ref::<Vec<&str>>() {
        println!("the type of {:?} is Vect<&str>", s);
    }else{
        println!("unknown type");
    }
}

fn dyn_call(){
    println!("------------- dynamic call function -----------");
    let addr_ptr = add as *const u8;
    let add_fn: fn(i32, i32) -> i32 = unsafe {
        transmute(addr_ptr)
    };

    let result = add_fn(1,2);
    println!("result: {}", result);
}
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: i32,
}

fn serialize(){
    println!("------------- serialize -----------");
    let person = Person {
        name: "lee".to_string(),
        age: 12,
    };
    let json = serde_json::to_string(&person).unwrap();
    println!("json: {}", json);

    let person1:Person = serde_json::from_str(&json).unwrap();
    println!("person: {:?}", person1);
}

#[derive(Debug)]
struct Human {
    name: String,
    age: i32,
}
fn dyn_create(){
    println!("------------- dynamic create type -----------");
    let size = size_of::<Human>();
    let ptr = unsafe {
        alloc(Layout::from_size_align(size, 1024).unwrap())
    };
    let hubman: &mut Human = unsafe { transmute(ptr)};

    hubman.name = "lee".to_string();
    hubman.age = 18;

    println!("hubman: {:?}", hubman);
}
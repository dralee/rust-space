/*
 * 同一workspace中的类库
 * 2024.04.01 by dralee
 */

//! # add one
//! 
//! support to add one by the specify number

use rand::{self, Rng};

/// ## add_one
/// add one to the number
pub fn add_one(x: i32)-> i32 {
    x + 1
}

/// ## add_rand
/// add a random number to the specify
pub fn add_rand(x: i32) -> i32 {
    x + rand::thread_rng().gen_range(1..=100)
}
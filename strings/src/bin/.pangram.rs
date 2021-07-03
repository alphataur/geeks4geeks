#[allow(dead_code)]
/*
 *https://www.geeksforgeeks.org/pangram-checking/
 * */

use std::collections::HashSet;
use std::iter::FromIterator;

fn is_pangram_imperative(ips: String) -> bool{
    let set: HashSet<u8> = HashSet::from_iter(ips.to_lowercase().bytes());
    for i in 97..=122{
        if !set.contains(&i){
            return false;
        }
    }
    return true;
}

fn is_pangram_imperative_naive(ips: String) -> bool{
    let mut match_set: [bool; 26] = [false; 26];
    for i in ips.bytes(){
        if i > 96 && i < 123{
            match_set[(i-97) as usize] = true;
        }
    }
    for i in &match_set{
        if !i{
            return false;
        }
    }
    return true
}
fn is_pangram(ips: String) -> bool{
    //functional implementation
    ('a'..='z').all(|e|{
        ips.contains(e)
    })
}
fn main(){
    let a = String::from("The quick brown fox jumps over the lazy dog");
    let res = is_pangram_imperative(a.to_lowercase());
    println!("{:?}", res);
}

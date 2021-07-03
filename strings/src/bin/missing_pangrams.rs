//https://www.geeksforgeeks.org/missing-characters-make-string-pangram/
use std::collections::HashSet;
use std::iter::FromIterator;
fn filter_alphabets(a: &String) -> Vec<char>{
    a.to_lowercase()
        .chars()
        .filter(|e|e.is_alphabetic())
        .collect::<Vec<char>>()
}
fn experi(a: String) -> String{
    //working solution but end result is not sorted
    let set: HashSet<char> = HashSet::from_iter('a'..'z');
    let ips: HashSet<char> = HashSet::from_iter(a.to_lowercase()
        .chars()
        .filter(|e|e.is_alphabetic()));
    (&set - &ips).iter().collect()
}
fn main(){
    let input = "welcome to geeksforgeeks".to_string();
    let res = filter_alphabets(&input);
    let alphabets: HashSet<char> = HashSet::from_iter('a'..'z');
    let ialphabets: HashSet<char> = HashSet::from_iter(res.into_iter());
    let mut missing = (&alphabets - &ialphabets).iter().cloned().collect::<Vec<char>>();
    missing.sort();
    let result: String = missing.iter().collect();
    println!("{:?}", result);
    //println!("{}", experi(input));
}

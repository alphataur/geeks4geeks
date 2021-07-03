//https://www.geeksforgeeks.org/missing-characters-make-string-pangram/
use std::collections::HashSet;
use std::iter::FromIterator;
fn filter_alphabets(a: String) -> Vec<char>{
    a.to_lowercase()
        .chars()
        .filter(|e|e.is_alphabetic())
        .collect::<Vec<char>>()
}
fn main(){
    let input = "welcome to geeksforgeeks".to_string();
    let res = filter_alphabets(input);
    let alphabets: HashSet<char> = HashSet::from_iter('a'..'z');
    let ialphabets: HashSet<char> = HashSet::from_iter(res.into_iter());
    let mut results = (&alphabets - &ialphabets).iter().cloned().collect::<Vec<char>>();
    results.sort();
    println!("{:?}", results);
}

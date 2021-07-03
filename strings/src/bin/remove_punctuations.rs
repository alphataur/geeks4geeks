use std::collections::HashSet;
use std::iter::FromIterator;
//https://www.geeksforgeeks.org/removing-punctuations-given-string/
fn main(){
    let punches: HashSet<char> = HashSet::from_iter("!\"#$%&'()*+,-./:;?@[\\]^_`{|}~ <>".chars());
    let ips = "%welcome' to @geeksforgeek<s";
    let res: String = ips
                .chars()
                .filter(|e|!punches.contains(e))
                .collect();
    println!("{:?}", res);
}

use std::collections::HashMap;
fn counter(a: &String) -> HashMap<char, i32>{
    let mut res: HashMap<char, i32> = HashMap::new();
    for chr in a.chars(){
        *res.entry(chr).or_insert(0) += 1;
    }
    res
}
fn solve(a: &String) -> String{
    let counts = counter(a);
    
    println!("{:?}", counts);
    String::from("all ok")
}
fn main(){
    let ips: String = "aaabb".to_string();
    let counts = solve(&ips);
    println!("{}", counts);
}

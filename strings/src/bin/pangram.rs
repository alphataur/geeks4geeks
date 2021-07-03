fn main(){
    //let mut set: HashMap<char, i32> = HashMap::new()
    let input = "The quick brown fox jumps over the lazy dog".to_string();
    println!("{:?}", input.chars()
             .filter(|e|e.is_alphabetic())
             .all(|c| matches!(c, 'a'..='z' | 'A'..='Z')));
}

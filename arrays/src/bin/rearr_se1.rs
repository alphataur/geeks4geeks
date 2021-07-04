use std::collections::HashSet;
use std::iter::FromIterator;
fn solve(a: Vec<i32>, d: i32) -> Vec<i32>{
    let set: HashSet<i32> = HashSet::from_iter(a.clone().into_iter());
    let mut res = Vec::new();
    for i in 0..d{
        if set.contains(&i){
            res.push(i);
        }
        else{
            res.push(-1);
        }
    }
    res
}

fn main(){
    let arr: Vec<i32> = vec![-1, -1, 6, 1, 9, 3, 2, -1, 4, -1];
    println!("{:?}", solve(arr, 10))
}

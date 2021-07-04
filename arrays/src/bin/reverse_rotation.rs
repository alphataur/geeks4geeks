use std::iter::Skip;
use std::iter::Take;
fn solve(a: &Vec<i32>, d: usize) -> Vec<i32>{
    let mut res: Vec<i32> = Vec::new();
    for &i in a.into_iter().skip(d){
        res.push(i);
    }
    for &i in a.into_iter().take(d){
        res.push(i);
    }
    res
}
fn main(){
    let arr: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    println!("{:?}", solve(&arr, 2usize));
}

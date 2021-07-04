//https://www.geeksforgeeks.org/array-rotation/
use std::iter::Skip;
use std::iter::Take;
fn solve(a: &Vec<i32>, d: usize) -> Vec<i32>{
   let mut result: Vec<i32> = Vec::new();
   for &i in a.into_iter().skip(d){
        result.push(i);
   }
   for &i in a.into_iter().take(d){
        result.push(i);
   }
   result
}
fn main(){
    let d: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    println!("{:?}", solve(&d, 2usize))
}

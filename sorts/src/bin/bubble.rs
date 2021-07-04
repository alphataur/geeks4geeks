fn bubble(vector: Vec<usize>) -> Vec<usize>{
    let mut res: Vec<usize> = vector.clone();
    let n = vector.len();
    for i in 1usize..n{
        for j in i..n-1{
            if res[i] > res[j]{
                let temp = res[i];
                res[i] = res[j];
                res[j] = temp;
            }
        }
    }
    res
}
fn binary_search(vector: Vec<usize>, needle: usize) -> Result<(), usize>{
    let mut haystack = vector.clone();
    let needle
}
fn main(){
    let mut vec: Vec<usize> = vec![4, 19, 70, 73, 49, 54, 98, 34, 39, 26];
    vec = bubble(vec);
    println!("{:?}", vec);
}

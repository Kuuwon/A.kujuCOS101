fn main() {
    let v = vec![1,2,3,4,5,6,7,8,9];
    let x = vec![5,6,7,8,9,10,11,12];

    for index in 0..8 {
        let sum = v[index] + x[index];
        println!("{:?}", sum);
    }
}

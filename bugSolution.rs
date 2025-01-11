fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of a raw pointer, we will use a safe way to modify the data
    v[0] = 10; 
    println!("{:?}", v);
}
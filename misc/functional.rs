fn main() {
    let v1: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let v2: Vec<u8> = v1.iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x.pow(2))
        .collect();

    println!("{:?}", v2);
}
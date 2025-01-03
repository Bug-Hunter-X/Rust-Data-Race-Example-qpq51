fn main() {
    let mut x = 5;
    let y = &mut x;
    let z = &mut x;
    *y = 6;
    *z = 7; //This will cause a data race because two mutable references point to the same memory location
    println!("{}", x);
}
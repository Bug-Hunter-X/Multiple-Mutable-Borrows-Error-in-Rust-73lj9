fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; 
    let z = &mut x; //this will cause error 
    *z = 15;
}
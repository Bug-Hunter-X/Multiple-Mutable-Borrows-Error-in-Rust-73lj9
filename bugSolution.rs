fn main() {
    let mut x = 5;
    { //this is a scope
        let y = &mut x;
        *y = 10;
    }
    let z = &mut x;
    *z = 15;
}
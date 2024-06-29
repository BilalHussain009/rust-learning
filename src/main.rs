fn main() {
    let x = 4;
    let x = x + 1;
    print!("x is :{x}");
    {
        let x = 2;
        print!("x is {x}")
    }
    let x=x+1;
    print!("x is {x}")
}

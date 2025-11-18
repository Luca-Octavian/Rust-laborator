fn add_overflow(x:u32,y:u32)-> u32
{
    let a:u32 = u32::MAX;
    if x > a-y{
        panic!("Overflow la adunare!");
    }
    x+y
}
fn multiply_overflow(x:u32,y:u32)-> u32
{
    let a:u32 = u32::MAX;
    if x > a/y{
        panic!("Overflow la inmultire!");
    }
    x*y
}

fn main() {
    let x:u32 = 1000000000;
    let y:u32 = 7;
    let a:u32 = add_overflow(x,y);
    let b:u32 = multiply_overflow(x,y);
    println!("{a}");
    println!("{b}");
}

#[derive(Debug)]
enum Errors {
    Overflow32,
}

fn add_overflow(x: u32, y: u32) -> Result<u32, Errors> {
    let a: u32 = u32::MAX;
    if x > a - y {
        Err(Errors::Overflow32)
    } else {
        Ok(x + y)
    }
}

fn multiply_overflow(x: u32, y: u32) -> Result<u32, Errors> {
    let a: u32 = u32::MAX;
    if y != 0 && x > a / y {
        Err(Errors::Overflow32)
    } else {
        Ok(x * y)
    }
}

fn overflow_checker(x: u32, y: u32) -> Result<(u32, u32), Errors> {
    let sum = add_overflow(x, y)?;
    let product = multiply_overflow(x, y)?;
    Ok((sum, product))
}

fn main() {
    match overflow_checker(10, 20) {
        Ok((sum, product)) => println!("{} {}", sum, product),
        Err(e) => println!("{:?}", e),
    }

    match overflow_checker(2_500_000_000, 2_500_000_000) {
        Ok((sum, product)) => println!("{} {}", sum, product),
        Err(e) => println!("{:?}", e),
    }
}

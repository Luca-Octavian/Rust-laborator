fn next_prime(x: u16) -> Option<u16>
{
    let mut y = x;
    y=y.saturating_add(1);

    while y <= 65534{
        let mut ok = true;
        let mut d:u16=2;

        while (d as u32) * (d as u32) <= y as u32 {
            if y.is_multiple_of(d) {
                ok = false;
                break;
            }
            d += 1;
        }

        if ok {
            return Some(y);
        }

        if y == 65535 {
            break;
        }
        y += 1;
    }

     None
}


fn main() {
    let mut x:u16 =2;
    while x<=65521{
    match next_prime(x){
        Some(value) => println!("{value}"),
        None => println!("None"),
        }
        x+=1;
    }
}

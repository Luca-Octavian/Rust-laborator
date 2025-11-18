fn main() {
    let mut x = 0;
    while x <= 100 {
        prime(x);
        x += 1;
    }

    let mut a = 0;
    let mut b = 0;
    while a <= 100 {
        while b <= 100 {
            coprime(a, b);
            b += 1;
        }
        a += 1;
        b = 0;
    }

    bottles();
}

fn prime(x: i32) {
    if x < 2 {
        return;
    }
    let mut d = 2;
    let mut ok = 1;
    while d * d <= x {
        if x % d == 0 {
            ok = 0;
        }
        d += 1;
    }
    if ok == 1 {
        println!("{x}");
    }
}

fn gcd(mut a: i32, mut b: i32) -> bool {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a == 1
}

fn coprime(a: i32, b: i32) {
    if gcd(a, b) {
        println!("{a},{b}");
    }
}

fn bottles() {
    let mut nr = 99;

    while nr > 1 {
        println!("{nr} bottles of beer on the wall, {nr} bottles of beer");
        nr -= 1;

        if nr > 1 {
            println!("Take one down and pass it around, {nr} bottles of beer on the wall");
        } else {
            println!("Take one down and pass it around, {nr} bottle of beer on the wall");
        }
    }

    println!("1 bottle of beer on the wall, 1 bottle of beer");
    println!("Take one down and pass it around, no more bottles of beer on the wall");

    println!("No more bottles of beer on the wall, no more bottles of beer");
    println!("Go to the store and buy some more, 99 bottles of beer on the wall");
}

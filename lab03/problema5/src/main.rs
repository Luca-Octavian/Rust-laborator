fn perfect_square_root(n: u32) -> Option<u32> {
    let root = (n as f32).sqrt() as u32;
    if root * root == n {
        Some(root)
    } else {
        None
    }
}

fn main() {
    let mut num = 1;

    while num < 100 {
        match perfect_square_root(num) {
            Some(root) => println!("{num},{root}"),
            None => println!("None"),
        }
        num += 1;
    }
}


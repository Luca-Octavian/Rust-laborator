#[derive(Debug)]
enum Errors {
    NotAscii,
    NotDigit,
    NotBase16,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, Errors> {
    if !c.is_ascii_alphabetic() {
        return Err(Errors::NotLetter);
    }
    Ok(c.to_ascii_uppercase())
}

fn to_lowercase(c: char) -> Result<char, Errors> {
    if !c.is_ascii_alphabetic() {
        return Err(Errors::NotLetter);
    }
    Ok(c.to_ascii_lowercase())
}

fn print_char(c: char) -> Result<char, Errors> {
    if !c.is_ascii_graphic() && !c.is_ascii_whitespace() {
        return Err(Errors::NotPrintable);
    }
    Ok(c)
}

fn char_to_number(c: char) -> Result<u8, Errors> {
    if !c.is_ascii() {
        return Err(Errors::NotAscii);
    }
    if !c.is_ascii_digit() {
        return Err(Errors::NotDigit);
    }
    Ok(c as u8 - b'0')
}

fn char_to_number_hex(c: char) -> Result<u8, Errors> {
    if !c.is_ascii() {
        return Err(Errors::NotAscii);
    }
    match c {
        '0'..='9' => Ok(c as u8 - b'0'),
        'A'..='F' => Ok(c as u8 - b'A' + 10),
        _ => Err(Errors::NotBase16),
    }
}

fn print_error(e: Errors) {
    match e {
        Errors::NotLetter => println!("Caracterul trimis nu este o literă"),
        Errors::NotBase16 => println!("Caracterul trimis nu poate fi convertit în B16"),
        Errors::NotAscii => println!("Caracterul trimis nu face parte din codul ASCII"),
        Errors::NotPrintable => println!("Caracterul trimis nu poate fi printat"),
        Errors::NotDigit => println!("Caracterul trimis nu este o cifră"),
    }
}
fn main() {
    let r1 = to_uppercase('a');
    match r1 {
        Ok(c) => println!("Success: {:?}", c),
        Err(e) => print_error(e),
    }
    let r2 = to_uppercase('1');
    match r2 {
        Ok(c) => println!("Success: {:?}", c),
        Err(e) => print_error(e),
    }
    let r3 = to_lowercase('A');
    match r3 {
        Ok(c) => println!("Success: {:?}", c),
        Err(e) => print_error(e),
    }
    let r4 = to_lowercase('!');
    match r4 {
        Ok(c) => println!("Success: {:?}", c),
        Err(e) => print_error(e),
    }
    let r5 = print_char('X');
    match r5 {
        Ok(c) => println!("Success: {:?}", c),
        Err(e) => print_error(e),
    }
    let r6 = print_char('\u{7f}');
    match r6 {
        Ok(c) => println!("Success: {:?}", c),
        Err(e) => print_error(e),
    }
    let r7 = char_to_number('5');
    match r7 {
        Ok(n) => println!("Success: {}", n),
        Err(e) => print_error(e),
    }
    let r8 = char_to_number('a');
    match r8 {
        Ok(n) => println!("Success: {}", n),
        Err(e) => print_error(e),
    }
    let r9 = char_to_number_hex('F');
    match r9 {
        Ok(n) => println!("Success: {}", n),
        Err(e) => print_error(e),
    }
    let r10 = char_to_number_hex('g');
    match r10 {
        Ok(n) => println!("Success: {}", n),
        Err(e) => print_error(e),
    }
}

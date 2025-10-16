#[derive(Debug)]

enum errors{
    not_ASCII
    not_DIGIT
    not_BASE16
    not_LETTER
    not_PRINTABLE
}

fn to_uppercase(c: char)-> Result<char,errors>{
    if c < 97 || c > 122{
        Err(errors::not_LETTER)
    }
    else{
        Ok(c as u8 - b'a'-b'A')
    }
}

fn to_lowercase(c: char)-> Result<char,errors>{
    if c < 65 || c > 90{
        Err(errors::not_LETTER)
    }
    else{
        Ok(c as u8 + b'a'-b'A')
    }
}

fn print_char(c: char)-> Result<char,errors>{
    if c > 255{
        Err(errors::not_PRINTABLE)
    }
    else{
        Ok(c)
    }
}

fn char_to_number(c: char)-> Result<char,errors>{
    if c < 48 || c > 57{
        Err(errors::not_DIGIT)
    }
    else{
        Ok(c as u8)
    }
    if c < 32 || c >  127{
        Err(errors::not_ASCII)
    }
    else{
        Ok(c as u8)
    }
}

fn char_to_number_hex(c: char)-> Result<char,errors>{
   if (c as u8) > 127 || (c as u8) < 32{
        Err(errors::not_ASCII)
   }
   else{
    match c{
        '0'..='9' => Ok(c as u8 - b'0'),
        'A'..='F' => Ok(c as u8 - b'A' + 10),
        _ => Err(Errors::Not_BASE16),
    }
   }
}

fn print_error(errors){
    match errors{
        errors::not_LETTER => println!("Caracterul trimis nu este o litera");
        errors::not_BASE16 => println!("Caracterul trimis nu poate fi convertit in B16");
        errors::not_ASCII => println!("Caracterul trimis nu face parte din codul ASCII");
        errors::not_PRINTABLE => println!("Caracterul trimis nu poate fi printat");
        errors::not_DIGIT => println!("Caracterul trimis nu este o cifra");
    }
}

fn main() {
    let r1:char = to_uppercase('a');
     if r1.is_err(){
        print_error(r1.unwrap_err());
    }
    else{
        println!("{:?}",r1.ok().unwrap());
    }
    let r2:char = to_uppercase('A');
     if r2.is_err(){
        print_error(r2.unwrap_err());
    }
    else{
        println!("{:?}",r2.ok().unwrap());
    }
    let r3:char = to_lowercase('A');
     if r3.is_err(){
        print_error(r3.unwrap_err());
    }
    else{
        println!("{:?}",r3.ok().unwrap());
    }
     let r4:char = to_lowercase('a');
     if r4.is_err(){
        print_error(r4.unwrap_err());
    }
    else{
        println!("{:?}",r4.ok().unwrap());
    }
     let r5:char = print_char('X');
     if r5.is_err(){
        print_error(r5.unwrap_err());
    }
    else{
        println!("{:?}",r5.ok().unwrap());
    }
     let r6:char = print_char('0');
     if r6.is_err(){
        print_error(r6.unwrap_err());
    }
    else{
        println!("{:?}",r6.ok().unwrap());
    }
    
}

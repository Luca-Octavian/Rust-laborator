fn main() {
    let mut s:String=String::new();
    s.push_str("Text exemplu pentru ROT13,!@#$%%^&*()");
    let mut encryptat:String=String::new();
    for c in s.chars(){
        if !c.is_ascii(){
            eprintln!("Caracterul {c} nu este ascii");
            return;
        }
        if (c as u8)>=65 && (c as u8)<=90{
            let byte_encriptat=(((c as u8)-b'A'+13)%26)+b'A';
            encryptat.push(byte_encriptat as char);
        }
        else if (c as u8)>=97 && (c as u8)<=122{
            let byte_encriptat=(((c as u8)-b'a'+13)%26)+b'a';
            encryptat.push(byte_encriptat as char);
        }
        else{
            encryptat.push(c);
        }
    }
    println!("Original: {s}");
    println!("ROT13: {encryptat}");
}

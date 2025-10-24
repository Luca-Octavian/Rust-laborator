use std::fs;

fn problema1() {
    let mut maxbiti: usize = 0;
    let mut maxcaractere: usize = 0;
    let mut linie_biti_max: String = String::new();
    let mut linie_caractere_max: String = String::new();

    if let Ok(s) = fs::read_to_string("fisier.txt") {
        for line in s.lines() {
            let biti = line.len();
            let caractere = line.chars().count();
            if maxbiti < biti {
                maxbiti = biti;
                linie_biti_max = line.to_string();
            }
            if maxcaractere < caractere {
                maxcaractere = caractere;
                linie_caractere_max = line.to_string();
            }
        }
    }

    println!("biti maximi:{maxbiti}");
    println!("{linie_biti_max}");
    println!("caractere maxime:{maxcaractere}");
    println!("{linie_caractere_max}");
}

fn problema2() {
    let mut s: String = String::new();
    s.push_str("Text exemplu pentru ROT13,!@#$%%^&*()");
    let mut encryptat: String = String::new();
    for c in s.chars() {
        if !c.is_ascii() {
            eprintln!("Caracterul {c} nu este ascii");
            return;
        }
        if (c as u8) >= 65 && (c as u8) <= 90 {
            let byte_encriptat = (((c as u8) - b'A' + 13) % 26) + b'A';
            encryptat.push(byte_encriptat as char);
        } else if (c as u8) >= 97 && (c as u8) <= 122 {
            let byte_encriptat = (((c as u8) - b'a' + 13) % 26) + b'a';
            encryptat.push(byte_encriptat as char);
        } else {
            encryptat.push(c);
        }
    }
    println!("Original: {s}");
    println!("ROT13: {encryptat}");
}

fn problema3() {
    if let Ok(s) = fs::read_to_string("fisier2.txt") {
        for line in s.lines() {
            let mut n = String::new();

            for word in line.split_whitespace() {
                let replaced_word = if word == "pt" || word == "ptr" {
                    "pentru"
                } else if word == "dl" {
                    "domnul"
                } else if word == "dna" {
                    "doamna"
                } else {
                    word
                };

                if !n.is_empty() {
                    n.push(' ');
                }
                n.push_str(replaced_word);
            }

            println!("{n}");
        }
    }
}

fn problema4() {
    if let Ok(s) = fs::read_to_string("fisierul_normal_e_plin_de_#.txt") {
        for linie in s.lines() {
            let linie = linie.trim();
            if !linie.starts_with('#') && !linie.is_empty() {
                let mut cuvinte = linie.split_whitespace();
                let primul_camp = cuvinte.next();
                let al_doilea_camp = cuvinte.next();
                if let (Some(primul), Some(al_doilea)) = (primul_camp, al_doilea_camp) {
                    println!("{primul},{al_doilea}");
                }
            }
        }
    }
}

fn main() {
    println!("Problema 1");
    problema1();

    println!("\nProblema 2");
    problema2();

    println!("\nProblema 3");
    problema3();

    println!("\nProblema 4");
    problema4();
}

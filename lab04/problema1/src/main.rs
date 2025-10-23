use std::fs;
fn main() {
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

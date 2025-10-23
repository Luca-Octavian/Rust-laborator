use std::fs;

fn main() {
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

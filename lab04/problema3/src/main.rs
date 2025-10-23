use std::fs;

fn main() {
    if let Ok(s) = fs::read_to_string("fisier.txt") {
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

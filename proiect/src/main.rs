use std::io;

const INALTIME: usize = 6;
const LATIME: usize = 7;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum Jucator {
    Primul = 1,
    AlDoilea = 2,
    Nimeni = 0,
}

impl Jucator {
    fn from_u8(i: u8) -> Jucator {
        match i {
            1 => Jucator::Primul,
            2 => Jucator::AlDoilea,
            _ => Jucator::Nimeni,
        }
    }
}

struct Connect4 {
    terminat: bool,
    jucator_actual: Jucator,
    castigator: Jucator,
    nr_miscari: u8,
    tabla: [[u8; LATIME]; INALTIME],
}

#[derive(Debug)]
enum Error {
    InafaraHartii,
    JocTerminat,
    ColoanaPlina,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ColoanaPlina => write!(f, "Coloana este plina"),
            Error::InafaraHartii => write!(f, "Valoarea coloanei trebuie sa fie intre 1 si 7"),
            Error::JocTerminat => write!(f, "Jocul este deja terminat"),
        }
    }
}

impl Connect4 {
    fn default() -> Connect4 {
        Connect4 {
            terminat: false,
            jucator_actual: Jucator::Primul,
            castigator: Jucator::Nimeni,
            nr_miscari: 0,
            tabla: [[0; LATIME]; INALTIME],
        }
    }

    fn verificare_castigator(&mut self) -> Jucator {
        if self.nr_miscari < 7 {
            return Jucator::Nimeni;
        }

        let directii = [(0, 1), (1, 0), (1, 1), (-1, 1)];

        for r in 0..INALTIME {
            for c in 0..LATIME {
                let val = self.tabla[r][c];
                if val == 0 {
                    continue;
                }

                for (dr, dc) in directii {
                    let mut consec = 1;

                    let mut rr = r as isize + dr;
                    let mut cc = c as isize + dc;

                    while rr >= 0 && rr < INALTIME as isize && cc >= 0 && cc < LATIME as isize {
                        if self.tabla[rr as usize][cc as usize] == val {
                            consec += 1;

                            if consec == 4 {
                                self.terminat = true;
                                self.castigator = Jucator::from_u8(val);
                                return self.castigator;
                            }
                        } else {
                            break;
                        }

                        rr += dr;
                        cc += dc;
                    }
                }
            }
        }

        if self.nr_miscari == (INALTIME * LATIME) as u8 {
            self.terminat = true;
        }

        Jucator::Nimeni
    }

    fn joaca_miscare(&mut self, coloana: usize) -> Result<(), Error> {
        if self.terminat {
            return Err(Error::JocTerminat);
        }

        if coloana >= LATIME {
            return Err(Error::InafaraHartii);
        }

        if let Some(rand) = (0..INALTIME).rev().find(|&r| self.tabla[r][coloana] == 0) {
            self.tabla[rand][coloana] = self.jucator_actual as u8;
            self.nr_miscari += 1;
        } else {
            return Err(Error::ColoanaPlina);
        }

        let castigator = self.verificare_castigator();
        if castigator != Jucator::Nimeni {
            self.castigator = castigator;
            self.terminat = true;
        } else if self.jucator_actual == Jucator::Primul {
            self.jucator_actual = Jucator::AlDoilea;
        } else {
            self.jucator_actual = Jucator::Primul;
        }
        Ok(())
    }

    fn elibereaza_ecran(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn afisare(&self) {
        self.elibereaza_ecran();
        println!("--------------------");
        println!(" CONNECT 4 (Mutarea {})", self.nr_miscari);
        println!("--------------------");

        for row in self.tabla {
            let row_str: String = row
                .iter()
                .map(|&cell| match cell {
                    1 => "🔴",
                    2 => "🟡",
                    _ => "⚫",
                })
                .collect::<Vec<&str>>()
                .join(" ");

            println!("{}", row_str);
        }

        println!("--------------------");

        if self.terminat {
            match self.castigator {
                Jucator::Primul => println!("🔴 Jucatorul 1 a castigat!"),
                Jucator::AlDoilea => println!("🟡 Jucatorul 2 a castigat!"),
                Jucator::Nimeni => println!("Egalitate!"),
            }
        }
    }

    fn afisare_eroare(&self, error: String) {
        self.afisare();
        println!("Eroare: {}", error);
    }
}

fn main() {
    let mut joc = Connect4::default();
    joc.afisare();

    loop {
        while !joc.terminat {
            println!("\n");

            match joc.jucator_actual {
                Jucator::Primul => println!("Jucator 1 (🔴)"),
                Jucator::AlDoilea => println!("Jucator 2 (🟡)"),
                Jucator::Nimeni => (),
            }

            println!("Alege o coloana (1 - 7):");

            let mut user_move = String::new();
            io::stdin().read_line(&mut user_move).expect("Failed");

            let user_move: usize = match user_move.trim().parse() {
                Ok(num) => {
                    if !(1..=7).contains(&num) {
                        joc.afisare_eroare(Error::InafaraHartii.to_string());
                        continue;
                    }
                    num
                }
                Err(err) => {
                    joc.afisare_eroare(err.to_string());
                    continue;
                }
            };

            match joc.joaca_miscare(user_move - 1) {
                Ok(_) => joc.afisare(),
                Err(err) => joc.afisare_eroare(err.to_string()),
            }
        }

        println!("Apasa R pentru restart, Q pentru iesire.");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "R" | "r" => {
                joc = Connect4::default();
                joc.afisare();
            }
            "Q" | "q" => break,
            _ => joc.afisare_eroare("Input invalid".to_string()),
        }
    }
}

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}, //important ca altfel doubled inputs
};
use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};

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
    celule_castigatoare: Vec<(usize, usize)>, //needed this for the green squares
    joaca_ai: bool,
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
            celule_castigatoare: Vec::new(),
            joaca_ai: false,
        }
    }

    fn verificare_castigator(&mut self) -> Jucator {
        if self.nr_miscari < 7 {
            return Jucator::Nimeni;
        }

        let directii = [(0, 1), (1, 0), (1, 1), (-1, 1)];
        self.celule_castigatoare.clear();

        for r in 0..INALTIME {
            for c in 0..LATIME {
                let val = self.tabla[r][c];
                if val == 0 {
                    continue;
                }

                for (dr, dc) in directii {
                    let mut consec = vec![(r, c)];
                    let mut rr = r as isize + dr;
                    let mut cc = c as isize + dc;

                    while rr >= 0 && rr < INALTIME as isize && cc >= 0 && cc < LATIME as isize {
                        if self.tabla[rr as usize][cc as usize] == val {
                            consec.push((rr as usize, cc as usize));
                            if consec.len() == 4 {
                                self.terminat = true;
                                self.castigator = Jucator::from_u8(val);
                                self.celule_castigatoare = consec;
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
    fn creier_ai(&self) -> usize {
        let mut tabla_preferinte = [0i32; LATIME];

        if let Some(col) = self.castig_posibil(Jucator::AlDoilea as u8) {
            return col; //verific daca pot castiga AI
        }

        if let Some(col) = self.castig_posibil(Jucator::Primul as u8) {
            return col; //verific daca poate castiga player
        }

        for c in 0..LATIME {
            if let Some(r) = self.urmatoarea_coloana_goala(c) {
                let mut tabla_copy = self.tabla;
                tabla_copy[r][c] = Jucator::AlDoilea as u8;

                let ai_score = self.evalueaza_pozitie(r, c, Jucator::AlDoilea as u8);

                let mut enemy_threat = 0;
                for enemy_col in 0..LATIME {
                    if let Some(enemy_row) = self.urmatoarea_coloana_goala(enemy_col) {
                        let threat =
                            self.evalueaza_pericol(enemy_row, enemy_col, Jucator::Primul as u8);
                        enemy_threat = enemy_threat.max(threat);
                    }
                }

                tabla_preferinte[c] = ai_score * 10 - enemy_threat * 8; //prioritize a win over defense

                if c == LATIME / 2 {
                    tabla_preferinte[c] += 15; //centre bias
                }

                if c == LATIME / 2 - 1 || c == LATIME / 2 + 1 {
                    tabla_preferinte[c] += 10; //center bias but also rows 3 and 5
                }
            } else {
                tabla_preferinte[c] = -1000; //out of bounds
            }
        }

        let mut max = 0;
        for i in 0..LATIME {
            if tabla_preferinte[i] > max {
                max = tabla_preferinte[i];
            }
        }
        max as usize
    }

    fn castig_posibil(&self, player: u8) -> Option<usize> {
        for c in 0..LATIME {
            if let Some(r) = self.urmatoarea_coloana_goala(c) {
                let mut tabla_copy = self.tabla;
                tabla_copy[r][c] = player; //simulez miscarea urmatoare

                let directii = [(0, 1), (1, 0), (1, 1), (-1, 1)];

                for (dr, dc) in directii {
                    let mut consec = 1;

                    let mut rr = r as isize + dr;//check ahead
                    let mut cc = c as isize + dc;
                    while rr >= 0 && rr < INALTIME as isize && cc >= 0 && cc < LATIME as isize {
                        if tabla_copy[rr as usize][cc as usize] == player {
                            consec += 1;
                            rr += dr;
                            cc += dc;
                        } else {
                            break;
                        }
                    }

                    rr = r as isize - dr;//check backwards
                    cc = c as isize - dc;
                    while rr >= 0 && rr < INALTIME as isize && cc >= 0 && cc < LATIME as isize {
                        if tabla_copy[rr as usize][cc as usize] == player {
                            consec += 1;
                            rr -= dr;
                            cc -= dc;
                        } else {
                            break;
                        }
                    }

                    if consec >= 4 {
                        return Some(c);
                    }
                }
            }
        }
        None
    }

    fn urmatoarea_coloana_goala(&self, col: usize) -> Option<usize> {
        (0..INALTIME).rev().find(|&r| self.tabla[r][col] == 0)
    }

    fn evalueaza_pozitie(&self, row: usize, col: usize, player: u8) -> i32 {
        //how close is AI to win
        let mut score = 0;
        let directii = [(0, 1), (1, 0), (1, 1), (-1, 1)];

        for (dr, dc) in directii {
            let mut line_score = 0;
            let mut empty_spaces = 0;

            for dir in [-1, 1] {
                let mut count = 0;
                let mut steps = 1;

                while steps <= 3 {
                    let rr = row as isize + dr * steps * dir;
                    let cc = col as isize + dc * steps * dir;

                    if rr >= 0 && rr < INALTIME as isize && cc >= 0 && cc < LATIME as isize {
                        let cell = self.tabla[rr as usize][cc as usize];

                        if cell == player {
                            count += 1;
                        } else if cell == 0 {
                            empty_spaces += 1;
                            break;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                    steps += 1;
                }

                line_score += match count {
                    3 => 100, //castig eminent
                    2 => 25,  //2 la rand e ok
                    1 => 5,   //1 slab
                    _ => 0,   //nimic
                };
            }

            score += line_score;

            if empty_spaces > 0 {
                score += empty_spaces * 3;
            }
        }

        score
    }

    fn evalueaza_pericol(&self, row: usize, col: usize, enemy: u8) -> i32 {
        //how close is enemy to win

        let mut threat_score = 0;
        let directii = [(0, 1), (1, 0), (1, 1), (-1, 1)];

        for (dr, dc) in directii {
            let mut enemy_count = 0;

            for dir in [-1, 1] {
                let mut steps = 1;
                while steps <= 3 {
                    let rr = row as isize + dr * steps * dir;
                    let cc = col as isize + dc * steps * dir;

                    if rr >= 0 && rr < INALTIME as isize && cc >= 0 && cc < LATIME as isize {
                        if self.tabla[rr as usize][cc as usize] == enemy {
                            enemy_count += 1;
                        } else if self.tabla[rr as usize][cc as usize] != 0 {
                            break;
                        }
                    } else {
                        break;
                    }
                    steps += 1;
                }
            }

            threat_score += match enemy_count {
                3 => 100, //ft periculos
                2 => 30,  //pericol pe aproape
                1 => 5,   //nimic basically
                _ => 0,   //nimic
            };
        }

        threat_score
    }
    fn mesaj_stare(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        if self.terminat {
            match self.castigator {
                Jucator::Primul => {
                    lines.push(Line::from(Span::styled(
                        "🟥🟥🟥 Jucătorul 1 a câștigat!",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    )));
                }
                Jucator::AlDoilea => {
                    if self.joaca_ai == false {
                        lines.push(Line::from(Span::styled(
                            "🟨🟨🟨 Jucătorul 2 a câștigat!",
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )));
                    } else {
                        lines.push(Line::from(Span::styled(
                            "🟨🟨🟨 A castigat calculatorul!",
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )));
                    }
                }
                Jucator::Nimeni => {
                    lines.push(Line::from(Span::styled(
                        "Egalitate!",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )));
                }
            }
            lines.push(Line::from(""));
        } else {
            lines.push(Line::from(Span::styled(
                format!("Mutarea {}", self.nr_miscari),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )));

            let jucator_text = match self.jucator_actual {
                Jucator::Primul => Span::styled(
                    "🟥🟥🟥 Jucătorul 1",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Jucator::AlDoilea => {
                    if self.joaca_ai == false {
                        Span::styled(
                            "🟨🟨🟨 Jucătorul 2",
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else {
                        Span::styled(
                            "🟨🟨🟨 Calculator",
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )
                    }
                }
                Jucator::Nimeni => Span::raw("nimeni"),
            };

            lines.push(Line::from(vec![
                Span::styled("Este rândul: ", Style::default().fg(Color::White)),
                jucator_text,
            ]));
            lines.push(Line::from(""));
        }

        lines
    }

    fn mesaj_eroare(&self, err: &Error) -> Vec<Line<'static>> {
        vec![
            Line::from(""),
            Line::from(Span::styled(
                format!("⚠ {}", err),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )),
        ]
    }
}

fn run(mut terminal: DefaultTerminal, mut joc: Connect4) -> Result<(), Box<dyn std::error::Error>> {
    let mut mesaj = Vec::new();

    loop {
        terminal.draw(|frame| render(frame, &joc, &mesaj))?;

        if event::poll(std::time::Duration::from_millis(50))?
            && let Event::Key(key) = event::read()?
        {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,

                KeyCode::Char('r') => {
                    joc = Connect4::default();
                    mesaj.clear();
                }

                KeyCode::Char(c) => {
                    if let Some(col) = c.to_digit(10) {
                        //fixed warning
                        if (1..=7).contains(&col) {
                            let col = (col - 1) as usize;

                            match joc.joaca_miscare(col) {
                                Ok(_) => mesaj.clear(),
                                Err(err) => mesaj = joc.mesaj_eroare(&err),
                            }
                        } else {
                            mesaj = vec![Line::from(Span::styled(
                                "⚠ Trebuie să apeși o cifră între 1 și 7!",
                                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            ))];
                        }
                    }
                }

                _ => {}
            }
        }
    }

    Ok(())
}

fn render(frame: &mut Frame, joc: &Connect4, mesaj: &[Line]) {
    let full = frame.area();

    let left_width = full.width * 2 / 5; //40/60 cuz any bigger the table box and it just fits even less
    let right_width = full.width - left_width;

    let left = Rect {
        x: full.x,
        y: full.y,
        width: left_width,
        height: full.height,
    };

    let right = Rect {
        x: full.x + left_width,
        y: full.y,
        width: right_width,
        height: full.height,
    };

    let mut rows = Vec::new();

    for r in 0..INALTIME {
        let mut cells = Vec::new();
        for c in 0..LATIME {
            let is_winning = joc.celule_castigatoare.contains(&(r, c));

            let piece = if is_winning {
                "🟩🟩🟩" //green for winning squares(I had no other ideas)
            } else {
                match joc.tabla[r][c] {
                    1 => "🟥🟥🟥",
                    2 => "🟨🟨🟨",
                    _ => "⬛⬛⬛",
                }
            };

            let cell_with_border = format!(
                "⬜⬜⬜⬜⬜\n⬜{}⬜\n⬜{}⬜\n⬜{}⬜\n⬜⬜⬜⬜⬜", //white borders because it looks bad without
                piece, piece, piece
            );

            cells.push(Cell::from(cell_with_border));
        }
        rows.push(Row::new(cells).height(5));
    }

    let table = Table::new(rows, vec![Constraint::Length(10); LATIME])
        .block(
            Block::default()
                .title(Span::styled(
                    " CONNECT 4 ",
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .column_spacing(0);

    frame.render_widget(table, left);

    let mut text_content = Vec::new();

    text_content.extend(joc.mesaj_stare());

    text_content.extend(mesaj.iter().cloned());

    text_content.push(Line::from(""));
    text_content.push(Line::from(Span::styled(
        "INSTRUCȚIUNI:",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    )));
    text_content.push(Line::from(""));
    text_content.push(Line::from("• Tastează 1-7 pentru a pune o piesă"));
    text_content.push(Line::from("  în coloana respectivă"));
    text_content.push(Line::from("• R: Restartează jocul"));
    text_content.push(Line::from("• Q sau ESC: Ieși din joc"));

    let text = Paragraph::new(Text::from(text_content))
        .block(
            Block::default()
                .title(Span::styled(
                    " INFORMAȚII ",
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .alignment(Alignment::Left);

    frame.render_widget(text, right);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //main returns result because my run needs to return a result and it's the thing thaat actually executes and renders the code.
    enable_raw_mode()?;

    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let terminal = ratatui::init();
    let joc = Connect4::default();

    let result = run(terminal, joc);

    ratatui::restore();
    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    result
}

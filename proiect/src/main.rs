use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Table, Row, Cell};
use ratatui::style::{Color, Style, Modifier};
use ratatui::text::{Span, Line, Text};
use ratatui::DefaultTerminal;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::{
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},//important ca altfel doubled inputs
    execute,
};

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
    celule_castigatoare: Vec<(usize, usize)>,//needed this for the green squares
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
                    lines.push(Line::from(Span::styled(
                        "🟨🟨🟨 Jucătorul 2 a câștigat!",
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    )));
                }
                Jucator::Nimeni => {
                    lines.push(Line::from(Span::styled("Egalitate!", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))));
                }
            }
            lines.push(Line::from(""));
        } else {
            lines.push(Line::from(Span::styled(
                format!("Mutarea {}", self.nr_miscari),
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            )));
            
            let jucator_text = match self.jucator_actual {
                Jucator::Primul => Span::styled("🟥🟥🟥 Jucătorul 1", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Jucator::AlDoilea => Span::styled("🟨🟨🟨 Jucătorul 2", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
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
            ))
        ]
    }
}

fn run(mut terminal: DefaultTerminal, mut joc: Connect4) -> Result<(), Box<dyn std::error::Error>> {
    let mut mesaj = Vec::new();

    loop {
        terminal.draw(|frame| render(frame, &joc, &mesaj))?;

        if event::poll(std::time::Duration::from_millis(50))?  
            && let Event::Key(key) = event::read()? {
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
                        if let Some(col) = c.to_digit(10) {//warning I can't be bothered to fix rn
                            if col >= 1 && col <= 7 {
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
    
    let left_width = full.width * 2 / 5;//40/60 cuz any bigger the table box and it just fits even less
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
                "⬜⬜⬜⬜⬜\n⬜{}⬜\n⬜{}⬜\n⬜{}⬜\n⬜⬜⬜⬜⬜",//white borders because it looks bad without
                piece, piece, piece
            );
            
            cells.push(Cell::from(cell_with_border));
        }
        rows.push(Row::new(cells).height(5));
    }

    let table = Table::new(rows, vec![Constraint::Length(10); LATIME])
        .block(Block::default()
            .title(Span::styled(" CONNECT 4 ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue)))
        .column_spacing(0);

    frame.render_widget(table, left);

    let mut text_content = Vec::new();
    
    text_content.extend(joc.mesaj_stare());
    
    text_content.extend(mesaj.iter().cloned());
    
    text_content.push(Line::from(""));
    text_content.push(Line::from(Span::styled(
        "INSTRUCȚIUNI:",
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
    )));
    text_content.push(Line::from(""));
    text_content.push(Line::from("• Tastează 1-7 pentru a pune o piesă"));
    text_content.push(Line::from("  în coloana respectivă"));
    text_content.push(Line::from("• R: Restartează jocul"));
    text_content.push(Line::from("• Q sau ESC: Ieși din joc"));
    
    let text = Paragraph::new(Text::from(text_content))
        .block(Block::default()
            .title(Span::styled(" INFORMAȚII ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue)))
        .alignment(Alignment::Left);

    frame.render_widget(text, right);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {//main returns result because my run needs to return a result and it's the thing thaat actually executes and renders the code.
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
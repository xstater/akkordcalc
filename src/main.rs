extern crate crossterm;

use crossterm::{
    QueueableCommand,
    ExecutableCommand,
    terminal,
    style,
    event::{
        self,
        Event,
        KeyCode
    },
    cursor,
};
use std::collections::HashMap;
use std::io::Write;

macro_rules! hash_map {
  { $($key:expr => $value:expr),+ } => {
    {
      let mut m = std::collections::HashMap::new();
      $(
        m.insert($key, $value);
      )+
      m
    }
 };
}

fn main() -> crossterm::Result<()> {
    let key_mapping = hash_map! {
        'a' => "aug",
        'b' => "supertonic",
        'B' => "vermindet supertonic",
        'c' => "cadanza",
        'd' => "dominant",
        'D' => "Dominant",
        'e' => "aeolian",

        'f' => "finale",
        'g' => "grand",
        'h' => "halbvermindet",
        'i' => "Ionian",

        'k' => "k64",
        'K' => "K64",
        'l' => "vermindet leading",
        'L' => "minor leading",
        'm' => "mediant",
        'M' => "Mediant",
        'n' => "nature",

        'p' => "pause",

        's' => "subdominant",
        'S' => "Subdominant",
        't' => "tonica",
        'T' => "Tonica",
        'u' => "submidiant",
        'U' => "Submidiant",
        'v' => "vermindet"

    };

    let mut key_seq = vec![];

    let mut stdout = std::io::stdout();

    // let mut size= terminal::size()?;

    stdout
        .execute(cursor::Hide)?
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(cursor::MoveTo(0,0))?
        .execute(style::Print("Akkordcalc"))?
        .execute(cursor::MoveToNextLine(1))?
        .execute(style::Print("Press some key to calculate,Press Esc to quit"))?;

    'mainloop : loop {
        let event = event::read()?;
        match event {
            Event::Key(key) => {
                match key.code{
                    KeyCode::Backspace => {
                        key_seq.pop();
                        stdout
                            .queue(cursor::MoveTo(0,3))?
                            .queue(terminal::Clear(terminal::ClearType::CurrentLine))?;
                        for code in &key_seq {
                            //never failed because the
                            stdout
                                .queue(style::Print(key_mapping.get(code).unwrap()))?
                                .queue(style::Print(" "))?;
                        }
                    }
                    KeyCode::Enter => {}
                    KeyCode::Left => {}
                    KeyCode::Right => {}
                    KeyCode::Up => {}
                    KeyCode::Down => {}
                    KeyCode::Home => {}
                    KeyCode::End => {}
                    KeyCode::PageUp => {}
                    KeyCode::PageDown => {}
                    KeyCode::Tab => {}
                    KeyCode::BackTab => {}
                    KeyCode::Delete => {}
                    KeyCode::Insert => {}
                    KeyCode::F(_) => {}
                    KeyCode::Char(code) => {
                        stdout
                            .queue(cursor::MoveTo(0,2))?
                            .queue(terminal::Clear(terminal::ClearType::CurrentLine))?;
                        if let Some(&desc) = key_mapping.get(&code) {
                            //show desc
                            stdout.queue(style::Print(desc))?;

                            //show key buffer
                            key_seq.push(code);
                            stdout
                                .queue(cursor::MoveTo(0,3))?
                                .queue(terminal::Clear(terminal::ClearType::CurrentLine))?;
                            for code in &key_seq {
                                //never failed because the
                                stdout
                                    .queue(style::Print(key_mapping.get(code).unwrap()))?
                                    .queue(style::Print(" "))?;
                            }
                        }else{
                            stdout.queue(style::Print("unknown key"))?;
                        }
                    }
                    KeyCode::Null => {}
                    KeyCode::Esc => {
                        break 'mainloop;
                    }
                }
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {
                // size = terminal::size()?;
            }
        }
        stdout.flush()?;
    }

    Ok(())
}

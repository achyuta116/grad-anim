use std::{
    error::Error,
    fs::read_to_string,
    io::stdout,
    time::Duration,
};

use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, Event},
    style::Print,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use rand::Rng;

#[derive(Debug)]
struct Rgb {
    r: i16,
    g: i16,
    b: i16,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Specify a start colour R,G,B
    #[clap(
        short = 's',
        long = "start",
        default_value = "255,0,255",
        value_delimiter = ',',
        use_value_delimiter = true
    )]
    foreground: Vec<i32>,

    /// Specify an end colour R,G,B
    #[clap(
        short = 'e',
        long = "end",
        default_value = "0,255,0",
        value_delimiter = ',',
        use_value_delimiter = true
    )]
    background: Vec<i32>,
}

fn calculate_progress(i: usize, j: usize, m: usize, n: usize) -> f64 {
    (i as f64).hypot(j as f64) / (m as f64).hypot(n as f64)
}

fn lerp(start: &Rgb, end: &Rgb, progress: f64) -> Rgb {
    let color = Rgb {
        r: (start.r as f64 + progress * (end.r - start.r) as f64) as i16,
        g: (start.g as f64 + progress * (end.g - start.g) as f64) as i16,
        b: (start.b as f64 + progress * (end.b - start.b) as f64) as i16,
    };
    color
}

fn print_colored_text(
    text: &str,
    start: &Rgb,
    end: &Rgb,
    progress: f64,
) -> Result<(), Box<dyn Error>> {
    let line_count = text.lines().count();
    let max_line_len = text.lines().map(|line| line.len()).max().unwrap_or(0);
    let mut text = text.lines().enumerate();
    let mut to_print_line = String::from("");
    while let Some((i, line)) = text.next() {
        for (j, ch) in line.chars().enumerate() {
            let progress =
                (progress / 100.0 + calculate_progress(i, j, line_count, max_line_len)) % 1.0;
            let color = lerp(&start, &end, progress);
            to_print_line.push_str(
                format!(
                    "\x1b[38;2;{};{};{}m{}\x1b[0m",
                    color.r as u8, color.g as u8, color.b as u8, ch
                )
                .as_str(),
            );
        }
        to_print_line.push('\n');
    }
    stdout().execute(Print(to_print_line))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let _args = Args::parse();
    let mut rng = rand::thread_rng();
    let text = read_to_string("input.txt").unwrap();
    let text = text.as_str();

    let mut progress = 0;
    let start = Rgb {
        r: rng.gen_range(0..=255),
        g: rng.gen_range(0..=255),
        b: rng.gen_range(0..=255),
    };

    let end = Rgb {
        r: rng.gen_range(0..=255),
        g: rng.gen_range(0..=255),
        b: rng.gen_range(0..=255),
    };

    stdout().execute(Hide)?;

    loop {
        stdout().execute(MoveTo(0, 0))?;

        print_colored_text(text, &start, &end, progress as f64)?;

        if poll(Duration::from_millis(200))? {
            match crossterm::event::read()? {
                Event::Key(_) => break,
                _ => (),
            }
        }
        progress += 1;
        progress %= 100;
        stdout().execute(Clear(ClearType::All))?;
    }

    stdout().execute(Show)?;
    Ok(())
}

use chrono::{DateTime, Utc};
use std::io::{stdin, stdout, BufRead, BufReader, Write};

const REALLOC_THRESHOLD: usize = 0xFFFF;

fn main() -> std::io::Result<()> {
    let format = if let Some(format) = std::env::args().nth(1) {
        format
    } else {
        std::env::var("TS_FORMAT").unwrap_or_else(|_| "%Y-%m-%d %H:%M:%S".to_string())
    };

    let mut stdin = BufReader::new(stdin().lock());
    let mut stdout = stdout().lock();

    let mut input = String::new();
    loop {
        let size = stdin.read_line(&mut input)?;
        if size == 0 {
            break;
        }
        let now: DateTime<Utc> = Utc::now();

        if stdout
            .write_fmt(format_args!("{} {}", now.format(&format), input))
            .is_err()
        {
            break;
        };
        input.clear();
        if input.capacity() > REALLOC_THRESHOLD {
            input = String::new();
        }
    }

    Ok(())
}

extern crate chrono;
use chrono::{DateTime, Utc};

fn main() -> std::io::Result<()> {
    let format = if let Some(format) = std::env::args().nth(1) {
        format
    } else {
        std::env::var("TS_FORMAT").unwrap_or_else(|_| "%Y-%m-%d %H:%M:%S".to_string())
    };

    loop {
        let mut input = String::new();
        let size = std::io::stdin().read_line(&mut input)?;
        if size == 0 {
            break;
        }
        let now: DateTime<Utc> = Utc::now();
        print!("{} {}", now.format(&format), input);
    }

    Ok(())
}

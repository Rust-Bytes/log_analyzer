// Rust Bytes Challenge Issue #93 Log Analyzer
/// Abdul
/// read the file line by line into a vector,
/// use the index of the vector  to get the timestamp.


/// Elias
/// use a stream and parse everything into a hasmap
///

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
struct Log{
    timestamp: DateTime<Utc>,
    hostname: String,
    process_id: String,
    message: String
}


fn split_on_nth_occurrence(s: &str, pattern: char, n: usize) -> Option<(&str, &str)> {
    s.match_indices(pattern)
        .nth(n - 1)
        .map(|(index, _)| s.split_at(index))
}
fn main() -> io::Result<()> {
    let file_path = Path::new("./Mac_2k.log");
    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        let split_line = split_on_nth_occurrence(&line, ':', 3);
        if let Some((left , message)) = split_line{
            let s = left.split_whitespace().collect::<Vec<_>>();
            // deserialize the timestamp into a chrono type.
        }


    }

    Ok(())
}
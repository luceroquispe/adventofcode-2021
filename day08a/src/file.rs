use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::common::EncodedSegments;

pub fn read_file_to_segments(path: &str) -> io::Result<Vec<EncodedSegments>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let segments = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                Some((
                    parts[0].split_whitespace().map(|s| s.to_string()).collect(),
                    parts[1].split_whitespace().map(|s| s.to_string()).collect(),
                ))
            } else {
                None
            }
        })
        .filter_map(|opt| {
            if let Some((strings, output)) = opt {
                Some(EncodedSegments { strings, output })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(segments)
}

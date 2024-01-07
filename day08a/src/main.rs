use day08a::file::read_file_to_segments;

fn main() {
    let encoded_segments = read_file_to_segments("data/data1.txt").unwrap();
    println!("{:?}", encoded_segments);
    let total_count = encoded_segments
        .iter()
        .flat_map(|segment| segment.output.iter())
        .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
        .count();

    println!("Total Count: {}", total_count);
}

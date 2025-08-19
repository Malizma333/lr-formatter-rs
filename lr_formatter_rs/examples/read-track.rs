use lr_formatter_rs::formats::trk;
use std::{fs::File, io::Read};

fn main() {
    // Read a track file
    let mut input_data = Vec::new();
    File::open("../samples/HAM.trk")
        .unwrap()
        .read_to_end(&mut input_data)
        .unwrap();
    let track = trk::read(input_data).unwrap();

    // Use the track
    println!(
        "{} blue lines, {} red lines, {} green lines",
        track.line_group().standard_lines().len(),
        track.line_group().acceleration_lines().len(),
        track.line_group().scenery_lines().len()
    );
}

use std::io::{stdin};

fn main() {
    let mut lines = stdin().lines();
    let cost_to_sow_one_square_of_lawn: f32 = lines.next().unwrap().unwrap().parse().unwrap();
    let num_lawns_to_sow: u8 = lines.next().unwrap().unwrap().parse().unwrap();

    let mut lawns = vec![(0f32, 0f32); num_lawns_to_sow as usize];

    for lawn in &mut lawns {
        let width_and_height_str = lines.next().unwrap().unwrap();
        let (width_str, height_str) = width_and_height_str.split_once(' ').unwrap();
        *lawn = (width_str.parse().unwrap(), height_str.parse().unwrap());
    }

    let cost_to_sow_all = lawns.iter().fold(0f32, |acc, (width, height)| {
        acc + (width * height * cost_to_sow_one_square_of_lawn)
    });
    println!("{cost_to_sow_all}");
}

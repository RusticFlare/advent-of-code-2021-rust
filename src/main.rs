mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn main() {
    println!("Day 1: Part 1: {}", day_1::part_1(day_1::input::INPUT)); // 1215
    println!("Day 1: Part 2: {}", day_1::part_2(day_1::input::INPUT)); // 1150
    println!("Day 2: Part 1: {}", day_2::part_1(day_2::input::INPUT)); // 1728414
    println!("Day 2: Part 2: {}", day_2::part_2(day_2::input::INPUT)); // 1765720035
    println!("Day 3: Part 1: {}", day_3::part_1(day_3::input::INPUT)); // 3923414
    println!("Day 3: Part 2: {}", day_3::part_2(day_3::input::INPUT)); // 5852595
    println!("Day 4: Part 1: {}", day_4::part_1(day_4::input::INPUT)); // 11774
    println!("Day 4: Part 2: {}", day_4::part_2(day_4::input::INPUT)); // 4495
    println!("Day 5: Part 1: {}", day_5::part_1(day_5::input::INPUT)); // 7468
    println!("Day 5: Part 2: {}", day_5::part_2(day_5::input::INPUT)); // 22364
}

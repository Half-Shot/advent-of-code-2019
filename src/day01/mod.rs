use conv::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn calculate_fuel_required(mass: i64) -> i64 {
    let intermediate = ((mass / 3) as f64).approx_as_by::<i64, RoundToZero>();
    (intermediate.unwrap() as i64) - 2
}

pub fn main() -> std::io::Result<()> {
    let file = File::open("./input/day01.txt")?;
    let buf_reader = BufReader::new(file);
    let mut fuel_required: i64 = 0;
    for line in buf_reader.lines() {
        let mass = line?.parse().unwrap();
        let mut fuel_for_mod = calculate_fuel_required(mass);
        let mut new_fuel = calculate_fuel_required(fuel_for_mod);
        while new_fuel > 0 {
            fuel_for_mod += new_fuel;
            new_fuel = calculate_fuel_required(new_fuel);
        }
        fuel_required += fuel_for_mod
    }

    println!("Fuel required: {}", fuel_required);

    Ok(())

    //let value = calculate_fuel_required(100756);
}
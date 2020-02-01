use std::error::Error;
use std::io;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let masses: Vec<i32> = read_stdin()?;
    println!("{}", total_fuel(&masses, fuel_required));
    println!("{}", total_fuel(&masses, fuel_required_recursive));
    Ok(())
}

fn read_stdin() -> Result<Vec<i32>, Box<dyn Error>> {
    let mut xs = Vec::new();
    for line in io::stdin().lock().lines() {
        xs.push(line?.parse()?);
    }
    Ok(xs)
}

fn fuel_required(mass: i32) -> i32 {
    mass / 3 - 2
}

fn fuel_required_recursive(mass: i32) -> i32 {
    let fuel = fuel_required(mass);
    if fuel < 0 {
        0
    } else {
        fuel + fuel_required_recursive(fuel)
    }
}

fn total_fuel(masses: &[i32], fuel_required: fn(i32) -> i32) -> i32 {
    let mut total = 0;
    for &mass in masses {
        total += fuel_required(mass);
    }
    total
}

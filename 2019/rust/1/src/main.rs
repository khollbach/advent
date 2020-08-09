use rayon::prelude::*;
use std::error::Error;
use std::io::{self, prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let masses: Vec<u32> = read_stdin()?;
    println!("{}", total_fuel(&masses, fuel_required));
    println!("{}", total_fuel(&masses, fuel_required_recursive));
    Ok(())
}

fn read_stdin() -> Result<Vec<u32>, Box<dyn Error>> {
    let mut xs = Vec::new();
    for line in io::stdin().lock().lines() {
        xs.push(line?.parse()?);
    }
    Ok(xs)
}

fn fuel_required(mass: u32) -> u32 {
    let fuel = mass as i32 / 3 - 2;
    if fuel < 0 {
        0
    } else {
        fuel as u32
    }
}

fn fuel_required_recursive(mut mass: u32) -> u32 {
    let mut fuel = 0;

    loop {
        let f = fuel_required(mass);
        if f == 0 {
            break;
        } else {
            fuel += f;
            mass = f;
        }
    }

    fuel
}

fn _fuel_required_recursive(mass: u32) -> u32 {
    // Rust doesn't have tail call optimization though (yet).
    fn helper(mass: u32, acc: u32) -> u32 {
        match fuel_required(mass) {
            0 => acc,
            f => helper(f, acc + f),
        }
    }

    helper(mass, 0)
}

fn total_fuel<F>(masses: &[u32], fuel_required: F) -> u32
where
    F: Fn(u32) -> u32 + Sync,
{
    masses.par_iter().map(|&m| fuel_required(m)).sum()
}

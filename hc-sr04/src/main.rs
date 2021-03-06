extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::env;
use std::time::{Duration, Instant};

struct Args {
    output: u64,
    input: u64,
}

fn calc_distance(output: u64, input: u64) -> sysfs_gpio::Result<()> {
    let output = Pin::new(output);
    let input = Pin::new(input);

    output.with_exported(|| {
        input.set_direction(Direction::In)?;
        output.set_direction(Direction::High)?;

        sleep(Duration::new(0, 15000));

        output.set_value(0)?;

        while input.get_value().unwrap() == 0 {
            continue;
        }

        let start = Instant::now();

        while input.get_value().unwrap() == 1 {
            continue;
        }

        let time = start.elapsed().subsec_nanos();

        println!("Distance: {:.2}cm", time as f64 * 0.00001715);

        Ok(())
    })
}

fn print_usage() {
    println!("Usage: cargo run <output> <input>");
}

fn get_args() -> Option<Args> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return None;
    }

    let output = if let Ok(output) = args[1].parse::<u64>() {
        output
    } else {
        return None;
    };
    let input = if let Ok(input) = args[2].parse::<u64>() {
        input
    } else {
        return None;
    };

    Some(Args { output, input })
}

fn main() {
    if let Some(args) = get_args() {
        let output = args.output;
        let input = args.input;

        loop {
            match calc_distance(output, input) {
                Ok(()) => sleep(Duration::from_secs(1)),
                Err(err) => println!("Something wrong when measure: {}", err),
            }
        }
    } else {
        print_usage();
    }
}

use std::any::Any;
use std::env;
use std::fs::{OpenOptions};
use std::io::{BufReader};
use std::io::prelude::*;
use std::str::FromStr;
use core::fmt::Debug;

use rand::distributions::WeightedIndex;
use rand_distr::{Distribution, Uniform};

trait Loot {
    fn print(&self);
    fn as_any(&self) -> &dyn Any;
}

impl Debug for dyn Loot {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct WeightedLoot {
    name: String,
    weight: u32,
}

impl Loot for WeightedLoot {
    fn print(&self) {
        println!("{:?}", self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Loot for String {
    fn print(&self) {
        println!("{:?}", self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// pick_random_weighted
/// 
/// # Arguments
/// 
/// * items: Vec<Box<dyn Loot>>
/// 
/// # Usage
/// 
/// Uses a Uniform distribution to randomly choose an item
/// Returns the name of the chosen item as a String
fn pick_random_uniform(items: Vec<Box<dyn Loot>>) -> String {
    let mut rng = rand::thread_rng();
    let result = &items[Uniform::from(0..items.len()).sample(&mut rng)];
    match result.as_any().downcast_ref::<String>() {
        Some(val) => String::from(val),
        None => panic!("Unable to get String from item Box."),
    }
}

/// pick_random_weighted
/// 
/// # Arguments
/// 
/// * items: Vec<Box<dyn Loot>>
/// 
/// # Usage
/// 
/// Populates a weighted table and chooses randomly
/// Returns the name of the chosen item as a String
fn pick_random_weighted(items: Vec<Box<dyn Loot>>) -> String {
    let mut rng = rand::thread_rng();

    let mut choices = Vec::new();
    let mut weights = Vec::new();

    for item in items {
        let item = match item.as_any().downcast_ref::<WeightedLoot>() {
            Some(item) => item,
            None => panic!("Unable to get WeightedLoot from Box."),
        };

        choices.push(String::from(&item.name));
        weights.push(item.weight);
    }

    let dist = WeightedIndex::new(&weights).unwrap();
    String::from(&choices[dist.sample(&mut rng)])
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Error! Pass loot table filename as argument.\n\tUsage: loot_table_selector loot_table.txt");
    }

    let mut options = OpenOptions::new();
    let file = options.read(true).open(&args[1]);
    if file.is_err() {
        panic!("Error while opening file.");
    }

    // Because this can hold weighted and unweighted loot, we must use Box<dyn Loot>
    let mut loot_table: Vec<Box<dyn Loot>> = Vec::new();
    let mut format = String::new();

    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        if line.is_err() {
            continue; // continue to try the next line rather than panic
        }

        let line = line.unwrap();

        if line.starts_with('#') {
            continue; // this line is a comment, so we ignore
        }

        // If format is not assigned, check first line for format (Weighted or Uniform)
        if format.is_empty() {
            format = String::from(&line);
            match format.as_str() {
                "Weighted" | "Uniform" => continue,
                _ => panic!("Error found in format. First non-commented line should be \"Weighted\" or \"Unweighted\"."),
            }
        }

        // If Weighted, split line by !! and populate loot_table accordingly
        if format == "Weighted" {
            let tokens: Vec<&str> = line.split("!!").collect();
            if tokens.len() != 2 {
                panic!("Error found when parsing line. Name and weight should be separated by \"!!\".\n\tEx: greatsword!!10");
            }
            // push a Boxed WeightedLoot struct into the vector
            loot_table.push(Box::new(
                WeightedLoot {
                    name: String::from(tokens[0]),
                    weight: FromStr::from_str(tokens[1]).unwrap(),
                }
            ));
        } 
        // If Uniform
        // TODO: Check if line contains \"!!\" and warn user that they may be using the wrong format
        else if format == "Uniform" {
            loot_table.push(Box::new(String::from(line)));
        }
    }

    // Uncomment to print loot_table
    // for loot in loot_table.iter() {
    //     loot.print();        
    // }

    // Based on the format, call the correct function to pick a random item
    let result = match format.as_str() {
        "Weighted" => pick_random_weighted(loot_table),
        "Uniform" => pick_random_uniform(loot_table),
        _ => panic!("Error when attempting to call function to pick random loot. Format is invalid."),
    };

    println!("Result: {:?}", result);
    
}

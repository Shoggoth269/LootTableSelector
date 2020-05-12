use std::any::Any;
use std::env;
use std::fs::{OpenOptions};
use std::io::{BufReader};
use std::io::prelude::*;
use std::str::FromStr;
use core::fmt::Debug;

use rand::distributions::WeightedIndex;
use rand_distr::{Distribution, Uniform};

extern crate iui;
use iui::prelude::*;
use iui::controls::{Label, Spinbox, Slider, Entry, MultilineEntry, LayoutGrid, 
    GridAlignment, GridExpand, HorizontalSeparator, Button};
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;


/// This struct will hold the values that multiple callbacks will need to access.
struct State {
    slider_val: i64,
    spinner_val: i64,
    entry_val: String,
    multi_val: String,
    loot_val: String,
}

trait Loot: {
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
fn pick_random_uniform(items: &Vec<Box<dyn Loot>>) -> String {
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
fn pick_random_weighted(items: &Vec<Box<dyn Loot>>) -> String {
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
    // let result = match format.as_str() {
    //     "Weighted" => pick_random_weighted(loot_table),
    //     "Uniform" => pick_random_uniform(loot_table),
    //     _ => panic!("Error when attempting to call function to pick random loot. Format is invalid."),
    // };

    // println!("Result: {:?}", result);
    
    /// Initialize the UI framework.
    let ui = UI::init().unwrap();

    // Initialize the state of the application.
    let state = Rc::new(RefCell::new(State { slider_val: 0, spinner_val: 0, entry_val: "".into(), multi_val: "".into(), loot_val: "".into() }));

    // Create the grid which we'll use to lay out controls
    let mut grid = LayoutGrid::new(&ui);
    grid.set_padded(&ui, true);

    // Set up the inputs for the application.
    // While it's not necessary to create a block for this, it makes the code a lot easier
    // to read; the indentation presents a visual cue informing the reader that these
    // statements are related.
    let (mut slider, mut spinner, mut entry, mut multi, mut button) = {
        // Numerical inputs
        let slider = Slider::new(&ui, 1, 100);
        let spinner = Spinbox::new(&ui, 1, 100);
        // Text inputs
        let entry = Entry::new(&ui);
        let multi = MultilineEntry::new(&ui);
        let button = Button::new(&ui, "Pick Loot");
        // Add everything into the grid
        grid.append(&ui, slider.clone(),
            // This is position (by slot) and size, expansion, and alignment.
            // In this case, row 0, col 0, 1 by 1, compress as much as possible,
            // and align to the fill.
            0, 0, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, spinner.clone(),
            // This one is at column zero, row 1.
            0, 1, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, HorizontalSeparator::new(&ui),
            0, 3, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, entry.clone(),
            0, 4, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, multi.clone(),
            // The multiline entry is at column 0, row 1, and expands vertically.
            0, 5, 1, 1, GridExpand::Vertical, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, button.clone(),
            0, 6, 1, 1, GridExpand::Vertical, GridAlignment::Fill, GridAlignment::Fill);
        (slider, spinner, entry, multi, button)
    };

    // Set up the outputs for the application. Organization is very similar to the
    // previous setup.
    let (add_label, sub_label, text_label, bigtext_label, random_item_label) = {
        let add_label = Label::new(&ui, "");
        let sub_label = Label::new(&ui, "");
        let text_label = Label::new(&ui, "");
        let bigtext_label = Label::new(&ui, "");
        let random_item_label = Label::new(&ui, "");
        grid.append(&ui, add_label.clone(), 
            1, 0, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, sub_label.clone(),
            1, 1, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        // We skip the #2 & 3 slots so that the text labels will align with their inputs.
        // This is important because the big text label can expand vertically.
        grid.append(&ui, text_label.clone(),
            1, 4, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, bigtext_label.clone(),
            1, 5, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, random_item_label.clone(),
            1, 6, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        (add_label, sub_label, text_label, bigtext_label, random_item_label)
    };

    // The window allows all constituent components to be displayed.
    let mut window = Window::new(&ui, "Loot Picker", 300, 150, WindowType::HasMenubar);
    window.set_child(&ui, grid);
    window.show(&ui);

    // These on_changed functions allow updating the application state when a
    // control changes its value.

    slider.on_changed(&ui, {
        let state = state.clone();
        move |val| { state.borrow_mut().slider_val = val; }
    });

    spinner.on_changed(&ui, {
        let state = state.clone();
        move |val| { state.borrow_mut().spinner_val = val; }
    });

    entry.on_changed(&ui, {
        let state = state.clone();
        move |val| { state.borrow_mut().entry_val = val; }
    });

    multi.on_changed(&ui, {
        let state = state.clone();
        move |val| { state.borrow_mut().multi_val = val; }
    });

    button.on_clicked(&ui, {
        let state = state.clone();
        move |_| { state.borrow_mut().loot_val = 
            match format.as_str() {
                "Weighted" => pick_random_weighted(&loot_table),
                "Uniform" => pick_random_uniform(&loot_table),
                _ => panic!("Error when attempting to call function to pick random loot. Format is invalid."),
            };
        }
    });

    // Rather than just invoking ui.run(), using EventLoop gives a lot more control
    // over the user interface event loop.
    // Here, the on_tick() callback is used to update the view against the state.
    let mut event_loop = ui.event_loop();
    event_loop.on_tick(&ui, {
        let ui = ui.clone();
        let mut add_label = add_label.clone();
        let mut sub_label = sub_label.clone();
        let mut text_label = text_label.clone();
        let mut bigtext_label = bigtext_label.clone();
        let mut random_item_label = bigtext_label.clone();
        move || {
            let state = state.borrow();

            // Update all the labels
            add_label.set_text(&ui, &format!("Added: {}", state.slider_val + state.spinner_val));
            sub_label.set_text(&ui, &format!("Subtracted: {}", state.slider_val - state.spinner_val));
            text_label.set_text(&ui, &format!("Text: {}", state.entry_val));
            bigtext_label.set_text(&ui, &format!("Multiline Text: {}", state.multi_val));
            random_item_label.set_text(&ui, &format!("Selected Item: {}", state.loot_val));
        }
    });
    event_loop.run(&ui);
}

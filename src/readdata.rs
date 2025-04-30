#![allow(dead_code)]
use csv;
use std::error::Error;

#[derive(Debug, Clone)]
//Reads a CSV and returns a dataframe in rust. 

pub enum ColumnVal { //an enum that captures possibilities of an element in CSV and matches it to a datatype
    One(String),
    Two(bool),
    Three(f64),
    Four(i64),
}

#[derive(Debug)]
pub struct DataFrame { // creates a struct that takes labels as a vector of strings and data as a vector of one of the enum filled vectors
    pub labels: Vec<String>,
    pub data: Vec<Vec<ColumnVal>>,
}

impl DataFrame {
    pub fn new() -> Self { //initialzes an empty dataframe with empty vectors of labels and data.
        DataFrame {
            labels: Vec::new(),
            data: Vec::new(),
        }
    }
    // reads a CSV taking an argument to a reference to a df , the path to the file, and the types of data to expect.
    pub fn read_csv(&mut self, path: &str, types: &Vec<u32>) -> Result<(), Box<dyn Error>> { 
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .flexible(true)
            .from_path(path)?;
        // reads a csv
        let mut first_row = true; // sets first row equal to true to direct data into label row

        for result in rdr.records() { 
            let r = result?;
            let mut row: Vec<ColumnVal> = vec![];

            if first_row {
                self.labels = r.iter().map(|s| s.to_string()).collect();
                first_row = false; // reads first row into label area and then marks as false to send rest of data to data struct
                continue;
            }

            for (i, elem) in r.iter().enumerate() {
                let val = match types[i] {
                    1 => ColumnVal::One(elem.to_string()),
                    2 => ColumnVal::Two(elem.parse::<bool>().unwrap()),
                    3 => ColumnVal::Three(elem.parse::<f64>().unwrap()),
                    4 => ColumnVal::Four(elem.parse::<i64>().unwrap()),
                    _ => panic!("Unknown type"),
                };
                row.push(val); // for each entry in each entry add the value to an empty initialized vector
            }

            self.data.push(row); // push the filled vector to the data
        }

        Ok(()) // return
    }
}// returns a data frame with different types of data and labels.

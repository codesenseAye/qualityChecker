// Check tab count
// Check column count

// Check comments above functions
// Check comments within functions

// Check typechecking of function parameters
// Check typechecking of variables within and outside of functions

// Tabulate score 
// Output score

// Output grade
// Comment on push with score and grade

use std::env;
use std::fs::{self, DirEntry};

use std::path::Path;
use std::io;

use std::marker::Sized;

use std::cell::{Cell, RefCell};
use regex::Regex;

fn process_lua_file(path: String, luaContents: String) -> String {
    let mut score = String::from(String::from("score at ") + path.as_str() + &String::from(": \n"));
    score = score.to_string() + "lines:" + &luaContents.lines().count().to_string();
    
    let mut max_tab = 0;
    let mut max_column = 0;
        
    let written_comments_re = Regex::new(r"((\-\-)+\s*\w+\s+)+(\s*local\s*)*\s+.*function\s+.*\)").unwrap();
    let all_functions = Regex::new(r"(\s*local\s*)*\s+.*function\s+.*\)").unwrap();

    let comments_count = written_comments_re.captures_iter(luaContents.as_str()).count();
    let functions_count = all_functions.captures_iter(luaContents.as_str()).count();

    let comments_to_function_diff = (comments_count as i32) - (functions_count as i32);
    score = score + "\ncomment to function ratio: " + &comments_to_function_diff.to_string();

    for line in luaContents.lines() {
        let line_str = line.chars().as_str();
        let column_count = line_str.chars().count();

        if column_count > max_column {
            max_column = column_count;
        }

        let tabbing_re = Regex::new(r"\t|[ ]{4,}").unwrap();
        let mat = tabbing_re.captures_iter(line_str);

        let tabs = mat.count();

        if tabs > max_tab {
            max_tab = tabs;
        }
    }

    score = score + "\nmaximum tabs reached: " + &max_tab.to_string();
    score = score + "\nmaximum column reached: " + &max_column.to_string();

    return score + "\n\n";
}

fn parse_lua_files(start_dir: &Path, tabulate: impl Fn(RefCell<String>) -> () + Copy) {
    let dir = fs::read_dir(start_dir);

    let dir = match dir {
        Ok(d) => d,
        Err(error) => panic!("Problem finding the directory: {:?}", error),
    };

    for entry in dir {
        let entry = match entry {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        let path = entry.path();

        if path.is_dir() {
            parse_lua_files(&path, tabulate);
        } else {
            let contents = fs::read_to_string(&path);

            let contents = match contents {
                Ok(text) => text,
                Err(error) => panic!("Failed to read file {:?}", error),
            };

            let path_str = String::from(path.to_string_lossy());
            if path_str.find(".lua").is_some() {
                tabulate(RefCell::new(process_lua_file(path_str, contents)));
            } else {
                println!("found a non lua file: {}", path_str);
            }
        }
    };
}

fn main() {
    let scores: RefCell<String> = RefCell::new(String::from(""));

    parse_lua_files(Path::new("../../../src"), |score: RefCell<String>| {
        let score = score.borrow();
        scores.replace(scores.take().clone() + &score.clone());
    });
    
    fs::write("score.text", scores.clone().get_mut().to_string());
}
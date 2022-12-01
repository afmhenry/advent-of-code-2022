use std::file;
use std::fs;
use std::fs::read;
use std::ops::Index;
use std::str::FromStr;

fn file_name_helper(stage: String) -> String {
    let file_name: Vec<&str> = file!().split("/").collect();
    let day: Vec<&str> = file_name.index(1).split(".").collect();
    let mut parsed_name: String = day.index(0).to_string();
    parsed_name.push_str(&stage);
    parsed_name.insert_str(0, "./src/io/");
    return parsed_name;
}

fn read_all<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}

pub fn task1() {
    println!("Hello, world!");
    let name: String = file_name_helper("-1-sim.txt".to_string());
    println!("run {}", name);
    let contents = read_all::<String>(&name);
    for entry in contents {
        println!("{}", entry.unwrap());
    }
}

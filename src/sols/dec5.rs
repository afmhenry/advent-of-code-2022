use crate::sols::helpers as h;
use std::str;

const DAY: &str = "dec5";

fn task(task: &str, data: &str, mode: &str) -> i32 {
    let contents = h::read_all_lines::<String>(DAY, data.clone(), mode);
    let mut result: i32 = 0;
    let mut state: bool = true;
    for entry in contents {
        match entry {
            Ok(val) => {
                if state {
                    let start_state = val
                        .as_bytes()
                        .chunks(4)
                        .map(str::from_utf8)
                        .collect::<Result<Vec<&str>, _>>()
                        .unwrap();

                    println!("{:?}", &start_state);
                    for ele in start_state {
                        println!(
                            "{:?}",
                            &ele.to_string().trim().replace("[", "").replace("]", "")
                        )
                    }
                }
                if val.is_empty() && state {
                    state = false;
                }
            }
            Err(_err) => {
                println!("{:?}", _err);
            }
        }
    }
    return result;
}

pub fn tasks() {
    //assert!(2 == task("1", "1", "sim"), "Task 1 Sim Failed");
    println!("Task 1 Live Result: {:?}", task("1", "1", "sim"));
    // Result is 485
    //assert!(4 == task("2", "1", "sim"), "Task 2 Sim Failed");
    //println!("Task 2 Live Result: {:?}", task("2", "1", "live"));
    // Result is 857
}

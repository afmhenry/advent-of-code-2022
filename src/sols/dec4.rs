use crate::sols::helpers as h;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

const DAY: &str = "dec4";

fn task(task: &str, data: &str, mode: &str) -> i32 {
    let contents = h::read_all_lines::<String>(DAY, data.clone(), mode);
    let mut result: i32 = 0;
    for entry in contents {
        match entry {
            Ok(val) => {
                let elf_pairs: Vec<&str> = val.split(",").collect();
                let elf1_range: Vec<&str> = elf_pairs[0].split("-").collect();
                let elf2_range: Vec<&str> = elf_pairs[1].split("-").collect();
                let e1_start: i32 = elf1_range[0].parse::<i32>().unwrap();
                let e1_end: i32 = elf1_range[1].parse::<i32>().unwrap();
                let e2_start: i32 = elf2_range[0].parse::<i32>().unwrap();
                let e2_end: i32 = elf2_range[1].parse::<i32>().unwrap();

                if task == "1" {
                    if e1_end - e1_start < e2_end - e2_start {
                        if e1_end <= e2_end && e1_start >= e2_start {
                            result += 1;
                        }
                    } else if e1_end - e1_start > e2_end - e2_start {
                        if e1_end >= e2_end && e1_start <= e2_start {
                            result += 1;
                        }
                    } else if e1_start == e2_start && e1_end == e2_end {
                        result += 1;
                    }
                } else {
                    if (e1_end < e2_start && e1_end < e2_end)
                        || e2_end < e1_start && e2_end < e1_end
                    {
                    } else {
                        result += 1;
                    }
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
    assert!(2 == task("1", "1", "sim"), "Task 1 Sim Failed");
    println!("Task 1 Live Result: {:?}", task("1", "1", "live"));
    // Result is 485
    assert!(4 == task("2", "1", "sim"), "Task 2 Sim Failed");
    println!("Task 2 Live Result: {:?}", task("2", "1", "live"));
    // Result is 2609
}

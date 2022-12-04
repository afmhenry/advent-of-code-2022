use crate::sols::helpers as h;

const DAY: &str = "dec5";

fn task(task: &str, data: &str, mode: &str) -> i32 {
    let contents = h::read_all_lines::<String>(DAY, data.clone(), mode);
    let mut result: i32 = 0;
    for entry in contents {
        match entry {
            Ok(val) => {
                println!("{:?}", val)
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
    // Result is 857
}

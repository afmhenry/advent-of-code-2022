use crate::sols::helpers as h;

const DAY: &str = "dec1";

fn task(task: &str, data: &str, mode: &str) -> i32 {
    let contents = h::read_all_lines::<i32>(DAY, data.clone(), mode);
    let mut max_calories: i32 = 0;
    let mut top_three_calories: [i32; 3] = [0, 0, 0];
    let mut curr_calories: i32 = 0;
    for entry in contents {
        match entry {
            Ok(val) => {
                curr_calories += val;
                if curr_calories > max_calories {
                    max_calories = curr_calories
                }
            }
            Err(_err) => {
                for i in 0..top_three_calories.len() {
                    if top_three_calories[i] < curr_calories {
                        top_three_calories[i] = curr_calories;
                        top_three_calories.sort();
                        break;
                    }
                }
                // Do something with the error if you want
                curr_calories = 0
            }
        }
    }
    if task == "2" {
        max_calories = 0;
        for top in top_three_calories {
            max_calories += top
        }
    }
    return max_calories;
}

pub fn tasks() {
    assert!(24000 == task("1", "1", "sim"), "Task 1 Sim Failed");
    println!("Task 1 Live Result: {:?}", task("1", "1", "live"));
    // Result is 69177
    assert!(45000 == task("2", "1", "sim"), "Task 2 Sim Failed");
    println!("Task 2 Live Result: {:?}", task("2", "1", "live"));
    // Result is 207456
}

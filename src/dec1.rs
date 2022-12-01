use std::file;
use std::ops::Index;
use std::str::FromStr;

fn file_name_helper(task: String, mode: String) -> String {
    let mut name: String = "-<TASK>-<MODE>.txt".to_string();
    name = name.replace("<MODE>", &mode).to_string();
    name = name.replace("<TASK>", &task).to_string();

    let file_name: Vec<&str> = file!().split("/").collect();
    let day: Vec<&str> = file_name.index(1).split(".").collect();
    let mut parsed_name: String = day.index(0).to_string();
    parsed_name.push_str(&name);
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

fn task(task: String, mode: String) -> i32 {
    let name: String = file_name_helper(task.clone(), mode);
    println!("Run task on {}", name);
    let contents = read_all::<i32>(&name);
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
    println!("{:?}", max_calories);

    return max_calories;
}

pub fn all_tasks() {
    assert!(
        24000 == task("1".to_string(), "sim".to_string()),
        "Sim Failed"
    );
    assert!(
        69177 == task("1".to_string(), "live".to_string()),
        "Live Failed"
    );
    assert!(
        45000 == task(String::from("2"), "sim".to_string()),
        "Sim Failed"
    );
    assert!(
        207456 == task("2".to_string(), "live".to_string()),
        "Live Failed"
    );
}

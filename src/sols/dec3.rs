use crate::sols::helpers as h;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

const DAY: &str = "dec3";

fn task(task: &str, data: &str, mode: &str) -> usize {
    let contents = h::read_all_lines::<String>(DAY, data.clone(), mode);
    let mut i: usize = 0;
    let mut group: [Vec<_>; 3] = Default::default();
    let mut prio_sum: usize = 0;
    for entry in contents {
        match entry {
            Ok(val) => {
                let ruk: Vec<_> = val.chars().collect();
                if task == "1" {
                    let comp1 = &mut &ruk[0..ruk.len() / 2];
                    let comp2 = &ruk[ruk.len() / 2..ruk.len()];
                    for item in comp1.iter() {
                        if comp2.contains(item) {
                            prio_sum += ALPHABET.chars().position(|x| x == *item).unwrap_or(0) + 1;
                            break;
                        }
                    }
                } else {
                    group[i] = ruk;
                    if i == 2 {
                        for gm_char in group[0].iter() {
                            if group[1].contains(&gm_char) && group[2].contains(&gm_char) {
                                prio_sum +=
                                    ALPHABET.chars().position(|x| x == *gm_char).unwrap_or(0) + 1;
                                break;
                            }
                        }
                        group = Default::default();
                        i = 0;
                    } else {
                        i += 1;
                    }
                }
            }
            Err(_err) => {
                println!("{:?}", _err);
            }
        }
    }
    println!("{:?}", prio_sum);
    return prio_sum;
}

pub fn tasks() {
    assert!(157 == task("1", "1", "sim"), "Task 1 Sim Failed");
    println!("Task 1 Live Result: {:?}", task("1", "1", "live"));
    // Result is 7727
    assert!(70 == task("2", "1", "sim"), "Task 2 Sim Failed");
    println!("Task 2 Live Result: {:?}", task("2", "1", "live"));
    // Result is 2609
}

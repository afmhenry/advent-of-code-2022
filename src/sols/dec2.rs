use crate::sols::helpers as h;
use std::ops::Index;

const DAY: &str = "dec2";

struct MoveMapping {
    a: &'static str,
    b: &'static str,
    c: &'static str,
}
struct RevMoveMapping {
    x: &'static str,
    y: &'static str,
    z: &'static str,
}
impl MoveMapping {
    fn get(&self, field_string: &str) -> &str {
        match field_string {
            "a" => &self.a,
            "b" => &self.b,
            "c" => &self.c,
            _ => &self.a,
        }
    }
}
impl RevMoveMapping {
    fn get(&self, field_string: &str) -> &str {
        match field_string {
            "x" => &self.x,
            "y" => &self.y,
            "z" => &self.z,
            _ => &self.x,
        }
    }
}

static WIN_MAPPING: MoveMapping = MoveMapping {
    a: "z",
    b: "x",
    c: "y",
};

static LOSE_MAPPING: MoveMapping = MoveMapping {
    a: "y",
    b: "z",
    c: "x",
};

static ENC_MAPPING: MoveMapping = MoveMapping {
    a: "x",
    b: "y",
    c: "z",
};
static REV_ENC_MAPPING: RevMoveMapping = RevMoveMapping {
    x: "a",
    y: "b",
    z: "c",
};

struct Points {
    x: i32,
    y: i32,
    z: i32,
}

impl Points {
    fn get(&self, field_string: &str) -> &i32 {
        match field_string {
            "x" | "a" => &self.x,
            "y" | "b" => &self.y,
            "z" | "c" => &self.z,
            _ => &self.y,
        }
    }
}

static MOVE_POINT_MAPPING: Points = Points { x: 1, y: 2, z: 3 };
static RESULT_POINT_MAPPING: Points = Points { x: 0, y: 3, z: 6 };

fn task(task: &str, data: &str, mode: &str) -> i32 {
    let contents = h::read_all_lines::<String>(DAY, data.clone(), mode);
    let mut my_score: i32 = 0;

    for entry in contents {
        match entry {
            Ok(val) => {
                //println!("{:?}", val);
                let moves: Vec<&str> = val.split(" ").collect();
                let elf_move = &moves.index(0).to_lowercase();
                let my_move = &moves.index(1).to_lowercase();

                if task == "1" {
                    my_score += MOVE_POINT_MAPPING.get(my_move);

                    if my_move == ENC_MAPPING.get(elf_move) {
                        my_score += 3
                    } else if elf_move != my_move && WIN_MAPPING.get(elf_move) != my_move {
                        my_score += 6
                    } else {
                        my_score += 0
                    }
                } else {
                    my_score += RESULT_POINT_MAPPING.get(my_move);
                    if my_move == "x" {
                        my_score +=
                            MOVE_POINT_MAPPING.get(REV_ENC_MAPPING.get(WIN_MAPPING.get(elf_move)))
                    } else if my_move == "z" {
                        my_score +=
                            MOVE_POINT_MAPPING.get(REV_ENC_MAPPING.get(LOSE_MAPPING.get(elf_move)));
                    } else {
                        my_score += MOVE_POINT_MAPPING.get(elf_move);
                    }
                }

            }
            Err(_err) => {
                println!("{:?}", _err);
            }
        }
    }

    return my_score;
}

pub fn tasks() {
    assert!(15 == task("1", "1", "sim"), "Task 1 Sim Failed");
    println!("Task 1 Live Result: {:?}", task("1", "1", "live"));
    // Result is 8890
    assert!(12 == task("2", "1", "sim"), "Task 2 Sim Failed");
    println!("Task 2 Live Result: {:?}", task("2", "1", "live"));
    // Result is 10238
}

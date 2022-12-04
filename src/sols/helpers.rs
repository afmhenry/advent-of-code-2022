use std::str::FromStr;

fn io_name(day: &str, data: &str, mode: &str) -> String {
    let mut name: String = "./src/io/<DAY>-<DATA>-<MODE>.txt".to_string();
    name = name.replace("<DAY>", &day);
    name = name.replace("<DATA>", &data);
    name = name.replace("<MODE>", &mode);
    return name.to_string();
}

pub fn read_all_lines<T: FromStr>(
    day: &str,
    data: &str,
    mode: &str,
) -> Vec<Result<T, <T as FromStr>::Err>> {
    let file_name: String = io_name(day, data, mode);
    println!("Run task on {}", file_name);
    std::fs::read_to_string(file_name)
        .expect("File not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}

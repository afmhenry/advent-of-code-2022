fn task(task: &str, data: &str, mode: &str) -> i32 {
    let contents = h::read_all_lines::<String>(DAY, data.clone(), mode);
    println!("{:?}", contents);
    for entry in contents {
        match entry {
            Ok(val) => {
                println!("{:?}", val);
            }
            Err(_err) => {
                println!("{:?}", _err);
            }
        }
    }
    if task == "2" {}
    return 0;
}

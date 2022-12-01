mod dec1;

fn main() {
    assert!(
        24000 == dec1::task("1".to_string(), "sim".to_string()),
        "Sim Failed"
    );
    assert!(
        69177 == dec1::task("1".to_string(), "live".to_string()),
        "Live Failed"
    );
    assert!(
        45000 == dec1::task("2".to_string(), "sim".to_string()),
        "Sim Failed"
    );
    assert!(
        207456 == dec1::task("2".to_string(), "live".to_string()),
        "Live Failed"
    );
}

mod dec1;
use dec1::task1;

fn main() {
    assert!(24000 == task1("sim".to_string()), "sim failed");
    assert!(69177 == task1("live".to_string()), "live failed");
}

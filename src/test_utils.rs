use std::env;

pub const TEST_FLAG: &str = "RECISION_INTEGRATION_TEST";

pub fn test_flag_is_set() -> bool {
    if env::var(TEST_FLAG).is_err() {
        return false
    }

    if env::var(TEST_FLAG).unwrap().trim() != "1" {
        return false
    }

    return true
}

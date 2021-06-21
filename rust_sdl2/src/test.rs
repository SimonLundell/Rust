// Defined testcase. Can run all testcases with "cargo test" from command line
#[test]
fn cool_test() {
    assert_eq!(1,1);
}

pub fn other_function() -> i32 { // explicitly made pub, else private and wont work to import
    return -23;
}
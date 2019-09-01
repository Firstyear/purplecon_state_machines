extern {
    fn test_microwave() -> isize;
}

fn run_test() -> isize {
    unsafe {
        test_microwave()
    }
}

#[cfg(test)]
mod tests {
    use crate::run_test;

    #[test]
    fn it_works() {
        let x = run_test();
        assert_eq!(x, 1);
    }
}

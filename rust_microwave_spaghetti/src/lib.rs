struct Microwave {
    door_open: bool,
    // This is an excellent example of why you always use
    // positive langage in booleans, rather than negatives :)
    magnetron_disabled: bool,
}

impl Microwave {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

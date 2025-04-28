pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod snails {
    mod good_snails {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod bad_snails {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
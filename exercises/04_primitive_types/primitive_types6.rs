fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);
        //             ^  ^  ^
        //           .0 .1 .2  <- These are the tuple indices

        // Access the second element using tuple indexing
        let second = numbers.1;
        //                   ^ Index 1 gets the second element (value: 2)

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}

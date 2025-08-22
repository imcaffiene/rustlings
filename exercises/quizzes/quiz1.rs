// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
// fn calculate_price_of_apples(???) -> ??? { ??? }

fn calculate_price_of_apples(quantity: i32) -> i32 {
    if quantity > 40 {
        quantity * 1
    } else {
        quantity * 2
    }
}

fn main() {
    println!("====== Apple price calculator =========");

    let test_quantities = [41, 20, 30, 40, 35, 100, 65];

    for qnty in test_quantities {
        let price = calculate_price_of_apples(qnty);
        let rate = if qnty > 40 { 1 } else { 2 };

        println!(
            "{} apples = {} rustbucks (@ {} rustbucks per apple)",
            qnty, price, rate
        )
    }
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}

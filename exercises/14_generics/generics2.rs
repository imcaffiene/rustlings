// Generic wrapper that can store any type T
struct Wrapper<T> {
    value: T,
}

// Generic implementation block - note the impl<T> syntax
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // You can optionally experiment here.
    let wrapper_u32 = Wrapper::new(42u32);
    let wrapper_str = Wrapper::new("Hello");

    println!("u32 wrapper: {}", wrapper_u32.value);
    println!("str wrapper: {}", wrapper_str.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}

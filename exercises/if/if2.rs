// if2.rs
//
// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
//
// Execute `rustlings hint if2` or use the `hint` watch subcommand for a hint.
// I AM NOT DONE

// I AM NOT DONE

pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz" {
        "bar"
    } else {
        "baz"
    }
}
// I AM NOT DONE

// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;
// I AM NOT DONE

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }
// I AM NOT DONE

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }
// I AM NOT DONE

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}

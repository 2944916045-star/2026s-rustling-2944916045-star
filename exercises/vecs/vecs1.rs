// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.
// I AM NOT DONE

// I AM NOT DONE

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v = vec![10, 20, 30, 40];// TODO: declare your vector here with the macro for vectors
// I AM NOT DONE

    (a, v)
}
// I AM NOT DONE

#[cfg(test)]
mod tests {
    use super::*;
// I AM NOT DONE

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}

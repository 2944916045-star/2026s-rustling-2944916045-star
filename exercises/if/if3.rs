// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.
// I AM NOT DONE

// I AM NOT DONE

pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        0
    };
// I AM NOT DONE

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };
// I AM NOT DONE

    habitat
}
// I AM NOT DONE

#[cfg(test)]
mod tests {
    use super::*;
// I AM NOT DONE

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }
// I AM NOT DONE

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }
// I AM NOT DONE

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }
// I AM NOT DONE

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}

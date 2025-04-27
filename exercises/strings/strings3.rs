// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(inp: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut a = inp;
    String::from(a.trim())
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    input.to_string() + &String::from(" world!")
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let mut index = 0;
    let mut count = 0;
    for i in input.chars() {
        index += 1;
    	if i == 'c' || i == 'a' || i == 'r' || i == 's' {
    		count += 1;
    	}
    	else {
    		count = 0;
    	}
    	if count == 4 {
    		break;
    	}
    }
    if count == input.chars().count() - 4 && count == 4{
    	input[0..index-4].to_string() + &String::from("balloons")
    }
    else if count == 4{
    	input[0..index-4].to_string() + &String::from("balloons") + &input[index..input.chars().count()].to_string()
    }
    else {
    	input.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}

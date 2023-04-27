fn is_lowercase_letter(c:&char) -> bool{
    'a' <= *c && *c <= 'z'
}

// skips zero or more tokens accepted by the closure f
// returns the amount of chars accepted
fn zero_or_more(f:fn(char)->bool, input:&[char])->usize{
    input
        .iter()
        .take_while(|c| f(**c))
        .fold(0, |acc, _| acc + 1)
}

fn detect(i:&str)->bool{
    let input = i.to_string().chars()
        .collect::<Vec<char>>();

    let input_length = input.len();

    if input_length < 3{
        return false;
    }

    let mut position = 0;
    if input[position] != '('{
        return false;
    }
    position += 1;

    position += zero_or_more(|c| c == ' ', &input[position..]);

    if position >= input_length{
        return false;
    }

    position += zero_or_more(|c| is_lowercase_letter(&c), &input[position..]);

    if position >= input_length{
        return false;
    }

    position += zero_or_more(|c| c == ' ', &input[position..]);

    if position >= input_length{
        return false;
    }

    if input[position] != ','{
        return false;
    }
    position += 1;


    position += zero_or_more(|c| c == ' ', &input[position..]);

    if position >= input_length{
        return false;
    }

    position += zero_or_more(|c| is_lowercase_letter(&c), &input[position..]);

    if position >= input_length{
        return false;
    }

    position += zero_or_more(|c| c == ' ', &input[position..]);

    return position == input_length -1 && *input.last().unwrap() == ')';
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(detect("(    asviaisdj       ,       ijoasdjioasdj     )"));
        assert!(detect("(a,b)"));
        assert!(detect("(,)"));
        assert!(detect("(a,)"));
        assert!(detect("(,a)"));
        assert!(!detect("(,"));
        assert!(!detect(","));
        assert!(!detect(""));
    }
}

fn main(){}
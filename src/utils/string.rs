pub fn shorten(input: &String, max_len: usize) -> String{
    if input.len() > max_len {
        let diff = input.len() - max_len;
        let to_remove = match diff {
            x if x > 3 => x,
            x if x <= 0 => 0,
            _ => 3
        };

        let head = (input.len() - to_remove) / 2;
        let tail = input.len()  - head - 3;
        let res = format!("{}...{}", &input[..head], &input[tail..]);

        return res;
    }

    return String::from(input);
}
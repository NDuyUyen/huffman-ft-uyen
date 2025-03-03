use std::num::ParseIntError;

pub fn vec_bool_to_string(bits: &Vec<bool>, form: usize) -> String {
    let mut result = String::new();
    let mut idx = 0;
    let mut base_2_exp = vec![0u8; form];

    if form > 0 {
        base_2_exp[form - 1] = 1;

        for i in 1..form {
            base_2_exp[form - i - 1] = base_2_exp[form - i] * 2;
        }
    }

    while bits.len() - idx >= form {
        let mut dec: u8 = 0;
        for e in 0..form {
            if bits[idx] {
                dec += base_2_exp[e];
            }
            idx += 1;
        }
        result = result + &(dec as char).to_string();
    }
    result
}

pub fn string_to_vec_bool(input: &str, size: usize) -> Vec<bool> {
    let mut result: Vec<bool> = vec![];
    let chars_vec: Vec<char> = input.chars().collect();

    chars_vec.into_iter().for_each(|c| {
        let ascii_dec = c as usize;
        let mut vec = usize_to_vec_bool(ascii_dec, size);

        result.append(&mut vec);
    });

    result
}

pub fn str_to_usize(input: &str) -> Result<usize, ParseIntError> {
    match input.parse::<usize>() {
        Ok(num) => Ok(num),
        Err(e) => Err(e),
    }
}

fn usize_to_vec_bool(mut num: usize, size: usize) -> Vec<bool> {
    let mut result: Vec<bool> = vec![];

    while num >= 2 {
        result.push(num % 2 == 1);
        num = num / 2 as usize;
    }

    result.push(num != 0);
    result.resize(size, false);
    result.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_vec_bool_to_string() {
        let t = true;
        let f = false;
        let bits = [
            t, f, f, t, f, f, f, t, t, t, f, t, f, t, t, t, f, f, t, t, f, t, t, f, f, t, t, f, t,
            t, f, t, t, f, t, t, t, f, f, f, f, t, t, t, f, t, t, t, f,
        ];
        let result = vec_bool_to_string(&bits.to_vec(), 7);
        assert_eq!(result, "Huffman");

        // Add more bits
        let bits = [
            t, f, f, t, f, f, f, t, t, t, f, t, f, t, t, t, f, f, t, t, f, t, t, f, f, t, t, f, t,
            t, f, t, t, f, t, t, t, f, f, f, f, t, t, t, f, t, t, t, f, f, f,
        ];
        let result = vec_bool_to_string(&bits.to_vec(), 7);
        assert_eq!(result, "Huffman");
    }

    #[test]
    fn test_usize_to_vec_bool() {
        assert_eq!(usize_to_vec_bool(0, 1), vec![false]);
        assert_eq!(usize_to_vec_bool(0, 2), vec![false, false]);
        assert_eq!(usize_to_vec_bool(1, 1), vec![true]);
        assert_eq!(usize_to_vec_bool(1, 3), vec![false, false, true]);
        assert_eq!(usize_to_vec_bool(2, 2), vec![true, false]);
        assert_eq!(usize_to_vec_bool(3, 2), vec![true, true]);
        assert_eq!(usize_to_vec_bool(4, 3), vec![true, false, false]);
        assert_eq!(
            usize_to_vec_bool(132, 8),
            vec![true, false, false, false, false, true, false, false]
        );
    }

    #[test]
    fn test_string_to_vec_bool() {
        let t = true;
        let f = false;
        let expect = [
            t, f, f, t, f, f, f, t, t, t, f, t, f, t, t, t, f, f, t, t, f, t, t, f, f, t, t, f, t,
            t, f, t, t, f, t, t, t, f, f, f, f, t, t, t, f, t, t, t, f,
        ];

        assert_eq!(string_to_vec_bool("Huffman", 7), expect);
    }
}

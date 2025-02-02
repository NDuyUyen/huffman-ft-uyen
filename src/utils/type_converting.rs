pub fn vec_bool_to_string(bits: &Vec<bool>, form: usize) -> String {
    let mut result = String::new();
    let mut idx = 0;
    let mut base_2_exp = vec![0u8; form];

    for i in 0..form {
        base_2_exp[form - i - 1] = 2u8.pow(i as u32);
    }

    while bits.len() - idx >= form {
        let mut dec: u8 = 0;
        for e in 0..form {
            if bits[idx] {
                dec += base_2_exp[e];
            }
            idx += 1;
        }
        result = result + &format!("{}", dec as char);
    }
    result
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
}

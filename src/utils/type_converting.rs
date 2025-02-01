use std::process::id;

const BASE_2_EXP: &'static [u8] = &[64, 32, 16, 8, 4, 2, 1];
const FORM_7: usize = 7;

pub fn vec_bool_to_string(bits: Vec<bool>) -> String {
    let mut result = String::new();
    let mut idx = 0;

    while bits.len() - idx >= FORM_7 {
        let mut dec: u8 = 0;
        for e in 0..FORM_7 {
            if bits[idx] {
                dec += BASE_2_EXP[e];
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
        let result = vec_bool_to_string(bits.to_vec());
        assert_eq!(result, "Huffman");

        // Add more bits
        let bits = [
            t, f, f, t, f, f, f, t, t, t, f, t, f, t, t, t, f, f, t, t, f, t, t, f, f, t, t, f, t,
            t, f, t, t, f, t, t, t, f, f, f, f, t, t, t, f, t, t, t, f, f, f,
        ];
        let result = vec_bool_to_string(bits.to_vec());
        assert_eq!(result, "Huffman");
    }
}

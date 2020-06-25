#[cfg(test)]
mod tests {
    const HOBBES_PLAIN: &str = "Man is distinguished, not only by his reason, but by \
    this singular passion from other animals, which is a lust of the mind, that \
    by a perseverance of delight in the continued and indefatigable generation \
    of knowledge, exceeds the short vehemence of any carnal pleasure.";

    const HOBBES_ENCODED: &str = r#"<~9jqo^BlbD-BleB1DJ+*+F(f,q/0JhKF<GL>Cj@.4Gp$d7F!,L7@<6@)/0JDEF<G%<+EV:2F!,O<DJ+*.@<*K0@<6L(Df-\0Ec5e;DffZ(EZee.Bl.9pF"AGXBPCsi+DGm>@3BB/F*&OCAfu2/AKYi(DIb:@FD,*)+C]U=@3BN#EcYf8ATD3s@q?d$AftVqCh[NqF<G:8+EV:.+Cf>-FD5W8ARlolDIal(DId<j@<?3r@:F%a+D58'ATD4$Bl@l3De:,-DJs`8ARoFb/0JMK@qB4^F!,R<AKZ&-DfTqBG%G>uD.RTpAKYo'+CT/5+Cei#DII?(E,9)oF*2M7/c~>"#;

    #[test]
    fn simple_z() {
        assert_eq!(crate::encode(&[0, 0, 0, 0]), "<~z~>".to_string());
    }
    #[test]
    fn simple_enc() {
        assert_eq!(crate::encode("Man ".as_bytes()), "<~9jqo^~>".to_string());
    }
    #[test]
    fn hobbes_encode() {
        assert_eq!(
            crate::encode(HOBBES_PLAIN.as_bytes()),
            HOBBES_ENCODED.to_string()
        );
    }
    #[test]
    fn simple_dec() {
        assert_eq!(crate::decode("<~9jqo^~>"), Some("Man ".as_bytes().to_vec()));
    }
    #[test]
    fn hobbes_decode() {
        assert_eq!(
            crate::decode(HOBBES_ENCODED),
            Some(HOBBES_PLAIN.as_bytes().to_vec())
        );
    }
}
type Bytes = Vec<u8>;

pub fn encode(input: &[u8]) -> String {
    let mut input: Vec<u8> = input.iter().cloned().collect();
    let input = &mut input;
    let last_chunk_length = if input.len() % 4 == 0 {
        0
    } else {
        4 - input.len() % 4
    };
    for _i in 0..last_chunk_length {
        input.push(0);
    }
    let mut chars: Vec<char> = Vec::new();
    chars.push('<');
    chars.push('~');
    for chunk in 0..(input.len() / 4) {
        let i = chunk * 4;
        if input[i..(i + 4)] == [0, 0, 0, 0] {
            chars.push('z');
        } else {
            let mut big_num: u32 = (input[i] as u32) * 256 * 256 * 256
                + (input[i + 1] as u32) * 256 * 256
                + (input[i + 2] as u32) * 256
                + (input[i + 3] as u32);
            let last = chars.len();
            chars.extend("     ".chars());
            chars[last + 4] = (33 + big_num % 85) as u8 as char;
            big_num /= 85;
            chars[last + 3] = (33 + big_num % 85) as u8 as char;
            big_num /= 85;
            chars[last + 2] = (33 + big_num % 85) as u8 as char;
            big_num /= 85;
            chars[last + 1] = (33 + big_num % 85) as u8 as char;
            big_num /= 85;
            chars[last] = (33 + big_num) as u8 as char;
        }
    }
    for _i in 0..last_chunk_length {
        chars.pop();
    }
    chars.push('~');
    chars.push('>');
    chars.iter().collect()
}
pub fn decode(input: &str) -> Option<Bytes> {
    let mut input: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    if input[0] != '<'
        || input[1] != '~'
        || input[input.len() - 1] != '>'
        || input[input.len() - 2] != '~'
    {
        None
    } else {
        let mut output: Vec<u8> = Vec::new();
        input.remove(0);
        input.remove(0);
        input.pop();
        input.pop();
        let mut i = 0;
        while i < input.len() {
            if input[i] == 'z' {
                output.extend([0, 0, 0, 0].iter());
                i += 1;
            } else if input.len() - i >= 5 {
                if !input[i..i + 5].iter().all(|c| c >= &'!' && c <= &'u') {
                    return None;
                }
                let mut big_number = (input[i] as u64 - 33) * 85 * 85 * 85 * 85
                    + (input[i + 1] as u64 - 33) * 85 * 85 * 85
                    + (input[i + 2] as u64 - 33) * 85 * 85
                    + (input[i + 3] as u64 - 33) * 85
                    + (input[i + 4] as u64 - 33);
                let output_end = output.len();
                output.extend([0, 0, 0, 0].iter());
                output[output_end + 3] = (big_number % 256) as u8;
                big_number /= 256;
                output[output_end + 2] = (big_number % 256) as u8;
                big_number /= 256;
                output[output_end + 1] = (big_number % 256) as u8;
                big_number /= 256;
                output[output_end] = big_number as u8;
                i += 5;
            } else if input.len() - i > 0 {
                let mut big_number = 0;
                for j in 0..4 {
                    if i + j < input.len() {
                        big_number += input[i + j] as u64 - 33
                    } else {
                        big_number += 84 as u64;
                    }
                    big_number *= 85;
                }
                if i + 4 < input.len() {
                    big_number += input[i + 4] as u64 - 33
                }
                let output_end = output.len();
                output.extend([0, 0, 0, 0].iter());
                output[output_end + 3] = (big_number % 256) as u8;
                big_number /= 256;
                output[output_end + 2] = (big_number % 256) as u8;
                big_number /= 256;
                output[output_end + 1] = (big_number % 256) as u8;
                big_number /= 256;
                output[output_end] = big_number as u8;
                for _j in 0..input.len() - i + 1 {
                    output.pop();
                }
                i += 5;
            }
        }
        Some(output)
    }
}

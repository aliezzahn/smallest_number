use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn smallest_number(num: String, t: i64) -> String {
        let mut num = num.into_bytes(); // Use Vec<u8> for ASCII manipulation
        let num_size = num.len();

        // Early exit if t == 1
        if t == 1 {
            for i in 0..num_size {
                if num[i] == b'0' {
                    for j in i..num_size {
                        num[j] = b'1';
                    }
                    break;
                }
            }
            return String::from_utf8(num).unwrap();
        }

        // Factorize t into powers of 2, 3, 5, and 7
        let mut num2 = t.trailing_zeros() as i32;
        let mut t = t >> num2;

        let mut num3 = 0;
        let mut num5 = 0;
        let mut num7 = 0;

        while t % 3 == 0 {
            num3 += 1;
            t /= 3;
        }
        while t % 5 == 0 {
            num5 += 1;
            t /= 5;
        }
        while t % 7 == 0 {
            num7 += 1;
            t /= 7;
        }

        // If t has prime factors other than 2, 3, 5, or 7, return "-1"
        if t != 1 {
            return "-1".to_string();
        }

        // Subtract contributions from existing digits
        let mut i = 0;
        while i < num_size && num[i] != b'0' {
            match num[i] {
                b'9' => num3 -= 2,
                b'8' => num2 -= 3,
                b'7' => num7 -= 1,
                b'6' => {
                    num2 -= 1;
                    num3 -= 1;
                }
                b'5' => num5 -= 1,
                b'4' => num2 -= 2,
                b'3' => num3 -= 1,
                b'2' => num2 -= 1,
                _ => {}
            }
            i += 1;
        }

        // Replace remaining digits with '1'
        for j in i..num_size {
            num[j] = b'1';
        }

        // If no more factors are needed, return the result
        if num2 <= 0 && num3 <= 0 && num5 <= 0 && num7 <= 0 {
            return String::from_utf8(num).unwrap();
        }

        // Try to find the smallest number by rearranging digits
        let mut need_increase = true;
        let mut i = num_size as i32 - 1;
        while i >= 0 {
            let a = num[i as usize] - b'0';
            match a {
                9 => num3 += 2,
                8 => num2 += 3,
                7 => num7 += 1,
                6 => {
                    num2 += 1;
                    num3 += 1;
                }
                5 => num5 += 1,
                4 => num2 += 2,
                3 => num3 += 1,
                2 => num2 += 1,
                _ => {}
            }

            let need2 = max(num2, 0);
            let need3 = max(num3, 0);
            let mut len = need2 / 3 + need3 / 2 + max(num5, 0) + max(num7, 0);
            let need2 = need2 % 3;
            let need3 = need3 % 2;
            len += (need2 + need3) / 2 + (need2 + need3) % 2;

            if len <= (num_size as i32 - i) {
                for ri in (a + if need_increase { 1 } else { 0 })..10 {
                    let mut ri2 = 0;
                    let mut ri3 = 0;
                    let mut ri5 = 0;
                    let mut ri7 = 0;
                    match ri {
                        9 => ri3 += 2,
                        8 => ri2 += 3,
                        7 => ri7 += 1,
                        6 => {
                            ri2 += 1;
                            ri3 += 1;
                        }
                        5 => ri5 += 1,
                        4 => ri2 += 2,
                        3 => ri3 += 1,
                        2 => ri2 += 1,
                        _ => {}
                    }

                    let need2 = max(num2 - ri2, 0);
                    let need3 = max(num3 - ri3, 0);
                    let mut len = need2 / 3 + need3 / 2 + max(num5 - ri5, 0) + max(num7 - ri7, 0);
                    let need2 = need2 % 3;
                    let need3 = need3 % 2;
                    len += (need2 + need3) / 2 + (need2 + need3) % 2;

                    if len < (num_size as i32 - i) {
                        num[i as usize] = ri + b'0';
                        num2 -= ri2;
                        num3 -= ri3;
                        num5 -= ri5;
                        num7 -= ri7;
                        let mut j = num_size as i32 - 1;
                        while j > i {
                            if num3 > 1 {
                                num[j as usize] = b'9';
                                num3 -= 2;
                            } else if num2 > 2 {
                                num[j as usize] = b'8';
                                num2 -= 3;
                            } else if num7 > 0 {
                                num[j as usize] = b'7';
                                num7 -= 1;
                            } else if num2 > 0 && num3 > 0 {
                                num[j as usize] = b'6';
                                num2 -= 1;
                                num3 -= 1;
                            } else if num5 > 0 {
                                num[j as usize] = b'5';
                                num5 -= 1;
                            } else if num2 > 1 {
                                num[j as usize] = b'4';
                                num2 -= 2;
                            } else if num3 > 0 {
                                num[j as usize] = b'3';
                                num3 -= 1;
                            } else if num2 > 0 {
                                num[j as usize] = b'2';
                                num2 -= 1;
                            } else {
                                num[j as usize] = b'1';
                            }
                            j -= 1;
                        }
                        return String::from_utf8(num).unwrap();
                    }
                }

                num[i as usize] = b'1';
                need_increase = false;

                let mut j = i - 1;
                while j >= 0 {
                    let a = num[j as usize] - b'0';
                    if a != 9 {
                        match a {
                            8 => {
                                num2 += 3;
                                num3 -= 2;
                            }
                            7 => {
                                num7 += 1;
                                num2 -= 3;
                            }
                            6 => {
                                num2 += 1;
                                num3 += 1;
                                num7 -= 1;
                            }
                            5 => {
                                num5 += 1;
                                num2 -= 1;
                                num3 -= 1;
                            }
                            4 => {
                                num2 += 2;
                                num5 -= 1;
                            }
                            3 => {
                                num3 += 1;
                                num2 -= 2;
                            }
                            2 => {
                                num2 += 1;
                                num3 -= 1;
                            }
                            _ => {
                                num2 -= 1;
                            }
                        }
                        num[j as usize] = (a + 1) + b'0';
                        break;
                    }
                    num[j as usize] = b'1';
                    num3 += 2;
                    j -= 1;
                }
                if j == -1 {
                    num.insert(0, b'1');
                    break;
                }
            }
            i -= 1;
        }

        if need_increase {
            num.insert(0, b'1');
        }

        let mut i = num.len() as i32 - 1;
        while num2 > 0 || num3 > 0 || num5 > 0 || num7 > 0 {
            let a = if num3 > 1 {
                num3 -= 2;
                b'9'
            } else if num2 > 2 {
                num2 -= 3;
                b'8'
            } else if num7 > 0 {
                num7 -= 1;
                b'7'
            } else if num2 > 0 && num3 > 0 {
                num2 -= 1;
                num3 -= 1;
                b'6'
            } else if num5 > 0 {
                num5 -= 1;
                b'5'
            } else if num2 > 1 {
                num2 -= 2;
                b'4'
            } else if num3 > 0 {
                num3 -= 1;
                b'3'
            } else if num2 > 0 {
                num2 -= 1;
                b'2'
            } else {
                b'1'
            };

            if i >= 0 {
                num[i as usize] = a;
                i -= 1;
            } else {
                num.insert(0, a);
            }
        }

        String::from_utf8(num).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to generate test cases
    fn test_case(num: &str, t: i64, expected: &str) {
        assert_eq!(Solution::smallest_number(num.to_string(), t), expected.to_string());
    }

    // Test cases
    #[test]
    fn test_case_1() {
        test_case("123", 6, "123");
    }

    #[test]
    fn test_case_2() {
        test_case("100", 1, "111");
    }

    #[test]
    fn test_case_3() {
        test_case("100", 2, "112");
    }

    #[test]
    fn test_case_4() {
        test_case("100", 3, "113");
    }

    #[test]
    fn test_case_5() {
        test_case("100", 4, "114");
    }

}
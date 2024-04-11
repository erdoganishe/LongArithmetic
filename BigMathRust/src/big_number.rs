use core::panic;
use std::cmp::Ordering;

pub struct BigNumber {
    pub value: Vec<u8>, 
}

impl Clone for BigNumber {

    fn clone(&self) -> Self {

        BigNumber {

            value: self.value.clone(),

        }

    }

}
impl BigNumber {
    pub fn new() -> Self {
        Self { value: Vec::new() }
    }


    pub fn set_hex(&mut self, hex_str: &str) {
        self.value.clear();
    
        let mut hex_chars: Vec<char> = hex_str.chars().collect();
        if hex_chars.len() % 2 != 0 {
            hex_chars.insert(0, '0');
        }
    
        let mut current_value: u128 = 0;
        let mut current_digit;
        let mut digit_count = 0;
    
        for &hex_char in &hex_chars {
            current_digit = match hex_char {
                '0'..='9' => hex_char as u128 - b'0' as u128,
                'A'..='F' => hex_char as u128 - b'A' as u128 + 10,
                'a'..='f' => hex_char as u128 - b'a' as u128 + 10,
                _ => {
                    panic!("Invalid hex character: {}", hex_char);
                }
            };
    
            current_value = (current_value << 4) | current_digit;
            digit_count += 1;
    
            if digit_count == 2 {
                self.value.push((current_value & 0xFF) as u8);
                current_value >>= 8;
                digit_count = 0;
            }
        }
    
        if digit_count > 0 {
            self.value.push(current_value as u8);
        }
    }
    
    
    pub fn get_hex(&self) -> String {
        if self.value.is_empty() {
            return "00".to_string();
        }
    
        let mut hex_string = String::new();
    
        for (_i, &byte) in self.value.iter().enumerate() {
            let byte_string = format!("{:02X}", byte);
            hex_string.push_str(&byte_string);
        }
    
        if let Some(first_char) = hex_string.chars().next() {
            if first_char == '0' {
                hex_string.remove(0);
            }
        }
    
        hex_string
    }

    pub fn compare(&self, other: &BigNumber) -> i32 {
        let str_self = self.get_hex();
        let str_other = other.get_hex();

        if str_self.len() > str_other.len() {return 1}
        if str_self.len() < str_other.len() {return -1}
        
        match str_self.cmp(&str_other) {

            Ordering::Less => return -1,
            Ordering::Equal => return 0,
            Ordering::Greater => return 1
        }

    }

    pub fn from_u(&mut self, u: &usize) {
        let u_hex: String = format!("{:02X}", u);
        self.set_hex(&u_hex);
    }
    
    pub fn to_u(&mut self) ->usize{

        let a_usize: Result<usize, _> = usize::from_str_radix(&self.get_hex(), 16);
        match a_usize {

            Ok(value) => return value,
        
            Err(_e) => println!("Error: {}", _e),     
        }
        return 0
    }

    pub fn inv(&mut self) {
        for byte in &mut self.value {
            *byte = !*byte;
        }
    }
    
    pub fn xor(&mut self, other: &BigNumber) {
        for (byte, other_byte) in self.value.iter_mut().zip(other.value.iter()) {
            *byte ^= *other_byte;
        }
    }

    pub fn or(&mut self, other: &BigNumber) {
        for (byte, other_byte) in self.value.iter_mut().zip(other.value.iter()) {
            *byte |= *other_byte;
        }
    }

    pub fn and(&mut self, other: &BigNumber) {
        for (byte, other_byte) in self.value.iter_mut().zip(other.value.iter()) {
            *byte &= *other_byte;
        }
    }

    pub fn shift_r(&mut self, n: usize) {
        let mut carry = 0;
        for byte in self.value.iter_mut() {
            let new_byte = (*byte >> n) | (carry << (8 - n));
            carry = *byte & ((1 << n) - 1);
            *byte = new_byte;
        }
    }

    pub fn shift_l(&mut self, n: usize) {
        let mut carry = 0;
        for byte in self.value.iter_mut().rev() {
            let new_byte = (*byte << n) | carry;
            carry = *byte >> (8 - n);
            *byte = new_byte;
        }
    }

    pub fn add(&mut self, other: &BigNumber) {
        let hex1 = self.get_hex();
        let hex2 = other.get_hex();
        let mut res: String = "".to_string();
        let mut carry: bool = false;
    
        for i in 0..hex1.len().max(hex2.len()) + 1 {
            let cur_char1 = hex1.chars().rev().nth(i).unwrap_or('0');
            let cur_char2 = hex2.chars().rev().nth(i).unwrap_or('0');
            let hex_values = "0123456789ABCDEF";
            let val1 = match hex_values.find(cur_char1) {
                Some(position) => position as u8,
                None => panic!(""),
            };
            let val2 = match hex_values.find(cur_char2) {
                Some(position) => position as u8,
                None => panic!(""),
            };
            
            let mut sum: u8 = val1 + val2;
    
            if carry {
                sum += 1;
                carry = false;
            }
    
            if sum > 15 {
                carry = true;
                sum -= 16;
            }
    
            let hex_char = match sum {
                0..=9 => (sum + b'0') as char,
                10..=15 => (sum - 10 + b'A') as char,
                _ => panic!(""),
            };
    
            res = hex_char.to_string() + &res;
                   }
        if let Some(first_char) = res.chars().next() {
            if first_char == '0' {
                res.remove(0);
            }
        }
        self.set_hex(&res);

    }
    
    pub fn subtract(&mut self, other: &BigNumber) {

        if self.compare(other) == -1 {
            panic!("");
        }
        let hex1 = self.get_hex();
        let hex2 = other.get_hex();
        let mut res: String = "".to_string();
        let mut borrow: bool = false;
        
        for i in 0..hex1.len().max(hex2.len()) {
            let cur_char1 = hex1.chars().rev().nth(i).unwrap_or('0');
            let cur_char2 = hex2.chars().rev().nth(i).unwrap_or('0');
            let hex_values = "0123456789ABCDEF";
            let val1 = match hex_values.find(cur_char1) {
                Some(position) => position as i32,
                None => panic!(""),
            };
            let val2 = match hex_values.find(cur_char2) {
                Some(position) => position as i32,
                None => panic!(""),
            };
    
            let mut difference = val1 - val2;
    
            if borrow {
                difference -= 1;
                borrow = false;
            }
    
            if difference < 0 {
                difference += 16;
                borrow = true;
            }
    
            let hex_char = match difference {
                0..=9 => (difference + b'0' as i32) as u8 as char,
                10..=15 => (difference - 10 + b'A' as i32) as u8 as char,
                _ => panic!(""),
            };
    
            res = hex_char.to_string() + &res;
        }
        if let Some(first_char) = res.chars().next() {
            if first_char == '0' {
                res.remove(0);
            }
        }
        self.set_hex(&res);
    }


    pub fn modulo(&mut self, other: &BigNumber) {
        if other.get_hex() == "00" {
            panic!("Division by zero");
        }
    
        let mut dividend = self.clone();
        let divisor = other.clone();
    
        while dividend.compare(&divisor) > 0 {
            dividend.subtract(&divisor);

        }
    
        *self = dividend;
    }

    pub fn div(&mut self, other: &BigNumber) {
        if other.get_hex() == "00" {
            panic!("Division by zero");
        }
    
        let mut dividend = self.clone();
        let divisor = other.clone();
        let mut div_res = BigNumber::new();
        div_res.set_hex("00");
    
        while dividend.compare(&divisor) > 0 {
            dividend.subtract(&divisor);
            let mut one = BigNumber::new();
            one.set_hex("01");

            div_res.add(&one);
        }
    
        *self = div_res;
    }

    pub fn basic_multiply(&mut self, other: &mut BigNumber) -> BigNumber {
        let first = &self.to_u();
        let second = &other.to_u();
        let mut res = BigNumber::new();
        res.from_u(&(first*second));
        return res
    }

    pub fn karatsuba_multiply(&mut self, other: &mut BigNumber) -> BigNumber {
        let self_len = self.get_hex().len() as u32;
        let other_len = self.get_hex().len() as u32;
    
    
        if self_len == 1 && other_len == 1 {
            let mut sample_res = BigNumber::new();
            sample_res = self.basic_multiply(other);
            return sample_res
        }

        let half_self_len = self_len / 2;
        let half_other_len = other_len / 2;
        let self_multiplier = 16_u128.pow(half_self_len as u32);
        let other_multiplier = 16_u128.pow(half_other_len as u32);
    
        let mut self_mult_big = BigNumber::new();
        self_mult_big.from_u(&(self_multiplier as usize));
        let mut other_mult_big = BigNumber::new();
        other_mult_big.from_u(&(other_multiplier as usize));

        let mut b = BigNumber::new();
        b = self.clone();
        b.modulo(&self_mult_big);

        let mut d = BigNumber::new();
        d = other.clone();
        d.modulo(&other_mult_big);

        let mut a = BigNumber::new();
        a = self.clone();
        a.div(&self_mult_big);

        let mut c = BigNumber::new();
        c = other.clone();
        c.div(&other_mult_big);

  
    
        let mut ac = BigNumber::new();
        ac = a.karatsuba_multiply(&mut c.clone());
        let mut bd = BigNumber::new(); 
        bd = b.karatsuba_multiply(&mut d.clone());
        let mut ad  = BigNumber::new();
        ad = a.karatsuba_multiply(&mut d.clone());
        let mut bc  = BigNumber::new(); 
        bc = b.karatsuba_multiply(&mut c.clone());
    
        let mut res = BigNumber::new();        
        res.set_hex("0");

        let mut first = BigNumber::new();
        first.from_u(&(16_u128.pow((half_self_len + half_other_len) as u32) as usize));
        let mut second = BigNumber::new();
        second.from_u(&((16_u128.pow((half_self_len) as u32))as usize));
        let mut third = BigNumber::new();
        third.from_u(&((16_u128.pow((half_other_len) as u32))as usize));

        res.add(&ac.basic_multiply(&mut first));
        res.add(&ad.basic_multiply(&mut second));
        res.add(&bc.basic_multiply(&mut third));
        res.add(&bd);
        return res
    }
    
    //conv Strings
    pub fn hex_to_binary(hex: &str) -> String {
        let mut binary = String::new();
        for c in hex.chars() {
            match c {
                '0' => binary.push_str("0000"),
                '1' => binary.push_str("0001"),
                '2' => binary.push_str("0010"),
                '3' => binary.push_str("0011"),
                '4' => binary.push_str("0100"),
                '5' => binary.push_str("0101"),
                '6' => binary.push_str("0110"),
                '7' => binary.push_str("0111"),
                '8' => binary.push_str("1000"),
                '9' => binary.push_str("1001"),
                'a' | 'A' => binary.push_str("1010"),
                'b' | 'B' => binary.push_str("1011"),
                'c' | 'C' => binary.push_str("1100"),
                'd' | 'D' => binary.push_str("1101"),
                'e' | 'E' => binary.push_str("1110"),
                'f' | 'F' => binary.push_str("1111"),
                _ => panic!("Invalid hex character: {}", c),
            }
        }
        binary
    }
    
    //another multiplication
    pub fn multiply(&mut self, other: &mut BigNumber)->BigNumber{        
        let mut result = BigNumber::new();
        result.set_hex("0");

        let binary = Self::hex_to_binary(&other.get_hex());

        for i in 0..binary.len() {
        
            match binary.chars().nth(i).unwrap() {
        
                '0' => {
                    result.add(&result.clone());
                }
        
                '1' => {
                    result.add(&result.clone());
                    result.add(&self.clone())
                }
        
                _ => panic!("Invalid binary character: {}", binary.chars().nth(i).unwrap()),
        
            }
        
        }

        return result
    }
    

    //pow 

    pub fn power(&mut self, other: &BigNumber)-> BigNumber{
        let mut res = BigNumber::new();
        res.set_hex("1");
        let exp = other.clone().to_u();
        let i_exp:i32 = exp.try_into().unwrap();
        for _i in 0..i_exp{
            res = res.multiply(self);
        }
        return res;
    }
}

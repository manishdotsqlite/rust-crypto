use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use regex::Regex;


// Challenge 1 - function that changes hex to base 64 
pub fn hex_to_base64(hex: &str) -> Result<String, &'static str> {
        let hex_to_vector = match hex::decode(hex) {
        Ok(some) => some,
        Err(_) => return Err("Couldn't decode hex."),
        };

        let vector_to_base64 = base64::encode(hex_to_vector);
        Ok(vector_to_base64)
}



// Challenge 2 - function that takes two equal length buffers and produces their XOR combination
pub fn xor(value1: &str, value2: &str) -> Result<String, &'static str> {

        if value1.len() != value2.len() {
                return Err("Two buffers must be equal length.")
        } 

        let vec_1 = match hex::decode(value1) {
                Ok(some) => some,
                Err(_) => return Err("Couldn't decode value 1."),
        };

        let vec_2 = match hex::decode(value2) {
                Ok(some) => some,
                Err(_) => return Err("Couldn't decode value 1."),
        };

        let xor_vec:Vec<u8> = vec_1.iter().zip(vec_2.iter()).map(|(a,b)| a ^ b ).collect();

        let product = hex::encode(xor_vec);
        Ok(product)
}



// Challenge 3 - single-byte xor cipher
pub fn single_byte_xor_cipher(hex: &str) {
        let vec = match hex::decode(hex){
                Ok(some) => some,
                Err(_) => Vec::new()
        };

        for i in 0..=255 {
                let character = i as u8;
                let indiv_product:Vec<u8> = vec.iter().map(|item| item ^ character).collect();
                let indiv_string = match String::from_utf8(indiv_product){
                        Ok(some) => some,
                        Err(_) => "Couldn't xor string.".to_owned(),
                };
                if indiv_string != "Couldn't xor string." && !indiv_string.contains("u{")  {
                        println!("Index: {:?}, Deciphered String: {:?}", i, indiv_string);
                }
        }

}



// Challenge 4 - Detect single-character XOR in a file
pub fn single_character_xor() -> Result<(), &'static str> {
        let file = match File::open("src/test-files/test.txt") {
                Ok(some) => some,
                Err(_) => return Err("Couldn't read file! ")
        };

        let reader = BufReader::new(file);
        for line in reader.lines() {
                let string_line: String = match line {
                    Ok(some) => some ,
                    Err(_) => String::from("Couldn't decipher string!")
                };

                single_byte_xor_cipher(&string_line);
        }

        Ok(())
}



// helper function - xor's a vector of u8 by a character
pub fn xor_of_vector(vect: Vec<u8>, c: char) -> Vec<u8> {
        let character = c as u8;
        let result: Vec<u8> = vect.iter().map(|item| item ^ character).collect();
        result
}

// Challenge 5 - Implementing repeating-key XOR
pub fn repeating_key_xor(value: &str, key: &str) -> Result<String, &'static str> {
        let hex = value.as_bytes().to_vec();
        let length: usize = hex.len();
        let mut result: Vec<u8> = Vec::new();
        let characters = key.as_bytes();
        for chunk in hex.chunks(3) {
                if chunk.len() == 1 {
                        result.push(chunk[0] ^ characters[0]);
                }
                if chunk.len() == 2 {
                        result.push(chunk[0] ^ characters[0]);
                        result.push(chunk[1] ^ characters[1]);
                }
                if chunk.len() == 3 {
                        result.push(chunk[0] ^ characters[0]);
                        result.push(chunk[1] ^ characters[1]);
                        result.push(chunk[2] ^ characters[2]);
                }
        }

        let result_string = String::from_utf8(result);
        match result_string {
            Ok(some) => Ok(hex::encode(some)),
            Err(_) => Err("Error when repeating xor.")
        }
}



// Hamming Distance function 
pub fn hamming_distance(value1: &str, value2: &str) -> Result<i32, &'static str> {
        
        if value1.len() != value2.len() {
                return Err("Two sequences should be of same length")
        }

        let bits1 = value1.chars().map(|c| format!("{:08b}", c as u8)).collect::<Vec<String>>().join("");
        let bits2 = value2.chars().map(|c| format!("{:08b}", c as u8)).collect::<Vec<String>>().join("");

        let mut hamming_distance = 0;
        for (c1, c2) in bits1.chars().zip(bits2.chars()) {
                if c1 != c2 {
                        hamming_distance = hamming_distance + 1;
                }
        }

        Ok(hamming_distance)
        
}

fn vector_to_matrix(data: Vec<u8>, rows: usize, cols: usize) -> Vec<Vec<u8>> {
        let mut matrix = Vec::new();

        for i in 0..rows {
                let start = i * cols;
                let end = std::cmp::min(start + cols, data.len());
                matrix.push(data[start..end].to_vec());
        }

        matrix
} 

fn transpose_matrix(matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        if matrix.is_empty() {
                return matrix;
        }

        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut transposed = vec![vec![None; rows]; cols];

        for i in 0..rows {
                for j in 0..cols {
                        transposed[j][i] = Some(matrix[i][j].clone());
                }
        }

        transposed.into_iter().map(|row| {
                row.into_iter().flatten().collect()
        }).collect()
}


// Challenge 6 - Decrypt test-2.txt
pub fn decrypt_file() -> Result<(), &'static str> {
        let mut file = match File::open("src/test-files/test-2.txt") {
                Ok(some) => some,
                Err(_) => return Err("Couldn't read file! ")
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents) ;
        let content = contents.replace("\n", "").replace("\r", "");
        let content_bytes = content.into_bytes();


        let mut normalization_vec: Vec<i32> = Vec::new();
        for i in 2..=50 {
                let first_n_bytes = match String::from_utf8(content_bytes[..i].to_vec()) {
                        Ok(some) => some,
                        Err(_) => return Err("Error 101!")
                };
                let second_n_bytes = match String::from_utf8(content_bytes[i..2 * i].to_vec()) {
                        Ok(some) => some,
                        Err(_) => return Err("Error 102!")
                };;
                let hamming_distance = hamming_distance(&first_n_bytes, &second_n_bytes)?;
                normalization_vec.push(hamming_distance);
        }

        normalization_vec.sort();
        let content_length = content_bytes.len();
        let average_normalization = ((normalization_vec[0] + normalization_vec[1] + normalization_vec[2] + normalization_vec[3]) / 4) as usize ;
        let matrix = vector_to_matrix(content_bytes, average_normalization, content_length/average_normalization);
        let transpose = transpose_matrix(matrix);

        for row in transpose {
                let row_string = match String::from_utf8(row) {
                        Ok(some) => some,
                        Err(_) => return Err("Error 103!")
                };
                println!("LINE: {:?}", &row_string);
                single_byte_xor_cipher(&row_string);
        }

        Ok(())
        
}
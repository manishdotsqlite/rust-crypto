
// function to turn a string into vector 
pub fn string_to_vec(value: &str) -> Vec<u8> {
        value.as_bytes().chunks(2).map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap()).collect()
}

//function to turn vector to string 
pub fn vec_to_string(value: Vec<u8>) -> String {
        match String::from_utf8(value) {
            Ok(string) => return string,
            Err(_) => return "ERROR during vector to string conversion.".to_owned()
        }
}


//function to print a vector like a list
pub fn print_vector_as_list(vec: Vec<u8>) {
        if vec.is_empty() {
            println!("Empty Vector");
            return;
        }
    
        let mut iter = vec.iter();
        print!("{{");
        if let Some(first) = iter.next() {
            print!("{}", first);
        }
        for item in iter {
            print!(", {}", item);
        }
        println!("}}");
    }


// function to xor a vec<u8> by a character
pub fn xor(vector: Vec<u8>, character: u8) -> Vec<u8> {
        let result = vector.iter().map(|item| item ^ character).collect();
        result
}


// function to xor a vec<u8> by all characters in utf8 
pub fn xor_by_all(vector: Vec<u8>) {
        let len = vector.len();
        for i in 0..len {
                let current_byte = i as u8;
                let result = xor(vector.clone(), current_byte);
                let vec_as_string = vec_to_string(result);
                println!("Result String in iteration {:?} : {:?}", i, vec_as_string)
        }
} 



use lib1::{hamming_distance, repeating_key_xor};

mod lib1;

fn main() {
   let value1 = "this is a test";
   let value2 = "wokka wokka!!!";
   
   let hello = hamming_distance(value1, value2);
}


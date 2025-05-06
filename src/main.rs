use lib1::repeating_key_xor;

mod lib1;

fn main() {
   let string = "Burning 'em, if you ain't quick and nimble I go crazy when I hear a cymbal".to_owned();
   match repeating_key_xor(&string, "ICE") {
    Ok(some) => println!("{:?}", some),
    Err(some) => println!("{:?}", some)
   }
}


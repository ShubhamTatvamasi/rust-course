const MEANING_OF_LIFE:u8 = 42; // no fixed address

static mut Z:i32 = 123;

fn main() {
  println!("meaning of life is {}", MEANING_OF_LIFE);
  unsafe {
    Z = 777;
    println!("z = {}", Z);
  }
}

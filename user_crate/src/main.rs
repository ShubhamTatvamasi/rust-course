extern crate building_modules_and_crates;

use building_modules_and_crates::greetings::french;

fn main() {

  println!("English: {}, {}",
    building_modules_and_crates::greetings::english::hello(),
    building_modules_and_crates::greetings::english::goodbye()
  );

  println!("French: {}, {}",
    french::hello(),
    french::goodbye()
  );

}

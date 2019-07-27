fn match_statement() {
  let country_code = 44; // 1-999

  let country = match country_code {
    44 => "UK",
    46 => "Sweden",
    7 => "Russia",
    1...999 => "unknown", // 1..999 = 1-998
    _ => "invalid"
  };

  println!("the country with code {} is {}", country_code, country);
}

fn main() {
  match_statement();
}

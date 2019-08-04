#[cfg(test)]
mod tests {

extern crate testing;

  #[test]
  #[should_panic]
  #[ignore]
  fn english_greeting_correct() {
    assert_eq!("helslo", testing::greetings::english::hello());
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn test_init() {
    let result = balancer_rs::vault::get_number();

    assert_eq!(result, 1);
  }

  // Test to see if the module can get the WETH address from the vault
  #[test]
  fn test_weth() {
    let result = balancer_rs::vault::weth();

    assert_eq!(result, "test");
  }
}

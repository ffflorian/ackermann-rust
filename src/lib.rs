pub fn ack(m: u64, n: u64) -> u64 {
  if m == 0 {
    n + 1
  } else if n == 0 {
    ack(m - 1, 1)
  } else {
    ack(m - 1, ack(m, n - 1))
  }
}

#[cfg(test)]
mod tests {
  use ack;

  #[test]
  fn test_ack() {
    assert_eq!(ack(3, 1), 13);
  }
}

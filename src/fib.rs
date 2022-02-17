pub fn fib(number: u32) -> u32 {
  if [0, 1].contains(&number) {
    return 1;
  }
  return fib(number - 1) + fib(number - 2);
}
fn main() {
   let s="Some Nights";
   let some=&s[0..=3];
   let nights=&s[5..=10];
   println!("{},{}",some,nights);
}

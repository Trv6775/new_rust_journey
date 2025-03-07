fn main() {
   let  s1=String::from("Hello");
   let(s2,len)=calculate_len(s1);
   println!("The length of {s2} is {len}");
}
fn calculate_len(s:String)->(String,usize){
   let length=s.len();
   (s,length)
}

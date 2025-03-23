#[derive(Debug)]
struct User{
   active:bool,
   username:String,
   email:String,
   sign_in_count:u64
}
fn main(){
   let mut user1=User{
      active:true,
      username:String::from("Trevor Kimutai"),
      email:String::from("trv6775@gmail.com"),
      sign_in_count:1
   };
   user1.email=String::from("trv6775330@gmail.com");
   println!("{:?}",user1);
}
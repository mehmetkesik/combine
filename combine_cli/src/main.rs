#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use combine_attributes::*;

#[combine_fn]
fn index(){
}

fn main() {
  let a = A{body:"xyz".to_string()};
  println!("Hello, world! {}",a.add_br("heyyyy".to_string(),5,7.5));
}

#[combine_type]
struct A {
  body:String,
}

impl A {
  #[combine_fn]
  fn add_br(&self,render_body:String,i:i32,f:f64) -> String {
      render_body
  }

  #[combine_fn]
  fn send_http_get_request(&self,path:String) {
    
  }
}
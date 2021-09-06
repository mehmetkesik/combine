#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate proc_macro;
use proc_macro::TokenStream;
use regex::Regex;

#[proc_macro_attribute]
pub fn combine_type(attr: TokenStream, item: TokenStream) -> TokenStream {
  //println!("attr: \"{}\"", attr.to_string());
  //println!("item: \"{}\"", item.to_string());
  /*let mut result = item.to_string();
  result = str::replace(result.as_str(), "{ render_body }", "{ println!(\"merhaba macrocum \"); render_body }");
  let stream: TokenStream = result.parse().unwrap();
  stream*/
  item
}

#[proc_macro_attribute]
pub fn combine_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
  //println!("attr: \"{}\"", attr.to_string());
  //println!("item: \"{}\"", item.to_string());
  
  let re = Regex::new(r"fn(.*)\((.*)\)").unwrap();
  assert!(re.is_match(item.to_string().as_str()));
  let re = Regex::new(r"fn ").unwrap();
  let result = item.to_string().replace("fn ","");
  //println!("{}",result);

  let param_start:usize;
  let param_end:usize;
  let re = Regex::new(r"\(").unwrap();
  if let Some(m) = re.find(result.as_str()){
    param_start = m.start();
    println!("{}",&result[..m.start()]);
  } else {
    return item;
  }

  let re = Regex::new(r"\)").unwrap();
  if let Some(m) = re.find(result.as_str()){
    param_end = m.start() + 1;
  } else {
    return item;
  }

  println!("{}",&result[param_start..param_end]);

  item
}
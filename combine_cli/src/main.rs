#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


extern crate reqwest;

use combine_attributes::*;
use combine::{combine::*, combine_context::*};

fn main() {
    let mut combine = Combine::new("demo.combine.html");

    combine.add("add_br", |context: CombineContext| -> String {
        "<br/>".to_string() //ct.params.get(0).to_string()
    });

    combine.add("get", |context: CombineContext| -> String {
        //let source = context.params.get(0).to_string();

        return match reqwest::blocking::get("https://www.google.com.tr/") {
            Ok(response) => {
                if response.status() == reqwest::StatusCode::OK {
                    return response.text().unwrap();
                } else {
                    println!("Not okey request, response status code: {}", response.status());
                    "".to_string()
                }
            }
            Err(e) => {
                println!("Error while request get: {}", e.to_string());
                "".to_string()
            }
        };
    });

    combine.add("js_var", js_var);

    combine.run();
}

fn js_var(_context: CombineContext) -> String {
    "var _v1 = [1,2,3,4,5];".to_string()
}
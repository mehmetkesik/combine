#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use combine_attributes::*;
use combine::*;

#[combine_fn]
fn index(){
}

fn main() {
    combine::Combine::run(||{
        fn x() {

        }
    });
}

#[combine_type]
struct A {
    body:String,
}

impl A {
    #[combine_fn]
    fn add_br(&self) -> String {
        "<br/>".to_string()
    }

    #[combine_fn]
    fn get(&self, path: String) -> bool {
        true
    }

    #[combine_fn]
    fn x_if(&self,path: String) -> String {
        path
    }
}
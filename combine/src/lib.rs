#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use combine_attributes::*;

enum FnBody {
    Body(String),
    None
}

pub enum CombineValue {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    Object(HashMap<String,Box<CombineValue>>),
    Array(Vec<CombineValue>)
}

pub struct Combine {
    fn_map: HashMap<String, fn(CombineValue) -> CombineValue>,
    fn_map_with_body: HashMap<String, fn(FnBody,CombineValue) -> FnBody>,
    body: String,
}

impl Combine {
    fn new(body: String) -> Combine {
        Combine {
            fn_map: HashMap::new(),
            fn_map_with_body: HashMap::new(),
            body: body,
        }
    }

    #[combine_type]
    pub fn run(func: fn()) {
        let combine = Combine::new("@if(@get('link')) <div> </div> @add_br(3) @end".to_string());
        //x_if("onceki string" + add_br(3) + "sonraki string",get("das"))
    }

    // @func()
    fn add(&mut self,name: String, fnx:fn(CombineValue) -> CombineValue) {
        self.fn_map.insert(name,fnx);
    }

    // @func()  @end
    fn add_with_body(&mut self,name: String, fnx:fn(FnBody,CombineValue) -> FnBody) {
        self.fn_map_with_body.insert(name,fnx);
    }

}

trait ICombine {
    fn combine_fn(&self,render_body:FnBody) -> FnBody;
}
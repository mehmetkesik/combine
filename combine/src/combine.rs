#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use crate::{combine_context::*, functions::*};

pub struct Combine {
    fn_map: HashMap<&'static str, fn(CombineContext) -> String>,
    body: String,
}

impl Combine {
    pub fn new(path: &str) -> Combine {
        let mut combine = Combine {
            fn_map: HashMap::new(),
            body: path.to_string(),
        };
        add_default_functions(&mut combine);
        combine
    }

    pub fn add(&mut self, name: &'static str, fnx: fn(CombineContext) -> String) {
        self.fn_map.insert(name, fnx);
    }

    pub fn run(&mut self) {
        //let combine = Combine::new("@if(@get('link')) <div> </div> @add_br(3) @end".to_string());
        //x_if("onceki string" + add_br(3) + "sonraki string",get("das"))

        /*let func: fn(CombineContext) -> String = match self.fn_map.get("get") {
            Some(t) => *t,
            None => return,
        };
        let ct = CombineContext { params: FnParams { fn_params: Vec::new() }, body: "".to_string() };
        func(ct);*/
        self.analysis();
    }

    fn analysis(&mut self) {}
}

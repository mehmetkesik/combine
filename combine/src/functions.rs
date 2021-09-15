use crate::{combine::*, combine_context::*};

pub fn add_default_functions(combine: &mut Combine) {
    combine.add("js_let", js_let);
}

fn js_let(context: CombineContext) -> String {
    "var _v1 = [1,2,3,4,5];".to_string()
}
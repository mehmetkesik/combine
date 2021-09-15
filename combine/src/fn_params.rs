pub struct FnParams {
    fn_params: Vec<&'static str>,
}

impl FnParams {
    pub fn get(&self, index: usize) -> &'static str {
        return self.fn_params[index];
    }
}
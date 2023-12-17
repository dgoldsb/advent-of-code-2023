pub struct StringManipulator {
    original_string: String,
    iteration: usize,
    manipulate_fn: Box<dyn FnMut(&mut String)>,
}

impl StringManipulator {
    pub fn new(input: &str, manipulate_fn: Box<dyn FnMut(&mut String)>) -> Self {
        StringManipulator {
            original_string: input.to_string(),
            iteration: 0,
            manipulate_fn,
        }
    }

    fn manipulate_string(&mut self) {
        (self.manipulate_fn)(&mut self.original_string);
    }
}

impl Iterator for StringManipulator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.manipulate_string();
        self.iteration += 1;
        Some(self.original_string.clone())
    }
}

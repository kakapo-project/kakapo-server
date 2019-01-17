
#[derive(Clone, Debug)]
pub struct PyRunner {
    python_path: String,
}

impl PyRunner {
    pub fn new(python_path: String) -> PyRunner {
        PyRunner {
            python_path: python_path
        }
    }

    pub fn get_script_path(&self) -> String {
        self.python_path.to_string()
    }
}
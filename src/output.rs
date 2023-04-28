use crate::store;

pub struct Output {
    this: String,
    children: Vec<Self>,
}

impl Output {
    pub fn push(&mut self, item: impl Into<Self>) {
        self.children.push(item.into());
    }

    pub fn with(mut self, item: impl Into<Self>) -> Self {
        self.push(item);
        self
    }

    pub fn finish(self) -> String {
        let mut result = self.this;
        
        for child in self.children {
            result.push_str("\n    ");
            result.push_str(&child.finish());
        }
        result
    }
}

impl From<String> for Output {
    fn from(this: String) -> Self {
        Self {
            children: Vec::new(),
            this
        }
    }
}

impl From<&str> for Output {
    fn from(this: &str) -> Self {
        Self {
            children: Vec::new(),
            this: this.to_owned()
        }
    }
}

pub trait OutputAsResult<T> {
    fn output_err(self) -> Result<T, Output>;
}

impl<T> OutputAsResult<T> for Result<T, s2rs::api::Error> {
    fn output_err(self) -> Result<T, Output> {
        type E = s2rs::api::Error;
        self.map_err(|e| {
            match e {
                E::Network(err) => {
                    Output::from("Network").with(err.to_string())
                },
                E::Parsing(err) => {
                    Output::from("Parsing").with(err.to_string())
                },
                E::Status(code) => {
                    let mut output = Output::from(format![
                        "Status code {} `{}`", code.as_u16(), code.canonical_reason().unwrap_or("???")
                    ]);
                    match code.as_u16() {
                        429 => output.push("Whoops! you are sending requests too fast!"),
                        _ => {}
                    }
                    output
                }
            }
        })
    }
}

impl<T> OutputAsResult<T> for Result<T, std::io::Error> {
    fn output_err(self) -> Result<T, Output> {
        self.map_err(|e| {
            Output::from("IO").with(e.to_string())
        })
    }
}

impl<T> OutputAsResult<T> for Result<T, store::WriteError> {
    fn output_err(self) -> Result<T, Output> {
        self.map_err(|e| {
            match e {
                store::WriteError::Io(err) => Err(err).output_err(),
                store::WriteError::Ser(err) => Output::from("Serialization").with(err.to_string())
            }
        })
    }
}
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
        Self::from(this.to_owned())
    }
}

pub trait OutputErr<T> {
    fn output_err(self) -> Result<T, Output>;
}

impl<T, E: Into<Output>> OutputErr<T> for Result<T, E> {
    fn output_err(self) -> Result<T, Output> {
        self.map_err(|e| e.into())
    }
}

impl From<s2rs::api::Error> for Output {
    fn from(value: s2rs::api::Error) -> Self {
        type E = s2rs::api::Error;
        match value {
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
    }
}

impl From<s2rs::api::LoginError> for Output {
    fn from(value: s2rs::api::LoginError) -> Self {
        type E = s2rs::api::LoginError;
        Output::from("Login")
        .with(match value {
            E::CookiesParsing(err) => Output::from("Parsing cookies").with(err.to_string()),
            E::HeaderParsing(err) => Output::from("Parsing Header").with(err.to_string()),
            E::HeadersConverting(err) => "Converting header".into(),
            E::Parsing(err) => err.into(),
            E::SessionIdCookieNotFound => "Finding Session ID cookie in response".into(),
            E::SetCookieHeaderNotFound => "Finding Set-Cookie header in response".into(),
            E::This(err) => err.into(),
        })
    }
}

impl From<s2rs::api::LoginParseError> for Output {
    fn from(value: s2rs::api::LoginParseError) -> Self {
        match value {
            s2rs::api::LoginParseError::EmptyArray => "Finding data in response".into(),
            s2rs::api::LoginParseError::Expected(err) => "Expected (todo!)".into()
        }
    }
}

impl From<std::io::Error> for Output {
    fn from(value: std::io::Error) -> Self {
        Output::from("IO").with(value.to_string())
    }
}

impl From<store::WriteError> for Output {
    fn from(value: store::WriteError) -> Self {
        Output::from("Writing to store")
        .with(match value {
            store::WriteError::Io(err) => err.into(),
            store::WriteError::Ser(err) => Output::from("Serialization").with(err.to_string()),
        })
    }
}

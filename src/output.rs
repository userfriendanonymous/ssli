use colored::{Colorize, ColoredString};

// pub trait OutputCover: Into<Output> {
//     fn output_cover() -> Output;
// }

// pub trait CoveredOutput {
//     fn covered_output(self) -> Output;
// }

// impl<T: OutputCover> CoveredOutput for T {
//     fn covered_output(self) -> Output {
//         Self::output_cover().with(self)
//     }
// }

pub trait WithOutput {
    fn with_output(self, output: impl Into<Output>) -> Output;
}

impl WithOutput for String {
    fn with_output(mut self, output: impl Into<Output>) -> Output {
        let (this, children) = output.into().unwrap();
        self.push_str(&this);

        let mut output = Output::from(self);
        output.replace_children(children);
        output
    }
}

impl WithOutput for &str {
    fn with_output(self, output: impl Into<Output>) -> Output {
        self.to_owned().with_output(output)
    }
}

pub struct Output {
    this: String,
    children: Vec<Self>,
}

impl Output {
    pub fn push(&mut self, item: impl Into<Self>) {
        self.children.push(item.into());
    }

    pub fn replace_children(&mut self, children: Vec<Self>) {
        self.children = children
    }

    pub fn with(mut self, item: impl Into<Self>) -> Self {
        self.push(item);
        self
    }

    pub fn unwrap(self) -> (String, Vec<Self>) {
        (self.this, self.children)
    }

    pub fn finish(self, mut ident: u8) -> String {
        let mut result = String::new();
        
        if !self.this.is_empty() {
            for _ in 0..ident {
                result.push_str("   ");
            }
            result.push_str(&self.this);
            ident += 1;
            if !self.children.is_empty() {
                dbg![&result];
                // result.push('\n');
            }
        }
        
        for (idx, child) in self.children.into_iter().enumerate() {
            if !(self.this.is_empty() && idx == 0) {
                result.push('\n');
            }
            result.push_str(&child.finish(ident));
        }
        // result.push('\n');
        result
    }
}

// impl From<()> for Output {
//     fn from(value: ()) -> Self {
//         Output::from("")
//     }
// }

impl From<String> for Output {
    fn from(this: String) -> Self {
        Self {
            children: Vec::new(),
            this
        }
    }
}

impl From<ColoredString> for Output {
    fn from(value: ColoredString) -> Self {
        Self::from(format!["{value}"])
    }
}

impl<T: Into<Output>, E: Into<Output>> From<Result<T, E>> for Output {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(data) => Output::from("Success".green()).with(data.into()),
            Err(err) => Output::from("Error".red()).with(err.into())
        }
    }
}

impl<E: Into<Output>> From<Result<(), E>> for Output {
    fn from(value: Result<(), E>) -> Self {
        match value {
            Ok(_) => "Success".green().into(),
            Err(err) => Output::from("Error".red()).with(err.into())
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
                #[allow(clippy::single_match)]
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
        match value {
            E::CookiesParsing(err) => Output::from("Parsing cookies").with(err.to_string()),
            E::HeaderParsing(err) => Output::from("Parsing Header").with(err.to_string()),
            E::HeadersConverting(_) => "Converting header".into(),
            E::Parsing(err) => Output::from("Parsing").with(err),
            E::SessionIdCookieNotFound => "Finding Session ID cookie in response".into(),
            E::SetCookieHeaderNotFound => "Finding Set-Cookie header in response".into(),
            E::This(err) => Output::from("").with(err),
        }
    }
}

impl From<s2rs::api::LoginParseError> for Output {
    fn from(value: s2rs::api::LoginParseError) -> Self {
        match value {
            s2rs::api::LoginParseError::EmptyArray => "Finding data in response".into(),
            s2rs::api::LoginParseError::Expected(_) => "Expected (todo!)".into()
        }
    }
}

impl From<std::io::Error> for Output {
    fn from(value: std::io::Error) -> Self {
        value.to_string().into()
    }
}

impl From<serde_json::Error> for Output {
    fn from(value: serde_json::Error) -> Self {
        value.to_string().into()
    }
}

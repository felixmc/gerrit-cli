use std::io::Error;
use exec::*;

pub type CurlResult = Result<CurlOutput, Error>;
pub struct CurlOutput {
    pub body: String,
}

impl CurlOutput {
    pub fn new (data: &str) -> CurlOutput {
        CurlOutput {
            body: data.to_owned()
        }
    }

    pub fn body_for_json (&self) -> String {
        // skip first line to bypass those security characters: )]}'
        let str_output: Box<Vec<String>> = Box::new(self.body.lines().skip(1).map(|x| x.to_string()).collect());
        str_output.join("\n")
    }

    pub fn is_unauthorized (&self) -> bool {
        match self.body.as_ref() {
            "Unauthorized" => true,
            _ => false,
        }
    }
}

pub fn get (url: &str, params: Vec<&str>) -> CurlResult {
    let args = [&params[..], &vec![url]].concat(); // wee bit hackyyy
    exec("curl", args).map(|result| CurlOutput::new(&result.output))
}

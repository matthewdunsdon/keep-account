//! keep-account is a tool to help you keep your account data organised.

#![cfg_attr(doc, warn(rustdoc::all))]
#![cfg_attr(doc, allow(rustdoc::missing_doc_code_examples))]
#![deny(missing_docs)]
#![warn(unreachable_pub)]
#![allow(clippy::disallowed_names)]

/// The application context
#[derive(Default, Debug)]
pub struct Application {}

impl Application {
    /// Performs the "general" run behaviour that is desired
    pub fn run(self) {
        println!("Hello, keep account is here to help!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


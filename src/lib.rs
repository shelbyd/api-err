#![warn(missing_docs)]

//! Errors for conveniently attaching status codes to error cases.
//!
//! ```
//! use api_err::{CategoryExt, Context};
//!
//! fn add_one(request: &str) -> api_err::Result<String> {
//!    let input = request.parse::<i64>().bad_request()?;
//!    let output = input.checked_add(1).context("Input too large").bad_request()?;
//!    Ok(output.to_string())
//! }
//! ```
//!
//! Errors without a category attached default to the "Internal Server"-like error.

mod category;

#[cfg(feature = "http")]
mod http;

#[cfg(feature = "json_rpc")]
mod json_rpc;

use std::fmt::Display;

pub use category::{Category, CategoryExt};

/// [std::result::Result] bound to [Error].
pub type Result<T> = std::result::Result<T, Error>;

/// Core error type.
///
/// Combination of [anyhow::Error] and a [Category].
pub struct Error {
    anyhow: anyhow::Error,
    category: Option<Category>,
}

impl Error {
    /// Convert into the underlying [anyhow::Error].
    pub fn into_anyhow(self) -> anyhow::Error {
        self.anyhow
    }

    /// Get a reference to the error's category (if it exists).
    pub fn category(&self) -> Option<&Category> {
        self.category.as_ref()
    }

    /// Attach additional context to the error. See [anyhow::Error::context].
    pub fn context<C>(mut self, context: C) -> Self
    where
        C: Display + Send + Sync + 'static,
    {
        self.anyhow = self.anyhow.context(context);
        self
    }

    #[cfg(feature = "http")]
    /// The HTTP status code that best corresponds to this error's category.
    pub fn http_status(&self) -> u16 {
        http::status_code(self.category.as_ref())
    }

    #[cfg(feature = "json_rpc")]
    /// The JSON-RPC status code that best corresponds to this error's category.
    pub fn json_rpc_status(&self) -> i32 {
        json_rpc::status_code(self.category.as_ref())
    }
}

impl<E> From<E> for Error
where
    anyhow::Error: From<E>,
{
    fn from(e: E) -> Self {
        Error {
            anyhow: anyhow::Error::from(e),
            category: None,
        }
    }
}

/// [anyhow::Context]-like trait for adding contexts and getting [Error]s.
pub trait Context<T, E> {
    /// See [anyhow::Context::context].
    fn context<C>(self, context: C) -> Result<T>
    where
        C: Display + Send + Sync + 'static;

    /// See [anyhow::Context::with_context].
    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T, E> Context<T, E> for std::result::Result<T, E>
where
    E: Into<Error>,
{
    fn context<C>(self, context: C) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.into().context(context)),
        }
    }

    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.into().context(f())),
        }
    }
}

impl<T> Context<T, std::convert::Infallible> for Option<T> {
    fn context<C>(self, context: C) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
    {
        match self {
            Some(t) => Ok(t),
            None => Err(anyhow::anyhow!(context.to_string()).into()),
        }
    }

    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Some(t) => Ok(t),
            None => Err(anyhow::anyhow!(f().to_string()).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_with_question_mark() {
        fn _api() -> super::Result<()> {
            "foo".parse::<usize>()?;
            Ok(())
        }
    }

    #[test]
    fn attaches_cause() {
        let err = "foo".parse::<usize>().bad_request();

        assert_eq!(err.unwrap_err().category(), Some(&Category::BadRequest));
    }

    #[test]
    fn can_attach_context() {
        let _ = "foo".parse::<usize>().bad_request().context("Some context");
    }
}

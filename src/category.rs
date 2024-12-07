use crate::Error;

/// What type of error this is. Roughly corresponds to HTTP error statuses.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
// TODO(shelbyd): Macro for cases.
pub enum Category {
    /// The client made an invalid request. Usually bad input.
    BadRequest,

    /// Fallback for custom error statuses. Will have fields based on if the `http`/`json_rpc` features are defined.
    #[non_exhaustive]
    Custom {
        /// Status code for HTTP.
        #[cfg(feature = "http")]
        http_status: u16,

        /// Status code for JSON-RPC.
        #[cfg(feature = "json_rpc")]
        json_rpc_status: i32,
    },
}

/// Convenience trait for easily adding categories to errors.
pub trait CategoryExt {
    /// The type that is returned from this trait's functions.
    type Ret;

    /// For internal use.
    fn _internal_error_mut(self, f: impl FnOnce(&mut Error)) -> Self::Ret;

    /// Convenience trait for easily adding categories to errors.
    fn with_category(self, category: Category) -> Self::Ret
    where
        Self: Sized,
    {
        self._internal_error_mut(|e| e.category = Some(category))
    }

    /// Convenience method for [Category::BadRequest].
    fn bad_request(self) -> Self::Ret
    where
        Self: Sized,
    {
        self.with_category(Category::BadRequest)
    }
}

impl<T, E> CategoryExt for Result<T, E>
where
    E: Into<Error>,
{
    type Ret = Result<T, Error>;

    fn _internal_error_mut(self, f: impl FnOnce(&mut Error)) -> Self::Ret {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                let mut e = e.into();
                f(&mut e);
                Err(e)
            }
        }
    }
}

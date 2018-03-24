//! Provides functions for error handling.
//!
//! # Examples
//!
//! Error chaining.
//!
//! ```
//!
//! # #[macro_use] extern crate mm_errors;
//!
//! use std::result::Result;
//!
//! use mm_errors::Error;
//!
//! fn level1() -> Result<u32, Error> {
//!     let _ = try_wrap!("XXX".parse::<u32>());
//!     panic!("This line is unreachable");
//! }
//!
//! fn level2() -> Result<u32, Error> {
//!     let v = try_wrap!(level1());
//!     Ok(v)
//! }
//!
//! fn level3() -> Result<u32, Error> {
//!     let v = try_wrap!(level2());
//!     Ok(v)
//! }
//!
//! # fn main() {
//!
//!     match level3() {
//!         Err(e) => println!("{}", e),
//!         Ok(_) => panic!("the function should panic!"),
//!     };
//!
//! # }
//!
//! ```
//!
//! The above code outputs following.
//!
//! ```xml
//! <error>
//!       <file>src\lib.rs</file>
//!       <line>20</line>
//!       <reason>
//!       <error>
//!           <file>src\lib.rs</file>
//!           <line>15</line>
//!           <reason>
//!               <error>
//!                   <file>src\lib.rs</file>
//!                   <line>10</line>
//!                   <reason>invalid digit found in string</reason>
//!               </error>
//!           </reason>
//!       </error>
//!       </reason>
//! </error>
//! ```
//!
//! Converts `Option::None` to `Result::Err`.
//!
//! ```
//!
//! # #[macro_use] extern crate mm_errors;
//!
//! use std::result::Result;
//!
//! use mm_errors::Error;
//!
//! fn level1() -> Result<u32, Error> {
//!     let mut v = vec![];
//!     let _ = try_opt!(v.pop(), "Stack underflow!");
//!     panic!("This line is unreachable");
//! }
//!
//! fn level2() -> Result<u32, Error> {
//!     let v = try_wrap!(level1());
//!     Ok(v)
//! }
//!
//! fn level3() -> Result<u32, Error> {
//!     let v = try_wrap!(level2());
//!     Ok(v)
//! }
//!
//! # fn main() {
//!
//!     match level3() {
//!         Err(e) => println!("{}", e),
//!         Ok(_) => panic!("the function should panic!"),
//!     };
//!
//! # }
//!
//! ```
//!
//! The above code outputs following.
//!
//! ```xml
//! <error>
//!     <file>src\lib.rs</file>
//!     <line>21</line>
//!     <reason>
//!         <error>
//!             <file>src\lib.rs</file>
//!             <line>16</line>
//!             <reason>
//!                 <error>
//!                     <file>src\lib.rs</file>
//!                     <line>11</line>
//!                     <reason>Stack underflow!</reason>
//!                 </error>
//!             </reason>
//!         </error>
//!     </reason>
//! </error>
//!
//! ```
//!
//! Creates simple error with error message.
//!
//! ```
//!
//! # #[macro_use] extern crate mm_errors;
//!
//! use std::result::Result;
//!
//! use mm_errors::Error;
//!
//! fn return_error() -> Result<u32, Error> {
//!     return new_result!("This function always returns error");
//!     panic!("This line is unreachable");
//! }
//!
//! # fn main() {
//!
//!     match return_error() {
//!         Err(e) => println!("{}", e),
//!         Ok(_) => panic!("the function should panic!"),
//!     };
//!
//! # }
//!
//! ```
//!
//! The above code outputs following.
//!
//! ```xml
//! <error>
//!     <file>src\lib.rs</file>
//!     <line>10</line>
//!     <reason>This function always returns error</reason>
//! </error>
//! ```
//!

pub mod oks;

use std::error;
use std::fmt;
use std::marker;
use std::result;

/// Holds error information.
///
///
/// See [the module level document] for detail.
///
/// [the module level document]: index.html
///
pub struct Error {
    /// File where error occurred.
    pub file: &'static str,

    /// line number where error occurred.
    pub line: u32,

    /// Error kind.
    pub kind: ErrorKind,
}

/// Error kinds.
#[derive(Debug)]
pub enum ErrorKind {
    /// Error with error message.
    String(String),

    /// Error with internal error.
    Wrapped(Box<error::Error + marker::Send + marker::Sync>),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error {
    /// Returns a new instance of `Error`.
    ///
    /// # Arguments
    ///
    /// * message - Error message.
    /// * file - File where error occurred.
    /// * line - Line number where error occurred.
    ///
    pub fn new(message: &str, file: &'static str, line: u32) -> Error {
        Error {
            file,
            line,
            kind: ErrorKind::String(message.to_string()),
        }
    }

    /// Returns a new instance of `Error`
    ///
    /// The return value holds `e` as inner error.
    ///
    /// # Arguments
    ///
    /// * e - Inner error.
    /// * file - File where error occurred.
    /// * line - Line number where error occurred.
    ///
    pub fn wrap<T>(e: T, file: &'static str, line: u32) -> Error
        where T: Into<Box<error::Error + marker::Send + marker::Sync>> {
        Error {
            file,
            line,
            kind: ErrorKind::Wrapped(e.into()),
        }
    }

    fn format_xml(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<error>")?;
        write!(f, "<file>{}", self.file)?;
        write!(f, "</file>")?;
        write!(f, "<line>{}", self.line)?;
        write!(f, "</line>")?;
        match self.kind {
            ErrorKind::String(ref s) => {
                write!(f, "<reason>{}", s)?;
                write!(f, "</reason>")?;
            },
            ErrorKind::Wrapped(ref e) => {
                write!(f, "<reason>{}", e)?;
                write!(f, "</reason>")?;
            }
        }
        write!(f, "</error>")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "font processing error"
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.kind {
            ErrorKind::String(..) => None,
            ErrorKind::Wrapped(ref e) => e.cause(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format_xml(f)
    }
}

/// Similar to `try!` macro, but this returns an `Error` instance wrapping the internal error.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate mm_errors;
///
/// use std::result::Result;
///
/// use mm_errors::Error;
///
/// fn return_err() -> Result<u32, Error> {
///     let _ = try_wrap!("XXX".parse::<u32>());
///     panic!("This line is unreachable");
/// }
///
///
/// # fn main() {
///
///     match return_err() {
///         Err(e) => match e {
///             Error{file, line, kind} => {
///                 println!("{}", file);
///                 println!("{}", line);
///                 println!("{:?}", kind);
///             }
///         },
///         Ok(_) => panic!("the function should panic!"),
///     };
///
/// # }
/// ```
///
#[macro_export]
macro_rules! try_wrap {
    ($exp:expr) => ({
        match $exp {
            Ok(x) => x,
            Err(e) => return Err($crate::Error::wrap(e, file!(), line!())),
        }
    })
}

/// Returns a new instance of `Error`.
#[macro_export]
macro_rules! new_error {
    ($message:expr) => ({
        $crate::Error::new($message, file!(), line!())
    })
}

/// Returns a new `Result::Err`.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate mm_errors;
///
/// use std::result::Result;
///
/// use mm_errors::{Error, ErrorKind};
///
/// fn return_error() -> Result<(), Error> {
///     return new_result!("This function always returns an error.");
/// }
///
/// # fn main() {
///     match return_error() {
///         Err(e) => match e {
///             Error{file, line, kind} => match kind {
///                 ErrorKind::String(s) => {
///                     println!("{}", file);
///                     println!("{}", line);
///                     println!("{}", s);
///                 },
///                 ErrorKind::Wrapped(_) => (),
///             },
///         },
///         Ok(_) => panic!("The function never success"),
///     }
/// # }
///
/// ```
///
#[macro_export]
macro_rules! new_result {
    ($message:expr) => ({
        Err(new_error!($message))
    })
}

/// Returns a new `Result::Err` if the expression's value is `None`.
///
/// # Examples
///
/// ```
///
/// # #[macro_use] extern crate mm_errors;
///
/// use std::result::Result;
///
/// use mm_errors::{Error, ErrorKind};
///
/// fn return_none() -> Result<(), Error> {
///     try_opt!(None, "This function returns always Err")
/// }
///
/// # fn main() {
///     match return_none() {
///         Err(e) => match e {
///             Error{file, line, kind} => match kind {
///                 ErrorKind::String(s) => {
///                     println!("{}", file);
///                     println!("{}", line);
///                     println!("{}", s);
///                 },
///                 ErrorKind::Wrapped(_) => (),
///             },
///         },
///         Ok(_) => panic!("The function never success"),
///     }
/// # }
///
/// ```
///
#[macro_export]
macro_rules! try_opt {
    ($exp:expr, $message:expr) => ({
        match $exp {
            Some(x) => x,
            None => return Err($crate::Error::new($message, file!(), line!())),
        }
    })
}

#[macro_export]
macro_rules! try_opt_ref {
    ($exp:expr, $message:expr) => ({
        match $exp {
            Some(ref x) => x,
            None => return Err($crate::Error::new($message, file!(), line!())),
        }
    })
}

/// Alias for `Result`.
pub type Result<T> = result::Result<T, Error>;

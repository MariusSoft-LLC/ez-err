//! Core code.

/// A custom [`std::result::Result<T, E>`] with the [`EzError`] type. This is used for
/// passing down errors.
pub type Result<T> = std::result::Result<T, EzError>;

/// Throws an error and returns early.
/// Shortcut for `Err(EzError::message("some error")).loc(flc!())?`
#[macro_export]
macro_rules! bail {
    ($($args:tt)*) => {
        Err(EzError::message(&::std::format_args!($($args)*).to_string())).loc(flc!())?
    };
}

/// The flc (File-Line-Column) macro expands to a [`ConstLocation`], which describes
/// a location in the source code.
#[macro_export]
macro_rules! flc {
    () => {{
        #[cfg(not(feature = "no_stacktrace"))]
        const LOC: ConstLocation = ConstLocation::new(file!(), line!(), column!());
        #[cfg(feature = "no_stacktrace")]
        const LOC: ConstLocation = ConstLocation::new("", 0, 0);
        &LOC
    }};
}

/// Execute the provided function and catch any errors. This is
/// useful for closures where no error type can be returned by default.
pub fn handle<F, R>(func: F) -> Option<R>
where
    F: FnOnce() -> Result<R>,
{
    func().handle()
}

/// Stores information about the error and is used for proper error
/// output to the Unity console.
#[derive(Debug, PartialEq)]
pub struct EzError {
    inner: Box<EzErrorInner>,
}

#[derive(Debug, PartialEq)]
struct EzErrorInner {
    ty: ErrorType,
    #[cfg(not(feature = "no_stacktrace"))]
    frames: Vec<&'static ConstLocation>,
}

impl EzError {
    /// Constructs a new `EzError` with the given error type.
    pub fn new(ty: ErrorType) -> EzError {
        #[cfg(not(feature = "no_stacktrace"))]
        return EzError {
            inner: Box::new(EzErrorInner {
                ty,
                frames: Vec::new(),
            }),
        };
        #[cfg(feature = "no_stacktrace")]
        return EzError {
            inner: Box::new(EzErrorInner { ty }),
        };
    }

    /// Constructs a new [`EzError`] with the type [`ErrorType::Message`]
    /// using the specified message.
    pub fn message(msg: &str) -> EzError {
        EzError::new(ErrorType::Message(msg.to_owned()))
    }

    /// Constructs a new [`EzError`] with the type [`ErrorType::Custom`].
    /// The code can be used to store arbitrary extra information.
    pub fn custom(code: u32, name: String, message: String) -> EzError {
        EzError::new(ErrorType::Custom {
            code,
            name,
            message,
        })
    }

    /// Adds a new frame to the `EzError` and sets `file_name`
    /// to `file` and `line_number` to `line`.
    pub fn add_frame(&mut self, loc: &'static ConstLocation) {
        self.inner.frames.push(loc);
    }

    /// Merges the other error into this by adding the frames of it to this.
    pub fn with(mut self, other: EzError) -> Self {
        self.inner.frames.extend_from_slice(&other.inner.frames);
        self
    }

    /// Returns the type of the error.
    pub fn ty(&self) -> &ErrorType {
        &self.inner.ty
    }

    /// Returns the stack frames of the error.
    #[cfg(not(feature = "no_stacktrace"))]
    pub fn frames(&self) -> &[&'static ConstLocation] {
        &self.inner.frames
    }
}

impl<E> From<E> for EzError
where
    E: std::fmt::Display,
{
    fn from(err: E) -> Self {
        EzError::new(ErrorType::Internal(format!("{}", err)))
    }
}

/// The different error types that can occur.
#[derive(Debug, PartialEq)]
pub enum ErrorType {
    /// Wraps an internal error that is not compatible with the
    /// custom error types by default.
    Internal(String),

    /// An error that occured where an `Option` was `None`.
    NoneOption,

    /// An error that occured when an array index was outside of the
    /// valid range.
    IndexOutOfBounds(usize, usize),

    /// The range is larger than the array.
    RangeOutOfBounds(usize, usize, usize),

    /// The given range index is not valid (`end < start`).
    InvalidRange,

    /// A custom error with an attached message.
    Message(String),

    /// No error specified.
    Custom {
        /// A custom message code used for storing custom information.
        code: u32,
        /// The name of the error.
        name: String,
        /// The message of the error.
        message: String,
    },
}

impl ErrorType {
    /// Formats the error type into a String for console output.
    pub fn format(self) -> String {
        match self {
            ErrorType::Internal(msg) => msg,
            ErrorType::NoneOption => format!("Option was none"),
            ErrorType::IndexOutOfBounds(idx, len) => {
                format!("Index {} was outside of the range 0..{}", idx, len)
            }
            ErrorType::RangeOutOfBounds(start, end, len) => {
                format!(
                    "Range {}..{} was larger than the array range 0..{}",
                    start, end, len
                )
            }
            ErrorType::InvalidRange => {
                "The provided range was invalid (end < start or X..=usize::MAX)".into()
            }
            ErrorType::Message(msg) => msg,
            ErrorType::Custom { message, .. } => message,
        }
    }

    /// Returns the name of the `ErrorType` that should be included
    /// in the stacktrace.
    pub fn name(&self) -> &str {
        match self {
            ErrorType::Internal(_) => "WrappedInternal",
            ErrorType::NoneOption => "NoneOption",
            ErrorType::IndexOutOfBounds(_, _) => "IndexOutOfBounds",
            ErrorType::RangeOutOfBounds(_, _, _) => "RangeOutOfBounds",
            ErrorType::InvalidRange => "InvalidRange",
            ErrorType::Message(_) => "Message",
            ErrorType::Custom { name, .. } => &name,
        }
    }
}

/// Information about the location in a source file in a constant context.
#[derive(Debug, PartialEq)]
pub struct ConstLocation {
    /// The file of the location.
    pub file: &'static str,
    /// The line of the location.
    pub line: u32,
    /// The column of the location.
    pub column: u32,
}

impl ConstLocation {
    /// Creates a new [`ConstLocation`] using the given file and line.
    pub const fn new(file: &'static str, line: u32, column: u32) -> ConstLocation {
        ConstLocation { file, line, column }
    }
}

/// Extension for `Result<T>` to allow for custom error handling.
pub trait LocData<T> {
    /// The return type of `add_info`. This can be used to convert
    /// between different error types.
    type Result;

    /// Adds a new frame info to the [`Result<T>`]. This only happens
    /// when the [`Result<T>`] is [`Err(T)`]. Commonly used with the [`flc!`] macro.
    fn loc(self, flc: &'static ConstLocation) -> Self::Result;
}

/// Extension for `Result<T>` to allow for custom error handling.
pub trait Handle<T> {
    /// Handles the result. If it contains an error a backtrace is
    /// created and the error is printed to the console.
    fn handle(self) -> Option<T>;

    /// Handles the result or panics if it is [`Err`]. If it contains
    /// an error a backtrace is created and the error is printed to the console.
    fn handle_or_panic(self) -> T;
}

impl<T> LocData<T> for Result<T> {
    type Result = Result<T>;

    #[inline(always)]
    fn loc(mut self, loc: &'static ConstLocation) -> Self::Result {
        if let Err(err) = &mut self {
            err.add_frame(loc);
        }

        self
    }
}

impl<T> Handle<T> for Result<T> {
    fn handle(self) -> Option<T> {
        fn inner(e: EzError) {
            let e = e.inner;

            #[cfg(not(feature = "no_stacktrace"))]
            let trace = {
                let mut s = String::with_capacity(1024);
                s.push_str("Stacktrace:\n");
                for frame in e.frames {
                    s.push_str(frame.file);
                    s.push(':');
                    s.push_str(&frame.line.to_string());
                    s.push(':');
                    s.push_str(&frame.column.to_string());
                    s.push('\n');
                }
                s
            };
            #[cfg(feature = "no_stacktrace")]
            let trace = "";

            let name = e.ty.name().to_owned();
            let message = e.ty.format();

            #[cfg(feature = "log")]
            log::error!("Error {}: {}\n\n{}", name, message, trace);
            #[cfg(not(feature = "log"))]
            println!("Error {}: {}\n\n{}", name, message, trace);
        }

        match self {
            Ok(v) => Some(v),
            Err(e) => {
                inner(e);
                None
            }
        }
    }
    fn handle_or_panic(self) -> T {
        match self.handle() {
            Some(v) => v,
            None => panic!(),
        }
    }
}

impl<T, E> LocData<T> for std::result::Result<T, E>
where
    E: std::fmt::Display,
{
    type Result = Result<T>;

    #[inline(always)]
    fn loc(self, loc: &'static ConstLocation) -> Self::Result {
        #[cfg(not(feature = "no_stacktrace"))]
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err({
                let mut err: EzError = e.into();
                err.add_frame(loc);
                err
            }),
        }
        #[cfg(feature = "no_stacktrace")]
        self
    }
}

impl<T> LocData<T> for Option<T> {
    type Result = Result<T>;

    #[inline(always)]
    fn loc(self, loc: &'static ConstLocation) -> Self::Result {
        #[cfg(not(feature = "no_stacktrace"))]
        match self {
            Some(v) => Ok(v),
            None => Err({
                let mut err = EzError::new(ErrorType::NoneOption);
                err.add_frame(loc);
                err
            }),
        }
        #[cfg(feature = "no_stacktrace")]
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_info() {
        let err: Result<()> = Err(EzError::message("test")).loc(flc!());
        let (file, line) = (file!(), line!());

        let loc = err.err().unwrap().frames()[0];
        assert_eq!(loc.file, file);
        assert_eq!(loc.line, line - 1);
        assert_eq!(loc.column, 65);
    }

    #[test]
    fn correct_bail() {
        let inner_line = line!() + 2;
        fn inner() -> Result<()> {
            bail!("bailed");

            Ok(())
        }

        let err = inner().err().unwrap();
        assert_eq!(&ErrorType::Message("bailed".into()), err.ty());
        assert_eq!(inner_line, err.frames()[0].line);
    }
}

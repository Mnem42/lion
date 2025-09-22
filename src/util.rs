/// Like try, but for iterators that return [`Option<Result<_, _>>`].
///
/// NOTE: ripped out of the walkdir crate
///
/// [`Option<Result<_, _>>`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
// Unused for now
#[macro_export]
macro_rules! itry {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(err) => return Some(Err(From::from(err))),
        }
    };
}

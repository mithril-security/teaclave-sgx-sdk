/// Terminates the process in an abnormal fashion.
///
/// The function will never return and will immediately terminate the current
/// process in a platform specific "abnormal" manner.
///
/// Note that because this function never returns, and that it terminates the
/// process, no destructors on the current stack or any other thread's stack
/// will be run.
///
/// This is in contrast to the default behaviour of [`panic!`] which unwinds
/// the current thread's stack and calls all destructors.
/// When `panic="abort"` is set, either as an argument to `rustc` or in a
/// crate's Cargo.toml, [`panic!`] and `abort` are similar. However,
/// [`panic!`] will still call the [panic hook] while `abort` will not.
///
/// If a clean shutdown is needed it is recommended to only call
/// this function at a known point where there are no more destructors left
/// to run.
///
/// # Examples
///
/// ```no_run
/// use std::process;
///
/// fn main() {
///     println!("aborting");
///
///     process::abort();
///
///     // execution never gets here
/// }
/// ```
///
/// The `abort` function terminates the process, so the destructor will not
/// get run on the example below:
///
/// ```no_run
/// use std::process;
///
/// struct HasDrop;
///
/// impl Drop for HasDrop {
///     fn drop(&mut self) {
///         println!("This will never be printed!");
///     }
/// }
///
/// fn main() {
///     let _x = HasDrop;
///     process::abort();
///     // the destructor implemented for HasDrop will never get run
/// }
/// ```
///
/// [panic hook]: crate::panic::set_hook
#[cold]
pub fn abort() -> ! {
    unsafe { crate::sys::abort_internal(); }
}

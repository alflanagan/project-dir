//! Module to handle writing shell script commands to stdout.
//!
//! We need for the program to affect the environment of the shell from which it was run.
//! Problem is, there are limited capabilities to do that (this is on purpose; they'd be a huge
//! security hole. So is our solution, unless you trust this program).
//! Probably the easiest way to handle this is to write shell commands to the stdout stream.
//! Then the user can execute the command `eval "$(project_dir)"` and those commands will be run
//! in the context of the shell where eval was invoked.
use rusqlite::Connection;

pub fn write_to_shell(conn: &Connection) {}

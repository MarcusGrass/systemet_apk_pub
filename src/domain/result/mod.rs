error_chain! {
    // The type defined for this error. These are the conventional
    // and recommended names, but they can be arbitrarily chosen.
    //
    // It is also possible to leave this section out entirely, or
    // leave it empty, and these names will be used automatically.
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    // Without the `Result` wrapper:
    //
    // types {
    //     Error, ErrorKind, ResultExt;
    // }

    // Automatic conversions between this error chain and other
    // error types not defined by the `error_chain!`. These will be
    // wrapped in a new error with, in the first case, the
    // `ErrorKind::Fmt` variant. The description and cause will
    // forward to the description and cause of the original error.
    //
    // Optionally, some attributes can be added to a variant.
    //
    // This section can be empty.
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        Reqwest(::reqwest::Error);
        Rusqlite(::rusqlite::Error);
        Time(::std::time::SystemTimeError);
        Json(::serde_json::Error);
    }

    // Define additional `ErrorKind` variants.  Define custom responses with the
    // `description` and `display` calls.
    errors {
        InvalidToolchainName(t: String) {
            description("invalid toolchain name")
            display("invalid toolchain name: '{}'", t)
        }

        // You can also add commas after description/display.
        // This may work better with some editor auto-indentation modes:
        UnknownToolchainVersion(v: String) {
            description("unknown toolchain version"), // note the ,
            display("unknown toolchain version: '{}'", v), // trailing comma is allowed
        }
    }

    // If this annotation is left off, a variant `Msg(s: String)` will be added, and `From`
    // impls will be provided for `String` and `&str`
    skip_msg_variant
}

pub fn fmt_backtrace(err: &Error) -> String {
    let full = format!("{:?}", err);
    let split = full.split("\n");
    let mut sites = Vec::new();
    for (_i, s) in split.enumerate() {
        if s.contains("src") {
            let line = s.trim().replace("at ", "");
            if line.starts_with("src") {
                sites.push(line);
            }
        }
    }
    let mut res = format!("{}\n", err);
    for (i, s) in sites.iter().enumerate() {
        res = format!("{}{}", res, s);
        if i != sites.len() {
            res = format!("{}\n", res);
        }
    }
    res

}

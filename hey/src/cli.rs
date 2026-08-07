/// Parsed command-line invocation.
pub struct Args {
    pub help: bool,
    pub version: bool,
    pub follow_up: bool,
    pub buddy: Option<String>,
}

/// Parses `args` (the process arguments, excluding the program name).
///
/// The first argument that isn't a recognized flag is treated as the buddy
/// name (see the `buddies` section of `hey.toml`). A second such argument is
/// rejected as an unexpected argument.
pub fn parse(args: &[String]) -> Result<Args, String> {
    let mut help = false;
    let mut version = false;
    let mut follow_up = false;
    let mut buddy = None;

    for arg in args {
        match arg.as_str() {
            "--help" | "-h" => help = true,
            "--version" | "-V" => version = true,
            "--follow-up" | "-f" => follow_up = true,
            other if other.starts_with('-') => {
                return Err(format!("Unrecognized option '{other}'"));
            }
            other => {
                if buddy.is_some() {
                    return Err(format!("Unexpected argument '{other}'"));
                }
                buddy = Some(other.to_string());
            }
        }
    }

    Ok(Args {
        help,
        version,
        follow_up,
        buddy,
    })
}

pub fn print_help() {
    println!(
        "hey {version} - a simple command-line AI assistant

USAGE:
    hey [BUDDY] [OPTIONS]

ARGS:
    <BUDDY>               Name of the buddy to ask (case-insensitive, configured in hey.toml).
                          Uses the default buddy when omitted.

OPTIONS:
    -f, --follow-up      Continue the previous conversation with a follow-up question
    -V, --version        Print version information
    -h, --help           Print this help message

ENVIRONMENT:
    OPENROUTER_API_KEY   Required. Your OpenRouter.ai API key",
        version = env!("CARGO_PKG_VERSION")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parses_no_args() {
        let parsed = parse(&args(&[])).unwrap();

        assert!(!parsed.help);
        assert!(!parsed.version);
        assert!(!parsed.follow_up);
        assert_eq!(parsed.buddy, None);
    }

    #[test]
    fn parses_buddy_name() {
        let parsed = parse(&args(&["John"])).unwrap();

        assert_eq!(parsed.buddy.as_deref(), Some("John"));
    }

    #[test]
    fn parses_buddy_name_with_follow_up_flag() {
        let parsed = parse(&args(&["John", "-f"])).unwrap();

        assert_eq!(parsed.buddy.as_deref(), Some("John"));
        assert!(parsed.follow_up);
    }

    #[test]
    fn parses_flags_before_buddy_name() {
        let parsed = parse(&args(&["--follow-up", "John"])).unwrap();

        assert_eq!(parsed.buddy.as_deref(), Some("John"));
        assert!(parsed.follow_up);
    }

    #[test]
    fn parses_help_and_version_flags() {
        let parsed = parse(&args(&["--help"])).unwrap();
        assert!(parsed.help);

        let parsed = parse(&args(&["-V"])).unwrap();
        assert!(parsed.version);
    }

    #[test]
    fn rejects_second_positional_argument() {
        let result = parse(&args(&["John", "Tom"]));

        assert!(result.is_err());
    }

    #[test]
    fn rejects_unrecognized_flag() {
        let result = parse(&args(&["--bogus"]));

        assert!(result.is_err());
    }
}

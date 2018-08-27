#[doc(hidden)]
#[macro_export]
macro_rules! _cli_login {
    ($registration:expr, $url_label:expr, $prompt:expr) => {{
        use std::io::{self, Read, Write};
        use $crate::{Mastodon, Result};
        let mut s = String::new();
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();

        let url = $registration.authorize_url()?;
        write!(&mut stdout, "{}: {}", $url_label, url)?;
        stdout.flush();

        write!(&mut stdout, "{}", $prompt)?;
        stdout.flush();

        stdin.read_line(&mut s)?;

        let code = s.trim();
        let client = $registration.complete(code.to_string())?;

        Ok(client) as Result<Mastodon>
    }};
}

#[macro_export]
/// Generates code to complete a user's registration on the command line
macro_rules! cli_login {
    ($registration:expr,url_label: $url_label:expr) => {
        _cli_login!(
            $registration,
            $url_label,
            "Paste the returned authorization code: "
        );
    };
    ($registration:expr,prompt: $prompt:expr) => {
        _cli_login!(
            $registration,
            "Click this link to authorize on Mastodon: {}",
            $prompt
        );
    };
    ($registration:expr,url_label: $url_label:expr,prompt: $prompt:expr) => {
        _cli_login!($registration, $url_label, $prompt);
    };
    ($registration:expr,prompt: $prompt:expr,url_label: $url_label:expr) => {
        _cli_login!($registration, $url_label, $prompt);
    };
    ($registration:expr) => {
        _cli_login!(
            $registration,
            "Click this link to authorize on Mastodon: {}",
            "Paste the returned authorization code: "
        );
    };
}

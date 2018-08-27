#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]

#[cfg_attr(any(feature = "toml", feature = "all"), macro_use)]
extern crate elefren;

#[cfg(feature = "toml")]
mod register;

#[cfg_attr(feature = "toml", macro_use)]
use elefren::MastodonClient;
use std::error;

#[cfg(feature = "toml")]
fn main() -> Result<(), Box<error::Error>> {
    let mastodon = register::get_mastodon_data()?;
    let input = register::read_line("Enter the account id you'd like to follow: ")?;
    let new_follow = mastodon.follow(input.trim().parse()?)?;

    println!("{:#?}", new_follow);

    Ok(())
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!(
        "examples require the `toml` feature, run this command for this example:\n\ncargo run \
         --example follow_profile --features toml\n"
    );
}

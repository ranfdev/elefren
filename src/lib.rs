//! # Elefren: API Wrapper around the Mastodon API.
//!
//! Most of the api is documented on [Mastodon's
//! github](https://github.com/tootsuite/mastodon/blob/master/docs/Using-the-API/API.md#tag)
//!
//! ```no_run
//! # extern crate elefren;
//! # fn main() {
//! #    try().unwrap();
//! # }
//! # fn try() -> elefren::Result<()> {
//! use elefren::prelude::*;
//!
//! let registration = Registration::new("https://mastodon.social")
//!     .client_name("elefren_test")
//!     .build()?;
//! let url = registration.authorize_url()?;
//! // Here you now need to open the url in the browser
//! // And handle a the redirect url coming back with the code.
//! let code = String::from("RETURNED_FROM_BROWSER");
//! let mastodon = registration.complete(&code)?;
//!
//! println!("{:?}", mastodon.get_home_timeline()?.initial_items);
//! # Ok(())
//! # }
//! ```

#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(test, deny(missing_docs))]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate doc_comment;
#[macro_use]
extern crate serde_json as json;
extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate try_from;
extern crate url;

#[cfg(feature = "toml")]
extern crate toml as tomlcrate;

#[cfg(test)]
extern crate tempfile;

use std::{borrow::Cow, ops};

use reqwest::{
    header::{Authorization, Bearer, Headers},
    Client,
    RequestBuilder,
    Response,
};

use entities::prelude::*;
use http_send::{HttpSend, HttpSender};
use page::Page;

pub use data::Data;
pub use errors::{ApiError, Error, Result};
pub use registration::Registration;
pub use requests::statuses::StatusesRequest;
pub use status_builder::StatusBuilder;

/// Registering your App
pub mod apps;
/// Contains the struct that holds the client auth data
pub mod data;
/// Entities returned from the API
pub mod entities;
/// Errors
pub mod errors;
/// Collection of helpers for serializing/deserializing `Data` objects
pub mod helpers;
/// Contains trait for converting `reqwest::Request`s to `reqwest::Response`s
pub mod http_send;
/// Handling multiple pages of entities.
pub mod page;
/// Registering your app.
pub mod registration;
/// Requests
pub mod requests;
/// Constructing a status
pub mod status_builder;
#[macro_use]
mod macros;
/// Automatically import the things you need
pub mod prelude {
    pub use apps::Scopes;
    pub use Data;
    pub use Mastodon;
    pub use MastodonClient;
    pub use Registration;
    pub use StatusBuilder;
    pub use StatusesRequest;
}

/// Your mastodon application client, handles all requests to and from Mastodon.
#[derive(Clone, Debug)]
pub struct Mastodon<H: HttpSend = HttpSender> {
    client: Client,
    http_sender: H,
    headers: Headers,
    /// Raw data about your mastodon instance.
    pub data: Data,
}

/// Represents the set of methods that a Mastodon Client can do, so that
/// implementations might be swapped out for testing
#[allow(unused)]
pub trait MastodonClient<H: HttpSend = HttpSender> {
    fn favourites(&self) -> Result<Page<Status, H>> {
        unimplemented!("This method was not implemented");
    }
    fn blocks(&self) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    fn domain_blocks(&self) -> Result<Page<String, H>> {
        unimplemented!("This method was not implemented");
    }
    fn follow_requests(&self) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    fn get_home_timeline(&self) -> Result<Page<Status, H>> {
        unimplemented!("This method was not implemented");
    }
    fn get_emojis(&self) -> Result<Page<Emoji, H>> {
        unimplemented!("This method was not implemented");
    }
    fn mutes(&self) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    fn notifications(&self) -> Result<Page<Notification, H>> {
        unimplemented!("This method was not implemented");
    }
    fn reports(&self) -> Result<Page<Report, H>> {
        unimplemented!("This method was not implemented");
    }
    fn followers(&self, id: &str) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    fn following(&self) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    fn reblogged_by(&self) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    fn favourited_by(&self) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    fn unblock_domain(&self, domain: String) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    fn instance(&self) -> Result<Instance> {
        unimplemented!("This method was not implemented");
    }
    fn verify_credentials(&self) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    fn report(&self, account_id: &str, status_ids: Vec<&str>, comment: String) -> Result<Report> {
        unimplemented!("This method was not implemented");
    }
    fn block_domain(&self, domain: String) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    fn authorize_follow_request(&self, id: &str) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    fn reject_follow_request(&self, id: &str) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    fn search(&self, q: String, resolve: bool) -> Result<SearchResult> {
        unimplemented!("This method was not implemented");
    }
    fn follows(&self, uri: Cow<'static, str>) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    fn media(&self, file: Cow<'static, str>) -> Result<Attachment> {
        unimplemented!("This method was not implemented");
    }
    fn clear_notifications(&self) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    fn get_account(&self, id: u64) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    fn follow(&self, id: u64) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    fn unfollow(&self, id: u64) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    fn block(&self, id: u64) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    fn unblock(&self, id: u64) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    fn mute(&self, id: u64) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    fn unmute(&self, id: u64) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    fn get_notification(&self, id: u64) -> Result<Notification> {
        unimplemented!("This method was not implemented");
    }
    fn get_status(&self, id: u64) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    fn get_context(&self, id: u64) -> Result<Context> {
        unimplemented!("This method was not implemented");
    }
    fn get_card(&self, id: u64) -> Result<Card> {
        unimplemented!("This method was not implemented");
    }
    fn reblog(&self, id: u64) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    fn unreblog(&self, id: u64) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    fn favourite(&self, id: u64) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    fn unfavourite(&self, id: u64) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    fn delete_status(&self, id: u64) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    fn update_credentials(&self, changes: CredentialsBuilder) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    fn new_status(&self, status: StatusBuilder) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    fn get_public_timeline(&self, local: bool) -> Result<Vec<Status>> {
        unimplemented!("This method was not implemented");
    }
    fn get_tagged_timeline(&self, hashtag: String, local: bool) -> Result<Vec<Status>> {
        unimplemented!("This method was not implemented");
    }
    fn statuses<'a, 'b: 'a, S>(&'b self, id: &'b str, request: S) -> Result<Page<Status, H>>
    where
        S: Into<Option<StatusesRequest<'a>>>,
    {
        unimplemented!("This method was not implemented");
    }
    fn relationships(&self, ids: &[&str]) -> Result<Page<Relationship, H>> {
        unimplemented!("This method was not implemented");
    }
    fn search_accounts(
        &self,
        query: &str,
        limit: Option<u64>,
        following: bool,
    ) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
}

impl<H: HttpSend> Mastodon<H> {
    methods![get, post, delete,];

    fn route(&self, url: &str) -> String {
        format!("{}{}", self.base, url)
    }

    pub(crate) fn send(&self, req: &mut RequestBuilder) -> Result<Response> {
        Ok(self
            .http_sender
            .send(&self.client, req.headers(self.headers.clone()))?)
    }
}

impl From<Data> for Mastodon<HttpSender> {
    /// Creates a mastodon instance from the data struct.
    fn from(data: Data) -> Mastodon<HttpSender> {
        let mut builder = MastodonBuilder::new(HttpSender);
        builder.data(data);
        builder
            .build()
            .expect("We know `data` is present, so this should be fine")
    }
}

impl<H: HttpSend> MastodonClient<H> for Mastodon<H> {
    paged_routes! {
        (get) favourites: "favourites" => Status,
        (get) blocks: "blocks" => Account,
        (get) domain_blocks: "domain_blocks" => String,
        (get) follow_requests: "follow_requests" => Account,
        (get) get_home_timeline: "timelines/home" => Status,
        (get) get_emojis: "custom_emojis" => Emoji,
        (get) mutes: "mutes" => Account,
        (get) notifications: "notifications" => Notification,
        (get) reports: "reports" => Report,
    }

    paged_routes_with_id! {
        (get) followers: "accounts/{}/followers" => Account,
        (get) following: "accounts/{}/following" => Account,
        (get) reblogged_by: "statuses/{}/reblogged_by" => Account,
        (get) favourited_by: "statuses/{}/favourited_by" => Account,
    }

    route! {
        (delete (domain: String,)) unblock_domain: "domain_blocks" => Empty,
        (get) instance: "instance" => Instance,
        (get) verify_credentials: "accounts/verify_credentials" => Account,
        (post (account_id: &str, status_ids: Vec<&str>, comment: String,)) report: "reports" => Report,
        (post (domain: String,)) block_domain: "domain_blocks" => Empty,
        (post (id: &str,)) authorize_follow_request: "accounts/follow_requests/authorize" => Empty,
        (post (id: &str,)) reject_follow_request: "accounts/follow_requests/reject" => Empty,
        (post (q: String, resolve: bool,)) search: "search" => SearchResult,
        (post (uri: Cow<'static, str>,)) follows: "follows" => Account,
        (post multipart (file: Cow<'static, str>,)) media: "media" => Attachment,
        (post) clear_notifications: "notifications/clear" => Empty,
    }

    route_id! {
        (get) get_account: "accounts/{}" => Account,
        (post) follow: "accounts/{}/follow" => Account,
        (post) unfollow: "accounts/{}/unfollow" => Account,
        (get) block: "accounts/{}/block" => Account,
        (get) unblock: "accounts/{}/unblock" => Account,
        (get) mute: "accounts/{}/mute" => Account,
        (get) unmute: "accounts/{}/unmute" => Account,
        (get) get_notification: "notifications/{}" => Notification,
        (get) get_status: "statuses/{}" => Status,
        (get) get_context: "statuses/{}/context" => Context,
        (get) get_card: "statuses/{}/card" => Card,
        (post) reblog: "statuses/{}/reblog" => Status,
        (post) unreblog: "statuses/{}/unreblog" => Status,
        (post) favourite: "statuses/{}/favourite" => Status,
        (post) unfavourite: "statuses/{}/unfavourite" => Status,
        (delete) delete_status: "statuses/{}" => Empty,
    }

    fn update_credentials(&self, changes: CredentialsBuilder) -> Result<Account> {
        let url = self.route("/api/v1/accounts/update_credentials");
        let response = self.send(self.client.patch(&url).multipart(changes.into_form()?))?;

        let status = response.status().clone();

        if status.is_client_error() {
            return Err(Error::Client(status));
        } else if status.is_server_error() {
            return Err(Error::Server(status));
        }

        deserialise(response)
    }

    /// Post a new status to the account.
    fn new_status(&self, status: StatusBuilder) -> Result<Status> {
        let response = self.send(
            self.client
                .post(&self.route("/api/v1/statuses"))
                .json(&status),
        )?;

        deserialise(response)
    }

    /// Get the federated timeline for the instance.
    fn get_public_timeline(&self, local: bool) -> Result<Vec<Status>> {
        let mut url = self.route("/api/v1/timelines/public");

        if local {
            url += "?local=1";
        }

        self.get(url)
    }

    /// Get timeline filtered by a hashtag(eg. `#coffee`) either locally or
    /// federated.
    fn get_tagged_timeline(&self, hashtag: String, local: bool) -> Result<Vec<Status>> {
        let mut url = self.route("/api/v1/timelines/tag/");
        url += &hashtag;

        if local {
            url += "?local=1";
        }

        self.get(url)
    }

    /// Get statuses of a single account by id. Optionally only with pictures
    /// and or excluding replies.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # extern crate elefren;
    /// # use elefren::prelude::*;
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # let data = Data {
    /// #   base: "".into(),
    /// #   client_id: "".into(),
    /// #   client_secret: "".into(),
    /// #   redirect: "".into(),
    /// #   token: "".into(),
    /// # };
    /// let client = Mastodon::from(data);
    /// let statuses = client.statuses("user-id", None)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```no_run
    /// # extern crate elefren;
    /// # use elefren::prelude::*;
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # let data = Data {
    /// #   base: "".into(),
    /// #   client_id: "".into(),
    /// #   client_secret: "".into(),
    /// #   redirect: "".into(),
    /// #   token: "".into(),
    /// # };
    /// let client = Mastodon::from(data);
    /// let request = StatusesRequest::default().only_media();
    /// let statuses = client.statuses("user-id", request)?;
    /// # Ok(())
    /// # }
    /// ```
    fn statuses<'a, 'b: 'a, S>(&'b self, id: &'b str, request: S) -> Result<Page<Status, H>>
    where
        S: Into<Option<StatusesRequest<'a>>>,
    {
        let mut url = format!("{}/api/v1/accounts/{}/statuses", self.base, id);

        if let Some(request) = request.into() {
            url = format!("{}{}", url, request.to_querystring());
        }

        let response = self.send(&mut self.client.get(&url))?;

        Page::new(self, response)
    }

    /// Returns the client account's relationship to a list of other accounts.
    /// Such as whether they follow them or vice versa.
    fn relationships(&self, ids: &[&str]) -> Result<Page<Relationship, H>> {
        let mut url = self.route("/api/v1/accounts/relationships?");

        if ids.len() == 1 {
            url += "id=";
            url += &ids[0];
        } else {
            for id in ids {
                url += "id[]=";
                url += &id;
                url += "&";
            }
            url.pop();
        }

        let response = self.send(&mut self.client.get(&url))?;

        Page::new(self, response)
    }

    /// Search for accounts by their name.
    /// Will lookup an account remotely if the search term is in the
    /// `username@domain` format and not yet in the database.
    fn search_accounts(
        &self,
        query: &str,
        limit: Option<u64>,
        following: bool,
    ) -> Result<Page<Account, H>> {
        let url = format!(
            "{}/api/v1/accounts/search?q={}&limit={}&following={}",
            self.base,
            query,
            limit.unwrap_or(40),
            following
        );

        let response = self.send(&mut self.client.get(&url))?;

        Page::new(self, response)
    }
}

impl<H: HttpSend> ops::Deref for Mastodon<H> {
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

struct MastodonBuilder<H: HttpSend> {
    client: Option<Client>,
    http_sender: H,
    data: Option<Data>,
}

impl<H: HttpSend> MastodonBuilder<H> {
    pub fn new(sender: H) -> Self {
        MastodonBuilder {
            http_sender: sender,
            client: None,
            data: None,
        }
    }

    #[allow(dead_code)]
    pub fn client(&mut self, client: Client) -> &mut Self {
        self.client = Some(client);
        self
    }

    pub fn data(&mut self, data: Data) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> Result<Mastodon<H>> {
        Ok(if let Some(data) = self.data {
            let mut headers = Headers::new();
            headers.set(Authorization(Bearer {
                token: (*data.token).to_owned(),
            }));

            Mastodon {
                client: self.client.unwrap_or_else(|| Client::new()),
                http_sender: self.http_sender,
                headers,
                data,
            }
        } else {
            return Err(Error::DataMissing);
        })
    }
}

// Convert the HTTP response body from JSON. Pass up deserialization errors
// transparently.
fn deserialise<T: for<'de> serde::Deserialize<'de>>(mut response: Response) -> Result<T> {
    use std::io::Read;

    let mut vec = Vec::new();
    response.read_to_end(&mut vec)?;

    match json::from_slice(&vec) {
        Ok(t) => Ok(t),
        // If deserializing into the desired type fails try again to
        // see if this is an error response.
        Err(e) => {
            if let Ok(error) = json::from_slice(&vec) {
                return Err(Error::Api(error));
            }
            Err(e.into())
        },
    }
}

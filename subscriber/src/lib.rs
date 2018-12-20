/* Handle external Push Subscription Requests.
 * "priviledged" system calls may require additional handling and should be flagged as such.
 */

extern crate serde_json;

extern crate crypto;
extern crate notifier;
extern crate storage;

use storage::{ChannelID, Storage};

pub struct SubscriptionError;

pub struct SubscriptionKeys {
    pub auth: Vec<u8>,
    pub p256dh: Vec<u8>,
}

// Subscription structure
pub struct Subscription {
    pub channelid: ChannelID,
    pub endpoint: String,
    pub keys: SubscriptionKeys,
}

pub trait Subscriber {
    // get a new subscription (including keys, endpoint, etc.)
    // note if this is a "priviledged" system call that does not require additional decryption
    fn get_subscription<S: Storage>(
        storage: S,
        origin_attributes: HashMap<String, String>, // Does this include the origin proper?
        app_server_key: &str,                       // Passed to server.
        priviledged: bool,                          // Is this a system call / skip encryption?
    ) -> Result<Subscription, SubscriptionError>;

    // Update an existing subscription (change bridge endpoint)
    fn update_subscription<S: Storage>(
        storage: S,
        chid: ChannelID,
        bridge_id: Option<String>,
    ) -> Result<Subscription, SubscriptionError>;

    // remove a subscription
    fn del_subscription<S: Storage>(store: S, chid: ChannelID) -> Result<bool, SubscriptionError>;

    // to_json -> impl Into::<String> for Subscriber...
}

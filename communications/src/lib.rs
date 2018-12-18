/* Server Communications.
 * Handles however communication to and from the remote Push Server should be done. For Desktop
 * this will be over Websocket. For mobile, it will probably be calls into the local operating
 * system and HTTPS to the web push server.
 *
 * In the future, it could be using gRPC and QUIC, or quantum relay.
 */

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;

pub struct RegisterResponse {

    // the UAID & Channel ID associated with the request
    uaid: String,
    channel_id: String,

    // Auth token for subsequent calls (note, only generated on new UAIDs)
    secret: Option<String>,

    // Push endpoint for 3rd parties
    endpoint: String,

    // The Sender/Group ID echoed back (if applicable.)
    senderid: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastValue {
    Value(String),
    Nested(HashMap<String, BroadcastValue>),
}

pub struct ConnectionError;

pub trait Connection {

    // Generate a new connection & send a "hello"
    fn new<C: Connection>(url: String, options: HashMap<String, String>) -> Result<C, ConnectionError>;

    // send a new subscription request to the server, get back the server registration response.
    fn subscribe(channel_id: &str, vapid_public_key: &str, registration_token: &str) -> Result<RegisterResponse, ConnectionError>;

    // Drop an endpoint
    fn unsubscribe(channel_id: &str, auth: &str) -> Result<bool, ConnectionError>;

    // Update an endpoint with new info
    fn update(channel_id: &str, auth: &str, new_token: &str) -> Result<bool, ConnectionError>;

    // Get a list of server known channels. If it differs from what we have, reset the UAID, and refresh channels.
    // Should be done once a day.
    fn channel_list() -> Vec<String>;

    // Add one or more new broadcast subscriptions.
    fn broadcast_subscribe(broadcast: BroadcastValue) -> Result<BroadcastValue, ConnectionError>;

    // get the list of broadcasts
    fn broadcasts() -> Result<BroadcastValue, ConnectionError>;

    //impl TODO: Handle a Ping response with updated Broadcasts.
    //impl TODO: Handle an incoming Notification
}


/* Handle Push data storage
 */
extern crate crypto;

use std::collections::HashMap;

use crypto::Key;

pub type ChannelID = String;

pub struct PushRecord {
    // Endpoint provided from the push server
    pub endpoint: String,

    // Designation label provided by the subscribing service
    pub designator: String,

    // List of origin Host attributes.
    pub origin_attributes: HashMap<String, String>,

    // Number of pushes for this record
    pub push_count: u8,

    // Last push rec'vd
    pub last_push: u64,

    // Private EC Prime256v1 key info. (Public key can be derived from this)
    pub private_key: Vec<u8>,

    // Push Server auth_secret
    pub auth_secret: Vec<u8>,

    // Is this as priviledged system record
    pub system_record: bool,

    // VAPID public key to restrict subscription updates for only those that sign
    // using the private VAPID key.
    pub app_server_key: String,

    // List of the most recent message IDs from the server.
    pub recent_message_ids: Vec<String>,

    // Time this subscription was created.
    pub ctime: u64,

    // Max quota count for sub
    pub quota: u8,

    // (if this is a bridged connection (e.g. on Android), this is the native OS Push ID)
    pub native_id: String,
}

//TODO: Add broadcasts storage

pub struct StorageError;

pub trait Storage {
    // Connect to the storage system
    fn connect<S: Storage>() -> S;

    // Generate a Push Record from the Subscription info, which has the endpoint,
    // encryption keys, etc.
    fn create_record(
        uaid: &str,
        chid: &str,
        origin_attributes: HashMap<String, String>,
        endpoint: &str,
        auth: Vec<u8>,
        private_key: &Key,
        system_record: bool,
    ) -> PushRecord;
    fn get_record(uaid: &str, chid: &str) -> Option<PushRecord>;
    fn put_record(uaid: &str, chid: &str, record: &PushRecord) -> Result<bool, StorageError>;
    fn purge(uaid: &str, chid: Option<&str>) -> Result<bool, StorageError>;
}

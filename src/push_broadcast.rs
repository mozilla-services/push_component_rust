use std::collections::HashMap;

use push_service::PushService;
use unknown::{ChromeUtils, JSONFile};

pub struct BroadcastService {
    pub service: PushService,
    pub listenerPath: JSONFile,
    // pub initializePromise: Promise,
}

struct SourceInfo {
    pub moduleURI: String,
    pub symbolName: String,
}

pub struct Broadcasts {
    pub broadcastId: String,
    pub version: String,
}

// External
impl BroadcastService {
    // aka "new"
    pub fn initializeBroadcastService(
        service: PushService,
        listenerPath: JSONFile,
    ) -> BroadcastService {
        BroadcastService {
            service,
            listenerPath,
            // initializePromise: Promise{}
        }
    }

    // Convert the listeners from on-disk format to format needed for "hello"
    pub fn getListeners() -> HashMap<String, String> {
        HashMap::new()
    }

    // Add an entry for a given listener, or update one if already present
    pub fn addListener(broadcastId: String, version: String, sourceInfo: SourceInfo) {}

    pub fn receivedBroadcastMessage(broadcasts: Vec<Broadcasts>) {}

    // (used in testing)
    pub fn _saveImmediately() {}
}

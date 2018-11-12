use std::collections::HashMap;

use push_service::PushService;
use unknown::{ChromeUtils, JSONFile};

pub struct BroadcastService {
    pub pushService: PushService,
    pub jsonFile: JSONFile,
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

pub struct PageRecord {
    pub scope: Option<String>,
    pub appServerKey: Option<Vec<u8>>,
    pub requestID: Option<u64>,
    pub domain: Option<String>,
    pub systemRecord: bool,
    pub originAttributes: ChromeUtils::Principal::OriginAttributes,
}

impl BroadcastService {
    pub fn initializeBroadcastService() -> BroadcastService {
        BroadcastService {
            pushService: PushService {},
            jsonFile: JSONFile {},
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
}

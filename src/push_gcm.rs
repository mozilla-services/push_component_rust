/// Used by the Android GCM core.
use std::time::Instant;

use futures::Future;
use url::Url;

use unknown::{nsISupports, BroadcastListener, ChromeUtils, EventDispatcher};

use push_db::{PushDB, PushDBRecord};
use push_service::{PushError, PushOptions, PushService};

struct PushServiceAndroidGCM {
    mainPushService: PushService,
    serverURI: Url,
}

impl PushServiceAndroidGCM {
    // External
    fn newPushDB(&self) -> PushDB {}

    fn validServerURI(&self, serverURI: Url) -> bool {}

    // External?
    fn observe(&self, subject: nsISupports, topic: String, data: String) {}

    // External
    fn init(
        Options: PushOptions,
        mainPushService: PushService,
        serverURL: Url,
    ) -> PushServiceAndroidGCM {
    }

    fn uninit(&self) {}

    fn onAlarmFired(&self) {}

    // External
    fn connect(
        &self,
        records: Vec<PushDBRecord>,
        broadcastListeners: Vec<BroadcastListener>,
    ) -> impl Future<Item = bool, Error = PushError> {
    }

    fn sendSubscribeBroadcast(&self, serviceId: String, version: String) {}

    fn isConnected(&self) -> bool {}

    fn disconnect(&self) {}

    // External
    fn register(&self, record: PushDBRecord) -> PushRecordAndroidGCM {}

    // External?
    fn unregister(&self, record: PushDBRecord) -> EventDispatcher {}
}

// External?
struct PushRecordAndroidGCM {
    channelID: String,
    pushEndpoint: Url,
    scope: String,
    originAttributes: ChromeUtils::Principal::OriginAttributes,
    ctime: Instant,
    systemRecord: bool,
    p256dhPublicKey: Vec<u8>,
    p256dhPrivateKey: Vec<u8>,
    authenticationSecret: Vec<u8>,
    appServerKey: Vec<u8>,
}

impl PushRecordAndroidGCM {
    fn get(&self) -> String {}
}

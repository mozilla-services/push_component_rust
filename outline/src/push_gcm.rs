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
    fn newPushDB(&self) -> PushDB {}

    fn validServerURI(&self, serverURI: Url) -> bool {}

    fn observe(&self, subject: nsISupports, topic: String, data: String) {}

    fn init(
        Options: PushOptions,
        mainPushService: PushService,
        serverURL: Url,
    ) -> PushServiceAndroidGCM {
    }

    fn uninit(&self) {}

    fn onAlarmFired(&self) {}

    fn connect(
        &self,
        records: Vec<PushDBRecord>,
        broadcastListeners: Vec<BroadcastListener>,
    ) -> impl Future<Item = bool, Error = PushError> {
    }

    fn sendSubscribeBroadcast(&self, serviceId: String, version: String) {}

    fn isConnected(&self) -> bool {}

    fn disconnect(&self) {}

    fn register(&self, record: PushDBRecord) -> PushRecordAndroidGCM {}

    fn unregister(&self, record: PushDBRecord) -> EventDispatcher {}
}

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

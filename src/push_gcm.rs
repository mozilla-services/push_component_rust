
use url::URI

use unknown::{ChromeUtils};

use push_service::{PushService};

struct PushServiceAndroidGCM {
    mainPushService: PushService,
    serverURI: URI,
}

trait PushServiceAndroidGCM {
    fn newPushDB() -> PushDB;

    fn validServerURI(serverURI: URI) -> bool;

    fn observe(subject:??, topic:String, data:String);

    fn init(Options:PushOptions, mainPushService: PushService,
        serverURL:URI)
        -> PushServiceAndroidGCM;

    fn uninit();

    fn onAlarmFired();

    fn connect(records: Vec<PushDBRecord>,
        broadcastListeners: Vec<BroadcastListeners>) -> Promise;

    fn sendSubscribeBroadcast(serviceId:String, version:String);

    fn isConnected() -> bool;

    fn disconnect();

    fn register(record:PushDBRecord) -> PushRecordAndroidGCM;

    fn unregister(record:PushDBRecord) -> EventDispatcher;
}

struct PushRecordAndroidGCM {
    channelID: String,
    pushEndpoint: URI,
    scope:String,
    originAttributes: ChromeUtils::Principal::OriginAttributes,
    ctime: Time,
    systemRecord:bool,
    p256dhPublicKey: Vec<u8>,
    p256dhPrivateKey: Vec<u8>,
    authenticationSecret: Vec<u8>,
    appServerKey: Vec<u8>,
}

trait PushRecordAndroidGCM {
    fn get() -> String;
}

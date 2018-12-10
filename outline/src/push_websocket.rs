/// Handle Websocket creation and interactions.
use futures::Future;
use url::Url;

use unknown::{nsISupports, BroadcastListener, Context, PageRecord};

use push_components::PushSubscription;
use push_db::{PushDB, PushDBRecord};
use push_record::PushRecord;
use push_service::{PushError, PushOptions, PushService};

struct PushWebSocketListner {
    pushservice: PushService,
}

trait PushWebSocketListener {
    fn onStart(context: Context);
    fn onStop(context: Context, statusCode: u64);
    fn onAcknowledge(context: Context, size: usize) {} // noop
    fn onBinaryMessageAvailable(context: Context, message: Vec<u8>) {} //noop
    fn onMessageAvailable(context: Context, message: String);
    fn onServerClose(context: Context, aStatusCode: u64, aReason: u64);
}

struct PushServiceWebSocket {
    mainPushService: PushService,
    serverURI: Url,
}

impl PushServiceWebSocket {
    fn newPushDB() -> PushDB {}
    fn disconnect() {}
    fn observe(aSubject: nsISupports, aTopic: String, aData: String) {}
    fn validServerURI(serverURI: Url) -> bool {}
    fn init(
        Options: PushOptions,
        mainPushService: PushService,
        serverURI: Url,
    ) -> impl Future<Error = PushError, Item = Self> {
    }
    fn uninit() {}
    fn connect(records: Vec<PushDBRecord>, broadcastListeners: Vec<BroadcastListener>) {}

    fn isConnected() -> bool {}
    fn reportDeliveryError(messageID: String, reason: u64) {}
    fn register(record: PageRecord) -> Result<PushRecord, PushError> {}
    fn unregister(record: PushRecord, reason: u64) -> Result<PushRecord, PushError> {}
    fn sendSubscribeBroadcast(serviceId: String, version: String) {}
}

struct PushRecordWebSocket {
    pushRecord: PushRecord,
    channelID: String,
    version: String,
}

// impl Into<PushSubscription> for PushRecordWebSocket{...}

impl PushRecordWebSocket {
    fn get() {}
    fn toSubscription() -> PushSubscription {}
}

/// Manage Push Service instance and function.
use std::collections::HashMap;

use futures::Future;
use url::Url;

use unknown::{
    nsISupports, DropJSObjects, JSCallback, ProcessQueue, SystemPrincipal, UnknownOperation,
};

use push_broadcast::{Broadcasts, PageRecord};
use push_components::PushSubscription;
use push_crypto::CryptoError;
use push_db::PushDB;
use push_record::PushRecord;

pub enum PushServiceEvent {
    STARTING_SERVICE_EVENT,
    CHANGING_SERVICE_EVENT,
    STOPPING_SERVICE_EVENT,
    UNINIT_EVENT,
}

pub type PushOptions = HashMap<String, DropJSObjects>;

pub struct PushError {
    message: String,
    result: u64,
}

pub fn errorWithResult(message: String, result: u64) -> PushError {
    PushError { message, result }
}

pub struct PushService {
    pub state: PushServiceState,
    pub db: PushDB,
    pub Options: PushOptions,
    pub visibleNotifications: HashMap<String, u64>,
    pub stateChangeProcessQueue: ProcessQueue,
    pub pendingRegisterRequest: HashMap<String, Result<PushSubscription, PushError>>,
    pub notifyActivated: Future<Item = bool, Error = PushError>,
    pub activated: bool,
    pub pushTopic: String,
    pub subscriptionChangeTopic: String,
    pub subscriptionModifiedTopic: String,
}

pub struct PushMessage {
    broadcasts: Vec<Broadcasts>,
    buffer: Vec<u8>,
}

pub enum PushServiceState {
    PUSH_SERVICE_UNINIT,
    PUSH_SERVICE_INIT,
    PUSH_SERVICE_ACTIVATING,
    PUSH_SERIVCE_CONNECTION_DISABLE,
    PUSH_SERVICE_ACTIVE_OFFLINE,
    PUSH_SERVICE_RUNNING,
}

impl PushService {
    // External
    fn init(Options: PushOptions) {}
    // External
    fn uninit() {}
    // External
    fn observe(aSubject: nsISupports, aTopic: String, aData: String) {}
    // External
    fn registration(aPageRecord: PageRecord) -> impl Future<Item = bool, Error = PushError> {}
    // External
    fn getSubscription(scope: String, principal: SystemPrincipal, callback: JSCallback) {}
    // External
    fn unregister(aPageRecord: PageRecord) -> impl Future<Item = bool, Error = PushError> {}
    // External?
    fn stateChangeProcessEnqueue(op: UnknownOperation) -> Result<Option<ProcessQueue>, PushError> {}

    fn subscribeBroadcast(broadcastId: String, version: String) {}
    fn clear(info: PageRecord) -> impl Future<Item = bool, Error = PushError> {}
    fn registration(aPageRecord: PageRecord) -> Option<PushSubscription> {}

    fn checkActivated() -> impl Future<Item = bool, Error = PushError> {}

    fn makePendingKey(aPageRecord: PageRecord) -> String {}
    fn lookupOrPutPendingRequest(aPageRecord: PageRecord) -> PushSubscription {}
    fn deletePendingRequest(aPageRecord: PageRecord) {}

    fn setState(aNewState: PushServiceState) {}
    fn changeStateOfflineEvent(offline: bool, calledFromConnEnabledEvent: bool) {}
    fn changeStateConnectionEnabledEvent(enabled: bool) {}

    fn clearOriginData(data: String) -> impl Future<Item = bool, Error = PushError> {}
    fn backgroundUnregister(record: PushRecord, reason: u64) {}
    fn findService(serverURL: String) -> (String, String) {}
    fn changeServerURL(
        serverURI: Url,
        event: PushServiceEvent,
        Options: HashMap<String, String>,
    ) -> impl Future<Item = bool, Error = PushError> {
    }

    fn changeStateConnectionEnabledEvent() {}

    fn dropUnexpiredRegistrations() -> bool {}
    fn updateRegistrationAndNotifyApp(aOldKey: String, aNewRecord: PushRecord) -> PushRecord {}
    fn updateRecordAndNotifyApp(aKeyID: String, aUpdateFunc: fn(PushRecord)) -> PushRecord {}
    fn ensureCrypto(record: PushRecord) -> impl Future<Item = bool, Error = CryptoError> {}
    fn receivedPushMessage(
        keyID: String,
        messageID: String,
        headers: HashMap<String, String>,
        data: Vec<u8>,
        updateFunc: fn(PushRecord),
    ) -> impl Future<Item = bool, Error = PushError> {
    }

    fn receivedBroadcastMessage(message: PushMessage) {}

    fn notificationForOriginShown(origin: String) {}
    fn notificationForOriginClosed(origin: String) {}
    fn notificationsCleared() {}

    fn reportDeliveryError(messageID: String, reason: u64) {}

    fn getByKeyID(aKeyId: String) -> Option<PushRecord> {}
    fn getAllUnexpired() -> Option<Vec<PushRecord>> {}
}

pub struct PushServiceOptions {
    pub serverURI: Url,
}

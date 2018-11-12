use std::time::Instant;

use unknown::{ChromeUtils, ComponentID, JSCallback, SystemPrincipal};

use push_crypto::P256DH_Raw_Key;
use push_service::{PushMessage, PushOptions};

/// DOM interface for the Push Component
pub struct PushServiceBase {
    pub classID: ComponentID,
    pub contractID: String,
    pub queryInterface: ChromeUtils::Interface,
    pub pushTopic: String,
    pub subscriptionChangeTopic: String,
    pub subscriptionModifiedTopic: String,
}

impl PushServiceBase {
    pub fn observe(subject: String, topic: String, data: Vec<u8>) {}
}

// trait PushServiceParent: PushServiceBase{
// extends PushServiceBase
// }

trait PushServiceContent {
    fn subscribe(scope: String, principal: SystemPrincipal, callback: JSCallback);
    fn subscribeWithKey(
        scope: String,
        principal: SystemPrincipal,
        keyLen: u64,
        key: Vec<u8>,
        callback: JSCallback,
    );
    fn getSubscription(scope: String, principal: SystemPrincipal, callback: JSCallback);
    fn clearForDomain(domain: String, callback: JSCallback);
    fn notificationForOriginShown(origin: String);
    fn notificationForOriginClosed(origin: String);
    fn reportDeliveryError(messageId: String, reason: u64);
    fn receiveMessage(message: PushMessage);
    fn unsubscribe(scope: String, principal: SystemPrincipal, callback: JSCallback);
}

trait PushServiceParent: PushServiceContent {
    // extends PushServiceContent
    fn replaceServiceBackend(Options: PushOptions);

    fn restoreServiceBackend();
}

/// Core Push Subscription record.
pub struct PushSubscription {
    aEndpoint: String,
    aScope: String,
    aRawP256dhKey: P256DH_Raw_Key,
    aAuthSecret: Vec<u8>,
    aAppServerKey: Vec<u8>,
    modified: Instant,
    ttl: Instant,
    count: u64,
    quota: u64,
    is_system: bool,
}

impl PushSubscription {
    pub fn endpoint(&self) -> String {
        self.aEndpoint
    }
    pub fn lastPush(&self) -> Instant {
        self.modified
    }
    pub fn pushCount(&self) -> u64 {
        self.count
    }
    pub fn quota(&self) -> u64 {
        self.quota
    }
    pub fn isSystemSubscription(&self) -> bool {
        self.is_system
    }

    /// return private half of self.aRawP256dhKey
    pub fn p256dhPrivateKey(&self) -> Vec<u8> {}

    pub fn quotaApplies(&self) -> bool {
        self.quota > 0
    }

    /// Has the TTL expired?
    pub fn isExpired() -> bool {}

    /// get the Public key
    pub fn getKey(name: String, outKeyLen: u64) -> Vec<u8> {}
}

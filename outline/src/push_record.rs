/// DOM record for a given push subscription manager.
use std::time::Instant;

use push_components::PushSubscription;
use push_crypto::P256DH_Raw_Key;

use unknown::{ChromeUtils, SystemPrincipal};

pub struct PushRecord {
    pushEndpoint: String,
    scope: String,
    originAttributes: ChromeUtils::Principal::OriginAttributes,
    pushCount: u64,
    lastPush: Instant,
    p256dhKey: P256DH_Raw_Key, // Public & Private Keys can be derived from Rawk
    authenticationSecret: Vec<u8>,
    systemRecord: bool, // is this a system record?
    appServerKey: Vec<u8>,
    recentMessageIDs: Vec<String>,
    ctime: Instant,
    quota: u64,
    principal: SystemPrincipal,
}

impl PushRecord {
    fn setQuota(suggestedQuota: u64) {}
    fn resetQuota() {}
    fn reduceQuota() {}
    fn quotaChanged() -> bool {}
    fn quotaApplies() -> bool {}
    fn updateQuota(lastVisit: Instant) {}

    fn receivedPush(lastVisit: Instant) {}
    fn noteRecentMessageID(id: String) {}
    fn hasRecentMessageID(id: String) -> bool {}
    fn getLastVisit() -> Instant {}
    fn isTabOpen() -> bool {}
    fn hasPermission() -> bool {}
    fn isExpired() -> bool {}
    fn matchesOriginAttributes(pattern: String) -> bool {}
    fn hasAuthenticationSecret() -> bool {}
    fn matchesApServerKey(key: Vec<u8>) -> bool {}
    fn toSubscription() -> PushSubscription {}
}
// PushNotifier -- Include??

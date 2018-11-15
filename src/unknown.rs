// This file contains placeholders for undefined or external requirements.
// I'm not sure what the structure of these are yet.

// From XULBroadcastManager.cpp
pub struct BroadcastListener {
    // nsWeakPtr mListener,
// RefPtr<nsAtom> mAttribute
}

type BroadcastListeners = Vec<BroadcastListener>;

pub struct Context {}

pub struct ComponentID {}

pub struct DBHandle {}
pub struct DropJSObjects {}

pub struct EventDispatcher {}

pub struct InputStream {}

pub struct JSCallback {}

pub struct JSONFile {}

pub struct JSONWebKey {}

// pub struct Promise {} // This is a rust futures handler

pub struct ProcessQueue {}

pub struct QueryInterface {}
pub struct nsIHttpChannel {}
pub struct nsISupports {}

pub struct Request {}

pub struct SystemPrincipal {}

pub struct TestClause {}

pub struct TransactionHandle {}

// Possible future function ref handled by stateChangeProcessQueue.
// see https://dxr.mozilla.org/mozilla-central/source/dom/push/PushService.jsm#114
pub struct UnknownOperation {} // this is an "op" field passed in to a push service state change monitor.

pub mod ChromeUtils {
    pub struct generateQI {}

    pub struct Interface {}

    pub mod Principal {
        pub struct OriginAttributes {}
    }
}

// This file contains placeholders for undefined or external requirements.

pub struct BroadcastListener {}

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

pub struct PageRecord {}

pub struct ProcessQueue {}

pub struct QueryInterface {}
pub struct nsIHttpChannel {}

pub struct Request {}

pub struct SystemPrincipal {}

pub struct TestClause {}

pub struct TransactionHandle {}

pub struct UnknownOperation {} // this is an "op" field passed in to a push service state change monitor.

pub mod ChromeUtils {
    pub struct generateQI {}

    pub struct Interface {}

    pub mod Principal {
        pub struct OriginAttributes {}
    }
}

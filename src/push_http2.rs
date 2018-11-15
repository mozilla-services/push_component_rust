use std::time::Instant;
use url::Url;

use futures::Future;

use unknown::{nsIHttpChannel, ChromeUtils, Context, InputStream, QueryInterface, Request};

use push_components::PushSubscription;
use push_service::{PushError, PushOptions, PushService};

// External?
trait PushSubscriptionListener {
    fn getInterface(aIID: ChromeUtils::generateQI) -> QueryInterface;
    fn onStartRequest(aRequest: Request, aContext: Context) {} //NOOP
    fn onDataAvailable(
        aRequest: Request,
        aContext: Context,
        aStream: InputStream,
        aOffset: u64,
        aCount: u64,
    );
    fn onStopRequest(aRequest: Request, aContext: Context, aStatusCode: u64);
    fn onPush(associatedChannel: nsIHttpChannel, pushChannel: nsIHttpChannel);
    fn disconnect();
}

struct PushChannelListener {
    mainListener: PushSubscriptionListener,
    message: Vec<u8>,
    ackUri: Option<Url>,
}

// External?
impl PushChannelListener {
    fn onStartRequest(aRequest: Request, aContext: Context) {}
    fn onDataAvailable(
        aRequest: Request,
        aContext: Context,
        aStream: InputStream,
        aOffset: u64,
        aCount: u64,
    ) {
    }
    fn onStopRequest(aRequest: Request, aContext: Context, aStatusCode: u64) {}
}

trait PushServiceHttp2 {
    fn init(
        aOptions: PushOptions,
        aMainPushService: PushService,
        aServerURL: Url,
    ) -> Future<Item = Self, Error = PushError>;
    fn startConnections(aSubscriptions: Vec<PushSubscription>);
    fn uninit();
    fn unregister(aRecord: PushRecordHttp2) -> Future<Item = bool, Error = PushError>;
    fn reportDeliveryError(messageID: String, reason: u64);

    // External?
    fn connOnStop(aRequest: Request, aSuccess: bool, aSubscriptionUri: Url);
    fn addListenerPendingRetry(aListener: SubscriptionListener);
    fn removeListenerPendingRetry(aListener: SubscriptionListener);

    // External?
    fn getHeaderField(aRequest: Request, name: String) -> Option<String>;
}

// External?
trait PushServiceDelete {
    fn new() -> Self;
    fn onStartRequest(aRequest: Request, aContext: Context);
    fn onStopRequest(aRequest: Request, aContext: Context);
    fn onDataAvailable(
        aRequest: Request,
        aContext: Context,
        aStream: InputStream,
        aOffset: u64,
        aCount: u64,
    );
}

struct PushEndpoint {
    pushEndpoint: String,
    pushReceiptEndpoint: String,
}

struct SubscriptionListener {
    subInfo: PushSubscription,
    // resolve: Promise,
    // reject:Promise,
    data: Vec<u8>,
    serverURI: Url,
    service: PushService,
    ctime: Instant,
    retryTimeoutID: Fn(_),
}

// External?
impl SubscriptionListener {
    fn onStartRequest(aRequest: Request, aContext: Context) {}
    fn onDataAvailable(
        aRequest: Request,
        aContext: Context,
        aStream: InputStream,
        aOffset: u64,
        aCount: u64,
    ) {
    }
    fn onStopRequest(aRequest: Request, aContext: Context, aStatus: u64) {}
    fn abortRetry() {}
    fn retryAfterParser(aRequest: Request) {}
    fn linkParser(linkHeader: String, serverURI: Url) -> Result<PushEndpoint, PushError> {}
}

struct PushRecordHttp2 {
    subscriptionUri: Url,
    pushEndpoint: Url,
    pushReceiptEndpoint: Url,
    scope: String,
    originAttributes: ChromeUtils::Principal::OriginAttributes,
    systemRecord: bool,
    appServerKey: Vec<u8>,
    ctime: Instant,
}

impl PushRecordHttp2 {
    // External?
    fn init(
        aOptions: PushOptions,
        aMainPushService: PushService,
        aServerURL: Url,
    ) -> impl Future<Item = PushRecordHttp2, Error = PushError> {
    }
    fn startConnections(aSubscriptions: Vec<PushSubscription>) {}
    fn uninit() {}
    fn unregister(aRecord: PushRecordHttp2) -> impl Future<Item = bool, Error = PushError> {}
    fn reportDeliveryError(messageID: String, reason: u64) {}

    // External?
    fn connOnStop(aRequest: Request, aSuccess: bool, aSubscriptionUri: Url) {}
    fn addListenerPendingRetry(aListener: SubscriptionListener) {}
    fn removeListenerPendingRetry(aListener: SubscriptionListener) {}

    // External?
    fn getHeaderField(aRequest: Request, name: String) -> Option<String> {
        None
    }

    fn new(record: PushRecordHttp2) {}
    fn toSubscription() -> PushSubscription {}
}

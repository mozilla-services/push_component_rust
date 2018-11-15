/// Renamed from push_http2 since that name raised confusion.
///
/// Apple Push Notification System (APNS) uses http2 as their communications
/// channel, so does RFC8030 WebPush. This can cause confusion for folks who
/// believe that this is using http2 for non APNS reasons.
use std::time::Instant;
use url::Url;

use futures::Future;

use unknown::{nsIHttpChannel, ChromeUtils, Context, InputStream, QueryInterface, Request};

use push_components::PushSubscription;
use push_service::{PushError, PushOptions, PushService};

pub struct PushSubscriptionListener;

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

pub struct PushChannelListener {
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

trait PushServiceApns {
    fn init(
        aOptions: PushOptions,
        aMainPushService: PushService,
        aServerURL: Url,
    ) -> Future<Item = Self, Error = PushError>;
    fn startConnections(aSubscriptions: Vec<PushSubscription>);
    fn uninit();
    fn unregister(aRecord: PushRecordApns) -> Future<Item = bool, Error = PushError>;
    fn reportDeliveryError(messageID: String, reason: u64);

    // External?
    fn connOnStop(aRequest: Request, aSuccess: bool, aSubscriptionUri: Url);
    fn addListenerPendingRetry(aListener: SubscriptionListener);
    fn removeListenerPendingRetry(aListener: SubscriptionListener);

    // External?
    fn getHeaderField(aRequest: Request, name: String) -> Option<String>;
}

pub struct PushServiceDelete;

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

pub struct SubscriptionListener {
    subInfo: PushSubscription,
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

pub struct PushRecordApns {
    subscriptionUri: Url,
    pushEndpoint: Url,
    pushReceiptEndpoint: Url,
    scope: String,
    originAttributes: ChromeUtils::Principal::OriginAttributes,
    systemRecord: bool,
    appServerKey: Vec<u8>,
    ctime: Instant,
}

impl PushRecordApns {
    // External?
    fn init(
        aOptions: PushOptions,
        aMainPushService: PushService,
        aServerURL: Url,
    ) -> impl Future<Item = PushRecordApns, Error = PushError> {
    }
    fn startConnections(aSubscriptions: Vec<PushSubscription>) {}
    fn uninit() {}
    fn unregister(aRecord: PushRecordApns) -> impl Future<Item = bool, Error = PushError> {}
    fn reportDeliveryError(messageID: String, reason: u64) {}

    // External?
    fn connOnStop(aRequest: Request, aSuccess: bool, aSubscriptionUri: Url) {}
    fn addListenerPendingRetry(aListener: SubscriptionListener) {}
    fn removeListenerPendingRetry(aListener: SubscriptionListener) {}

    // External?
    fn getHeaderField(aRequest: Request, name: String) -> Option<String> {
        None
    }

    fn new(record: PushRecordApns) {}
    fn toSubscription() -> PushSubscription {}
}

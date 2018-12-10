use unknown::*;

use futures::Future;

use push_record::PushRecord;

pub struct PushDBError {}

pub type DBError = Box<PushDBError>;

pub struct PushDBRecord {
    keyID: String,
    scope: String,
    originAttributes: ChromeUtils::Principal::OriginAttributes,
    quota: u64,
}

pub struct PushDB;

impl PushDB {
    pub fn new(
        dbName: String,
        dbVersion: u64,
        dbSToreName: String,
        keyPath: String,
        model: fn(PushDBRecord),
    ) {
    }

    pub fn initDBHelper(dbName: String, dbVersion: u64, dbStoreNames: Vec<String>) {}

    pub fn toPushRecord(record: PushDBRecord) -> PushRecord {}

    pub fn isValidRecord(record: PushDBRecord) {}

    pub fn upgradeSchema(
        aTransaction: TransactionHandle,
        aDb: DBHandle,
        aOldVersion: u64,
        aNewVersion: u64,
    ) {
    }

    pub fn put(aRecord: PushDBRecord) -> impl Future<Item = bool, Error = DBError> {}

    pub fn delete(aKeyID: String) -> impl Future<Item = bool, Error = DBError> {}

    pub fn clearIf(testFn: TestClause) -> impl Future<Item = bool, Error = DBError> {}

    pub fn getByPushEndpoint(
        aPushEndpoint: String,
    ) -> impl Future<Item = PushDBRecord, Error = DBError> {
    }

    pub fn getByKeyId(aKeyId: String) -> impl Future<Item = PushDBRecord, Error = DBError> {}

    pub fn forEachOrigin(
        origin: String,
        originAttributes: ChromeUtils::Principal::OriginAttributes,
        callback: JSCallback,
    ) -> impl Future<Item = PushDBRecord, Error = DBError> {
    }

    pub fn getByIdentifiers(
        aPageRecord: PageRecord,
    ) -> impl Future<Item = PushDBRecord, Error = DBError> {
    }

    pub fn getAllByOriginAttributes(
        aOriginAttributes: ChromeUtils::Principal::OriginAttributes,
    ) -> impl Future<Item = Vec<PushDBRecord>, Error = DBError> {
    }

    pub fn getAllKeyIDs() -> impl Future<Item = Vec<PushDBRecord>, Error = DBError> {}

    pub fn getAllUnexpired() -> impl Future<Item = Vec<PushDBRecord>, Error = DBError> {}

    pub fn getAllExpired() -> impl Future<Item = Vec<PushDBRecord>, Error = DBError> {}

    pub fn update(aKeyID: String, aUpdateFunc: fn(_)) -> impl Future<Item = bool, Error = DBError> {
    }

    pub fn drop() -> impl Future<Item = bool, Error = DBError> {}
}

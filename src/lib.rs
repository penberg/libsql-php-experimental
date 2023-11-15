#![allow(non_snake_case)]
#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();

    RUNTIME.get_or_try_init(Runtime::new).unwrap()
}

#[php_class]
pub struct SQLite3 {
    db: libsql::Database,
    conn: libsql::Connection,
}

#[php_impl]
impl SQLite3 {
    #[php_const]
    pub const OK: i32 = 0;
    #[php_const]
    pub const DENY: i32 = 1;
    #[php_const]
    pub const IGNORE: i32 = 2;
    #[php_const]
    pub const CREATE_INDEX: i32 = 3;
    #[php_const]
    pub const CREATE_TABLE: i32 = 4;
    #[php_const]
    pub const CREATE_TEMP_INDEX: i32 = 5;
    #[php_const]
    pub const CREATE_TEMP_TABLE: i32 = 6;
    #[php_const]
    pub const CREATE_TEMP_TRIGGER: i32 = 7;
    #[php_const]
    pub const CREATE_TEMP_VIEW: i32 = 8;
    #[php_const]
    pub const CREATE_TRIGGER: i32 = 9;
    #[php_const]
    pub const CREATE_VIEW: i32 = 10;
    #[php_const]
    pub const DELETE: i32 = 11;
    #[php_const]
    pub const DROP_INDEX: i32 = 12;
    #[php_const]
    pub const DROP_TABLE: i32 = 13;
    #[php_const]
    pub const DROP_TEMP_INDEX: i32 = 14;
    #[php_const]
    pub const DROP_TEMP_TABLE: i32 = 15;
    #[php_const]
    pub const DROP_TEMP_TRIGGER: i32 = 16;
    #[php_const]
    pub const DROP_TEMP_VIEW: i32 = 17;
    #[php_const]
    pub const DROP_TRIGGER: i32 = 18;
    #[php_const]
    pub const DROP_VIEW: i32 = 19;
    #[php_const]
    pub const INSERT: i32 = 20;
    #[php_const]
    pub const PRAGMA: i32 = 21;
    #[php_const]
    pub const READ: i32 = 22;
    #[php_const]
    pub const SELECT: i32 = 23;
    #[php_const]
    pub const TRANSACTION: i32 = 24;
    #[php_const]
    pub const UPDATE: i32 = 25;
    #[php_const]
    pub const ATTACH: i32 = 26;
    #[php_const]
    pub const DETACH: i32 = 27;
    #[php_const]
    pub const ALTER_TABLE: i32 = 28;
    #[php_const]
    pub const REINDEX: i32 = 29;
    #[php_const]
    pub const ANALYZE: i32 = 30;
    #[php_const]
    pub const CREATE_VTABLE: i32 = 31;
    #[php_const]
    pub const DROP_VTABLE: i32 = 32;
    #[php_const]
    pub const FUNCTION: i32 = 33;
    #[php_const]
    pub const SAVEPOINT: i32 = 34;
    #[php_const]
    pub const COPY: i32 = 35;
    #[php_const]
    pub const RECURSIVE: i32 = 36;

    pub fn __construct(path: &str) -> SQLite3 {
        let db = libsql::Database::open(path).unwrap();
        let conn = db.connect().unwrap();
        SQLite3 { db, conn }
    }

    pub fn backup() {
        todo!("Backups are not supported.");
    }

    pub fn busyTimeout() {
        todo!();
    }

    pub fn changes(&self) -> u64 {
        self.conn.changes()
    }

    pub fn close(&mut self) {
        self.conn.close();
    }

    pub fn createAggregate() {
        todo!("Aggregates are not supported.");
    }

    pub fn createCollation() {
        todo!("Collcations are not supported.");
    }

    pub fn createFunction() {
        todo!("Functions are not supported.");
    }

    pub fn enableExceptions() {
        todo!("Extensions are not supported.");
    }

    pub fn escapeString() {
        todo!();
    }

    pub fn exec(&self, query: &str) -> bool {
        let rt = runtime();
        let result = rt.block_on(self.conn.execute(query, ()));
        match result {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn lastErrorMsg() {
        todo!();
    }

    pub fn lastInsertRowID(&self) -> i64 {
        self.conn.last_insert_rowid()
    }

    pub fn loadExtension() {
        todo!();
    }

    pub fn open() {
        todo!();
    }

    pub fn openBlob() {
        todo!();
    }

    pub fn prepare() {
        todo!();
    }

    pub fn query(&self, query: &str) {
        todo!();
    }

    pub fn querySingle() {
        todo!();
    }

    pub fn setAuthorizer() {
        todo!();
    }

    pub fn version() {
        todo!();
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

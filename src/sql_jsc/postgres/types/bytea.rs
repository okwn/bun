use crate::jsc::{ArrayBuffer, JSGlobalObject, JSValue, js_error_to_postgres};
use bun_sql::postgres::AnyPostgresError;
use bun_sql::postgres::types::int_types::Short;
use bun_sql::shared::Data;

pub const TO: Short = 17;
pub const FROM: [Short; 1] = [17];

pub trait ByteaToJs {
    fn bytea_to_js(self, global: &JSGlobalObject) -> Result<JSValue, AnyPostgresError>;
}

// PORT NOTE: reshaped `value: *Data` + `defer value.deinit()` → owned `Data`;
// Drop at scope exit replaces the explicit deinit.
impl ByteaToJs for Data {
    fn bytea_to_js(self, global: &JSGlobalObject) -> Result<JSValue, AnyPostgresError> {
        ArrayBuffer::create_buffer(global, self.slice()).map_err(js_error_to_postgres)
    }
}

pub fn to_js<T: ByteaToJs>(global: &JSGlobalObject, value: T) -> Result<JSValue, AnyPostgresError> {
    value.bytea_to_js(global)
}

// ported from: src/sql_jsc/postgres/types/bytea.zig

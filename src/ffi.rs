//! Reexport odbc-sys as ffi
extern crate odbc_sys;
pub use self::odbc_sys::*;

#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlCpAttrValue {
    SQL_CP_OFF = 0,
    SQL_CP_ONE_PER_DRIVER = 1,
    SQL_CP_ONE_PER_HENV = 2,
}
pub use self::SqlCpAttrValue::*;

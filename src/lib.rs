/*
 * Copyright 2023 Oxide Computer Company
 */

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::marker::{PhantomData, PhantomPinned};
use std::os::raw::{c_char, c_int, c_ulong};

#[macro_use]
extern crate num_derive;

pub type scf_version_t = c_ulong;

pub const SCF_VERSION: scf_version_t = 1;

pub const SCF_PG_FLAG_NONPERSISTENT: u32 = 0x1;

/*
 * Flags for the smf_enable_instance(3SCF) family of functions:
 */
pub const SMF_IMMEDIATE: c_int = 0x1;
pub const SMF_TEMPORARY: c_int = 0x2;
pub const SMF_AT_NEXT_BOOT: c_int = 0x4;

macro_rules! opaque_handle {
    ($type_name:ident) => {
        #[repr(C)]
        pub struct $type_name {
            _data: [u8; 0],
            /*
             * See https://doc.rust-lang.org/nomicon/ffi.html; this marker
             * guarantees our type does not implement `Send`, `Sync`, or
             * `Unpin`.
             */
            _marker: PhantomData<(*mut u8, PhantomPinned)>,
        }
        impl Copy for $type_name {}
        impl Clone for $type_name {
            fn clone(&self) -> $type_name {
                *self
            }
        }
    };
}

opaque_handle!(scf_handle_t);
opaque_handle!(scf_iter_t);
opaque_handle!(scf_scope_t);
opaque_handle!(scf_service_t);
opaque_handle!(scf_instance_t);
opaque_handle!(scf_snapshot_t);
opaque_handle!(scf_propertygroup_t);
opaque_handle!(scf_property_t);
opaque_handle!(scf_value_t);
opaque_handle!(scf_snaplevel_t);
opaque_handle!(scf_transaction_t);
opaque_handle!(scf_transaction_entry_t);

#[derive(Debug, FromPrimitive, ToPrimitive)]
#[repr(C)]
pub enum scf_error_t {
    SCF_ERROR_NONE = 1000,            /* no error */
    SCF_ERROR_NOT_BOUND,              /* handle not bound */
    SCF_ERROR_NOT_SET,                /* cannot use unset argument */
    SCF_ERROR_NOT_FOUND,              /* nothing of that name found */
    SCF_ERROR_TYPE_MISMATCH,          /* type does not match value */
    SCF_ERROR_IN_USE,                 /* cannot modify while in-use */
    SCF_ERROR_CONNECTION_BROKEN,      /* repository connection gone */
    SCF_ERROR_INVALID_ARGUMENT,       /* bad argument */
    SCF_ERROR_NO_MEMORY,              /* no memory available */
    SCF_ERROR_CONSTRAINT_VIOLATED,    /* required constraint not met */
    SCF_ERROR_EXISTS,                 /* object already exists */
    SCF_ERROR_NO_SERVER,              /* repository server unavailable */
    SCF_ERROR_NO_RESOURCES,           /* server has insufficient resources */
    SCF_ERROR_PERMISSION_DENIED,      /* insufficient privileges for action */
    SCF_ERROR_BACKEND_ACCESS,         /* backend refused access */
    SCF_ERROR_HANDLE_MISMATCH,        /* mismatched SCF handles */
    SCF_ERROR_HANDLE_DESTROYED,       /* object bound to destroyed handle */
    SCF_ERROR_VERSION_MISMATCH,       /* incompatible SCF version */
    SCF_ERROR_BACKEND_READONLY,       /* backend is read-only */
    SCF_ERROR_DELETED,                /* object has been deleted */
    SCF_ERROR_TEMPLATE_INVALID,       /* template data is invalid */
    SCF_ERROR_CALLBACK_FAILED = 1080, /* user callback function failed */
    SCF_ERROR_INTERNAL = 1101,        /* internal error */
}

#[derive(Debug, FromPrimitive, ToPrimitive, Clone, Copy)]
#[repr(C)]
pub enum scf_type_t {
    SCF_TYPE_INVALID = 0,

    SCF_TYPE_BOOLEAN,
    SCF_TYPE_COUNT,
    SCF_TYPE_INTEGER,
    SCF_TYPE_TIME,
    SCF_TYPE_ASTRING,
    SCF_TYPE_OPAQUE,

    SCF_TYPE_USTRING = 100,

    SCF_TYPE_URI = 200,
    SCF_TYPE_FMRI,

    SCF_TYPE_HOST = 300,
    SCF_TYPE_HOSTNAME,
    SCF_TYPE_NET_ADDR_V4,
    SCF_TYPE_NET_ADDR_V6,
    SCF_TYPE_NET_ADDR,
}

pub const SCF_LIMIT_MAX_NAME_LENGTH: u32 = 0xfffff830;
pub const SCF_LIMIT_MAX_VALUE_LENGTH: u32 = 0xfffff82f;
pub const SCF_LIMIT_MAX_PG_TYPE_LENGTH: u32 = 0xfffff82e;
pub const SCF_LIMIT_MAX_FMRI_LENGTH: u32 = 0xfffff82d;

pub const SCF_SCOPE_LOCAL: &[u8] = b"localhost\0";

pub const SCF_DECODE_FMRI_EXACT: c_int = 0x00000001;
pub const SCF_DECODE_FMRI_TRUNCATE: c_int = 0x00000002;
pub const SCF_DECODE_FMRI_REQUIRE_INSTANCE: c_int = 0x00000004;
pub const SCF_DECODE_FMRI_REQUIRE_NO_INSTANCE: c_int = 0x00000008;

#[cfg(target_os = "illumos")]
mod native;
#[cfg(target_os = "illumos")]
pub use native::*;

#[cfg(not(target_os = "illumos"))]
mod stubs;
#[cfg(not(target_os = "illumos"))]
pub use stubs::*;

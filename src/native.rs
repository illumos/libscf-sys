/*
 * Copyright 2024 Oxide Computer Company
 */

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use libc::{c_void, size_t, ssize_t};

use crate::*;

#[link(name = "scf")]
extern "C" {
    pub fn scf_handle_create(version: scf_version_t) -> *mut scf_handle_t;
    pub fn scf_handle_destroy(handle: *mut scf_handle_t);
    pub fn scf_handle_bind(handle: *mut scf_handle_t) -> c_int;
    pub fn scf_handle_unbind(handle: *mut scf_handle_t) -> c_int;
    pub fn scf_handle_decorate(
        handle: *mut scf_handle_t,
        param: *const c_char,
        value: *mut scf_value_t,
    ) -> c_int;

    pub fn scf_myname(
        handle: *mut scf_handle_t,
        out: *mut c_char,
        sz: size_t,
    ) -> ssize_t;

    pub fn scf_error() -> u32;
    pub fn scf_strerror(error: scf_error_t) -> *const c_char;

    pub fn scf_limit(name: u32) -> ssize_t;

    pub fn scf_iter_create(handle: *mut scf_handle_t) -> *mut scf_iter_t;
    pub fn scf_iter_destroy(iter: *mut scf_iter_t);
    pub fn scf_iter_reset(iter: *mut scf_iter_t);

    pub fn scf_scope_create(handle: *mut scf_handle_t) -> *mut scf_scope_t;
    pub fn scf_scope_destroy(scope: *mut scf_scope_t);
    pub fn scf_scope_get_name(
        scope: *const scf_scope_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t;
    pub fn scf_handle_get_scope(
        handle: *mut scf_handle_t,
        name: *const c_char,
        out: *mut scf_scope_t,
    ) -> c_int;

    pub fn scf_scope_get_service(
        scope: *const scf_scope_t,
        name: *const c_char,
        out: *mut scf_service_t,
    ) -> c_int;
    pub fn scf_scope_add_service(
        scope: *mut scf_scope_t,
        name: *const c_char,
        out: *mut scf_service_t,
    ) -> c_int;

    pub fn scf_iter_handle_scopes(
        iter: *mut scf_iter_t,
        handle: *const scf_handle_t,
    ) -> c_int;
    pub fn scf_iter_next_scope(
        iter: *mut scf_iter_t,
        out: *mut scf_scope_t,
    ) -> c_int;

    pub fn scf_service_create(handle: *mut scf_handle_t) -> *mut scf_service_t;
    pub fn scf_service_destroy(service: *mut scf_service_t);
    pub fn scf_service_delete(service: *mut scf_service_t) -> c_int;

    pub fn scf_service_get_name(
        service: *const scf_service_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t;
    pub fn scf_service_get_instance(
        service: *const scf_service_t,
        name: *const c_char,
        out: *mut scf_instance_t,
    ) -> c_int;
    pub fn scf_service_add_instance(
        service: *mut scf_service_t,
        name: *const c_char,
        out: *mut scf_instance_t,
    ) -> c_int;

    pub fn scf_iter_scope_services(
        iter: *mut scf_iter_t,
        scope: *const scf_scope_t,
    ) -> c_int;
    pub fn scf_iter_next_service(
        iter: *mut scf_iter_t,
        out: *mut scf_service_t,
    ) -> c_int;

    pub fn scf_instance_create(
        handle: *mut scf_handle_t,
    ) -> *mut scf_instance_t;
    pub fn scf_instance_destroy(instance: *mut scf_instance_t);
    pub fn scf_instance_delete(instance: *mut scf_instance_t) -> c_int;

    pub fn scf_instance_get_name(
        instance: *const scf_instance_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t;
    pub fn scf_instance_get_pg(
        instance: *const scf_instance_t,
        name: *const c_char,
        out: *mut scf_propertygroup_t,
    ) -> c_int;
    pub fn scf_instance_add_pg(
        instance: *mut scf_instance_t,
        name: *const c_char,
        pgtype: *const c_char,
        flags: u32,
        out: *mut scf_propertygroup_t,
    ) -> c_int;
    pub fn scf_instance_get_pg_composed(
        instance: *const scf_instance_t,
        snapshot: *const scf_snapshot_t,
        name: *const c_char,
        out: *mut scf_propertygroup_t,
    ) -> c_int;
    pub fn scf_instance_get_snapshot(
        instance: *const scf_instance_t,
        name: *const c_char,
        out: *mut scf_snapshot_t,
    ) -> c_int;
    pub fn scf_instance_to_fmri(
        instance: *const scf_instance_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t;

    pub fn scf_iter_service_instances(
        iter: *mut scf_iter_t,
        service: *const scf_service_t,
    ) -> c_int;
    pub fn scf_iter_next_instance(
        iter: *mut scf_iter_t,
        out: *mut scf_instance_t,
    ) -> c_int;

    pub fn scf_snapshot_create(
        handle: *mut scf_handle_t,
    ) -> *mut scf_snapshot_t;
    pub fn scf_snapshot_destroy(snapshot: *mut scf_snapshot_t);

    pub fn scf_snapshot_get_name(
        snapshot: *const scf_snapshot_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t;
    pub fn scf_snapshot_get_parent(
        snapshot: *const scf_snapshot_t,
        inst: *mut scf_instance_t,
    ) -> c_int;

    pub fn scf_iter_instance_snapshots(
        iter: *mut scf_iter_t,
        instance: *const scf_instance_t,
    ) -> c_int;
    pub fn scf_iter_next_snapshot(
        iter: *mut scf_iter_t,
        out: *mut scf_snapshot_t,
    ) -> c_int;

    pub fn scf_iter_instance_pgs(
        iter: *mut scf_iter_t,
        instance: *const scf_instance_t,
    ) -> c_int;
    pub fn scf_iter_instance_pgs_composed(
        iter: *mut scf_iter_t,
        instance: *const scf_instance_t,
        snapshot: *const scf_snapshot_t,
    ) -> c_int;
    pub fn scf_iter_service_pgs(
        iter: *mut scf_iter_t,
        service: *const scf_service_t,
    ) -> c_int;
    pub fn scf_iter_next_pg(
        iter: *mut scf_iter_t,
        out: *mut scf_propertygroup_t,
    ) -> c_int;

    pub fn scf_pg_create(handle: *mut scf_handle_t)
        -> *mut scf_propertygroup_t;
    pub fn scf_pg_destroy(pg: *mut scf_propertygroup_t);
    pub fn scf_pg_delete(pg: *mut scf_propertygroup_t) -> c_int;

    pub fn scf_pg_get_name(
        pg: *const scf_propertygroup_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t;
    pub fn scf_pg_get_type(
        pg: *const scf_propertygroup_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t;
    pub fn scf_pg_get_flags(
        pg: *const scf_propertygroup_t,
        out: *mut u32,
    ) -> c_int;
    pub fn scf_pg_update(pg: *mut scf_propertygroup_t) -> c_int;
    pub fn scf_pg_get_property(
        pg: *const scf_propertygroup_t,
        name: *const c_char,
        out: *mut scf_property_t,
    ) -> c_int;

    pub fn scf_iter_pg_properties(
        iter: *mut scf_iter_t,
        pg: *const scf_propertygroup_t,
    ) -> c_int;
    pub fn scf_iter_next_property(
        iter: *mut scf_iter_t,
        out: *mut scf_property_t,
    ) -> c_int;

    pub fn scf_property_create(
        handle: *mut scf_handle_t,
    ) -> *mut scf_property_t;
    pub fn scf_property_destroy(prop: *mut scf_property_t);

    pub fn scf_property_get_name(
        prop: *const scf_property_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t;
    pub fn scf_property_type(
        prop: *const scf_property_t,
        typ: *mut scf_type_t,
    ) -> ssize_t;

    pub fn scf_iter_property_values(
        iter: *mut scf_iter_t,
        prop: *const scf_property_t,
    ) -> c_int;
    pub fn scf_iter_next_value(
        iter: *mut scf_iter_t,
        out: *mut scf_value_t,
    ) -> c_int;

    pub fn scf_value_create(handle: *mut scf_handle_t) -> *mut scf_value_t;
    pub fn scf_value_destroy(val: *mut scf_value_t);
    pub fn scf_value_reset(val: *mut scf_value_t);

    pub fn scf_value_type(val: *const scf_value_t) -> c_int;
    pub fn scf_value_base_type(val: *const scf_value_t) -> c_int;

    pub fn scf_value_get_as_string(
        val: *const scf_value_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t;
    pub fn scf_value_get_as_string_typed(
        val: *const scf_value_t,
        type_: scf_type_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t;
    pub fn scf_value_set_from_string(
        val: *mut scf_value_t,
        valtype: scf_type_t,
        valstr: *const c_char,
    ) -> c_int;

    pub fn scf_value_get_boolean(
        val: *const scf_value_t,
        out: *mut u8,
    ) -> c_int;
    pub fn scf_value_get_count(val: *const scf_value_t, out: *mut u64)
        -> c_int;
    pub fn scf_value_get_integer(
        val: *const scf_value_t,
        out: *mut i64,
    ) -> c_int;
    pub fn scf_value_get_time(
        val: *const scf_value_t,
        seconds: *mut i64,
        ns: *mut i32,
    ) -> c_int;
    pub fn scf_value_get_astring(
        val: *const scf_value_t,
        buf: *mut c_char,
        size: size_t,
    ) -> c_int;
    pub fn scf_value_get_ustring(
        val: *const scf_value_t,
        buf: *mut c_char,
        size: size_t,
    ) -> c_int;
    pub fn scf_value_get_opaque(
        val: *const scf_value_t,
        buf: *mut c_void,
        size: size_t,
    ) -> c_int;

    pub fn scf_value_set_boolean(val: *mut scf_value_t, new: u8);
    pub fn scf_value_set_count(val: *mut scf_value_t, new: u64);
    pub fn scf_value_set_integer(val: *mut scf_value_t, new: i64);
    pub fn scf_value_set_time(
        val: *mut scf_value_t,
        seconds: i64,
        ns: i32,
    ) -> c_int;
    pub fn scf_value_set_astring(
        val: *mut scf_value_t,
        new: *const c_char,
        size: size_t,
    ) -> c_int;
    pub fn scf_value_set_ustring(
        val: *mut scf_value_t,
        new: *const c_char,
        size: size_t,
    ) -> c_int;
    pub fn scf_value_set_opaque(
        val: *mut scf_value_t,
        new: *const c_void,
        size: size_t,
    ) -> c_int;

    pub fn scf_type_base_type(typ: scf_type_t, out: *mut scf_type_t) -> c_int;

    pub fn scf_transaction_create(
        handle: *mut scf_handle_t,
    ) -> *mut scf_transaction_t;
    pub fn scf_transaction_destroy(tran: *mut scf_transaction_t);
    pub fn scf_transaction_reset(tran: *mut scf_transaction_t);
    pub fn scf_transaction_reset_all(tran: *mut scf_transaction_t);

    pub fn scf_transaction_start(
        tran: *mut scf_transaction_t,
        pg: *mut scf_propertygroup_t,
    ) -> c_int;
    pub fn scf_transaction_commit(tran: *mut scf_transaction_t) -> c_int;

    pub fn scf_transaction_property_delete(
        tran: *mut scf_transaction_t,
        entry: *mut scf_transaction_entry_t,
        name: *const c_char,
    ) -> c_int;
    pub fn scf_transaction_property_new(
        tran: *mut scf_transaction_t,
        entry: *mut scf_transaction_entry_t,
        name: *const c_char,
        proptype: scf_type_t,
    ) -> c_int;
    pub fn scf_transaction_property_change(
        tran: *mut scf_transaction_t,
        entry: *mut scf_transaction_entry_t,
        name: *const c_char,
        proptype: scf_type_t,
    ) -> c_int;
    pub fn scf_transaction_property_change_type(
        tran: *mut scf_transaction_t,
        entry: *mut scf_transaction_entry_t,
        name: *const c_char,
        proptype: scf_type_t,
    ) -> c_int;

    pub fn scf_entry_create(
        handle: *mut scf_handle_t,
    ) -> *mut scf_transaction_entry_t;
    pub fn scf_entry_destroy(tran: *mut scf_transaction_entry_t);
    pub fn scf_entry_destroy_children(tran: *mut scf_transaction_entry_t);
    pub fn scf_entry_reset(tran: *mut scf_transaction_entry_t);

    pub fn scf_entry_add_value(
        tran: *mut scf_transaction_entry_t,
        value: *mut scf_value_t,
    ) -> c_int;

    pub fn smf_disable_instance(instance: *const c_char, flags: c_int)
        -> c_int;
    pub fn smf_enable_instance(instance: *const c_char, flags: c_int) -> c_int;
    pub fn smf_refresh_instance(instance: *const c_char) -> c_int;

    pub fn scf_handle_decode_fmri(
        handle: *mut scf_handle_t,
        fmri: *const c_char,
        out_scope: *mut scf_scope_t,
        out_service: *mut scf_service_t,
        out_instance: *mut scf_instance_t,
        out_pg: *mut scf_propertygroup_t,
        out_prop: *mut scf_property_t,
        flags: c_int,
    ) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! limit_test {
        ($limit:ident) => {
            let val = unsafe { scf_limit($limit) };
            println!("{} = {}", stringify!($limit), val);
            assert!(val > 0);
        };
    }

    #[test]
    fn limits() {
        limit_test!(SCF_LIMIT_MAX_NAME_LENGTH);
        limit_test!(SCF_LIMIT_MAX_VALUE_LENGTH);
        limit_test!(SCF_LIMIT_MAX_PG_TYPE_LENGTH);
        limit_test!(SCF_LIMIT_MAX_FMRI_LENGTH);
    }

    #[test]
    fn handle() {
        let handle = unsafe { scf_handle_create(SCF_VERSION) };
        assert!(!handle.is_null());
        unsafe { scf_handle_destroy(handle) };
    }
}

/*
 * Copyright 2023 Oxide Computer Company
 */

/*
 * The Service Management Facility (SMF) is available on illumos systems.  This
 * module contains unimplemented stubs for libscf routines to ease the process
 * of building higher-level crates on other platforms while using tools like
 * rust-analyzer.  If a true testing mock for SMF is wanted, that should not
 * happen at this level.
 */

#[cfg(not(target_os = "illumos"))]
mod stubs {
    #![allow(unused_variables)]

    use crate::*;
    use libc::{size_t, ssize_t};

    pub unsafe fn scf_handle_create(
        version: scf_version_t,
    ) -> *mut scf_handle_t {
        unimplemented!()
    }
    pub unsafe fn scf_handle_destroy(handle: *mut scf_handle_t) {
        unimplemented!()
    }
    pub unsafe fn scf_handle_bind(handle: *mut scf_handle_t) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_handle_unbind(handle: *mut scf_handle_t) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_handle_decorate(
        handle: *mut scf_handle_t,
        param: *const c_char,
        value: *mut scf_value_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_myname(
        handle: *mut scf_handle_t,
        out: *mut c_char,
        sz: size_t,
    ) -> ssize_t {
        unimplemented!()
    }

    pub unsafe fn scf_error() -> u32 {
        unimplemented!()
    }
    pub unsafe fn scf_strerror(error: scf_error_t) -> *const c_char {
        unimplemented!()
    }

    pub unsafe fn scf_limit(name: u32) -> ssize_t {
        unimplemented!()
    }

    pub unsafe fn scf_iter_create(
        handle: *mut scf_handle_t,
    ) -> *mut scf_iter_t {
        unimplemented!()
    }
    pub unsafe fn scf_iter_destroy(iter: *mut scf_iter_t) {
        unimplemented!()
    }
    pub unsafe fn scf_iter_reset(iter: *mut scf_iter_t) {
        unimplemented!()
    }

    pub unsafe fn scf_scope_create(
        handle: *mut scf_handle_t,
    ) -> *mut scf_scope_t {
        unimplemented!()
    }
    pub unsafe fn scf_scope_destroy(scope: *mut scf_scope_t) {
        unimplemented!()
    }
    pub unsafe fn scf_scope_get_name(
        scope: *mut scf_scope_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t {
        unimplemented!()
    }
    pub unsafe fn scf_handle_get_scope(
        handle: *mut scf_handle_t,
        name: *const c_char,
        out: *mut scf_scope_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_scope_get_service(
        scope: *mut scf_scope_t,
        name: *const c_char,
        out: *mut scf_service_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_scope_add_service(
        scope: *mut scf_scope_t,
        name: *const c_char,
        out: *mut scf_service_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_iter_handle_scopes(
        iter: *mut scf_iter_t,
        handle: *const scf_handle_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_iter_next_scope(
        iter: *mut scf_iter_t,
        out: *mut scf_scope_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_service_create(
        handle: *mut scf_handle_t,
    ) -> *mut scf_service_t {
        unimplemented!()
    }
    pub unsafe fn scf_service_destroy(service: *mut scf_service_t) {
        unimplemented!()
    }

    pub unsafe fn scf_service_get_name(
        service: *mut scf_service_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t {
        unimplemented!()
    }
    pub unsafe fn scf_service_get_instance(
        service: *mut scf_service_t,
        name: *const c_char,
        out: *mut scf_instance_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_service_add_instance(
        service: *mut scf_service_t,
        name: *const c_char,
        out: *mut scf_instance_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_iter_scope_services(
        iter: *mut scf_iter_t,
        scope: *const scf_scope_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_iter_next_service(
        iter: *mut scf_iter_t,
        out: *mut scf_service_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_instance_create(
        handle: *mut scf_handle_t,
    ) -> *mut scf_instance_t {
        unimplemented!()
    }
    pub unsafe fn scf_instance_destroy(instance: *mut scf_instance_t) {
        unimplemented!()
    }

    pub unsafe fn scf_instance_get_name(
        instance: *mut scf_instance_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t {
        unimplemented!()
    }
    pub unsafe fn scf_instance_get_pg(
        instance: *mut scf_instance_t,
        name: *const c_char,
        out: *mut scf_propertygroup_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_instance_add_pg(
        instance: *mut scf_instance_t,
        name: *const c_char,
        pgtype: *const c_char,
        flags: u32,
        out: *mut scf_propertygroup_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_instance_get_pg_composed(
        instance: *mut scf_instance_t,
        snapshot: *mut scf_snapshot_t,
        name: *const c_char,
        out: *mut scf_propertygroup_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_instance_get_snapshot(
        instance: *mut scf_instance_t,
        name: *const c_char,
        out: *mut scf_snapshot_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_instance_to_fmri(
        instance: *const scf_instance_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t {
        unimplemented!()
    }

    pub unsafe fn scf_iter_service_instances(
        iter: *mut scf_iter_t,
        service: *const scf_service_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_iter_next_instance(
        iter: *mut scf_iter_t,
        out: *mut scf_instance_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_snapshot_create(
        handle: *mut scf_handle_t,
    ) -> *mut scf_snapshot_t {
        unimplemented!()
    }
    pub unsafe fn scf_snapshot_destroy(snapshot: *mut scf_snapshot_t) {
        unimplemented!()
    }

    pub unsafe fn scf_snapshot_get_name(
        snapshot: *mut scf_snapshot_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t {
        unimplemented!()
    }
    pub unsafe fn scf_snapshot_get_parent(
        snapshot: *const scf_snapshot_t,
        inst: *mut scf_instance_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_iter_instance_snapshots(
        iter: *mut scf_iter_t,
        instance: *const scf_instance_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_iter_next_snapshot(
        iter: *mut scf_iter_t,
        out: *mut scf_snapshot_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_iter_instance_pgs(
        iter: *mut scf_iter_t,
        instance: *const scf_instance_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_iter_instance_pgs_composed(
        iter: *mut scf_iter_t,
        instance: *const scf_instance_t,
        snapshot: *const scf_snapshot_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_iter_service_pgs(
        iter: *mut scf_iter_t,
        service: *const scf_service_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_iter_next_pg(
        iter: *mut scf_iter_t,
        out: *mut scf_propertygroup_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_pg_create(
        handle: *mut scf_handle_t,
    ) -> *mut scf_propertygroup_t {
        unimplemented!()
    }
    pub unsafe fn scf_pg_destroy(pg: *mut scf_propertygroup_t) {
        unimplemented!()
    }

    pub unsafe fn scf_pg_get_name(
        pg: *mut scf_propertygroup_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t {
        unimplemented!()
    }
    pub unsafe fn scf_pg_get_type(
        pg: *mut scf_propertygroup_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t {
        unimplemented!()
    }
    pub unsafe fn scf_pg_get_flags(
        pg: *mut scf_propertygroup_t,
        out: *mut u32,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_pg_update(pg: *mut scf_propertygroup_t) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_pg_get_property(
        pg: *mut scf_propertygroup_t,
        name: *const c_char,
        out: *mut scf_property_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_iter_pg_properties(
        iter: *mut scf_iter_t,
        pg: *const scf_propertygroup_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_iter_next_property(
        iter: *mut scf_iter_t,
        out: *mut scf_property_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_property_create(
        handle: *mut scf_handle_t,
    ) -> *mut scf_property_t {
        unimplemented!()
    }
    pub unsafe fn scf_property_destroy(prop: *mut scf_property_t) {
        unimplemented!()
    }

    pub unsafe fn scf_property_get_name(
        prop: *mut scf_property_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t {
        unimplemented!()
    }
    pub unsafe fn scf_property_type(
        prop: *mut scf_property_t,
        typ: *mut scf_type_t,
    ) -> ssize_t {
        unimplemented!()
    }

    pub unsafe fn scf_iter_property_values(
        iter: *mut scf_iter_t,
        prop: *const scf_property_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_iter_next_value(
        iter: *mut scf_iter_t,
        out: *mut scf_value_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_value_create(
        handle: *mut scf_handle_t,
    ) -> *mut scf_value_t {
        unimplemented!()
    }
    pub unsafe fn scf_value_destroy(val: *mut scf_value_t) {
        unimplemented!()
    }
    pub unsafe fn scf_value_reset(val: *mut scf_value_t) {
        unimplemented!()
    }

    pub unsafe fn scf_value_type(val: *mut scf_value_t) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_value_base_type(val: *mut scf_value_t) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_value_get_as_string(
        val: *mut scf_value_t,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t {
        unimplemented!()
    }
    pub unsafe fn scf_value_set_from_string(
        val: *mut scf_value_t,
        valtype: scf_type_t,
        valstr: *const c_char,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_type_base_type(
        typ: scf_type_t,
        out: *mut scf_type_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_transaction_create(
        handle: *mut scf_handle_t,
    ) -> *mut scf_transaction_t {
        unimplemented!()
    }
    pub unsafe fn scf_transaction_destroy(tran: *mut scf_transaction_t) {
        unimplemented!()
    }
    pub unsafe fn scf_transaction_reset(tran: *mut scf_transaction_t) {
        unimplemented!()
    }
    pub unsafe fn scf_transaction_reset_all(tran: *mut scf_transaction_t) {
        unimplemented!()
    }

    pub unsafe fn scf_transaction_start(
        tran: *mut scf_transaction_t,
        pg: *mut scf_propertygroup_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_transaction_commit(
        tran: *mut scf_transaction_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_transaction_property_delete(
        tran: *mut scf_transaction_t,
        entry: *mut scf_transaction_entry_t,
        name: *const c_char,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_transaction_property_new(
        tran: *mut scf_transaction_t,
        entry: *mut scf_transaction_entry_t,
        name: *const c_char,
        proptype: scf_type_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_transaction_property_change(
        tran: *mut scf_transaction_t,
        entry: *mut scf_transaction_entry_t,
        name: *const c_char,
        proptype: scf_type_t,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn scf_transaction_property_change_type(
        tran: *mut scf_transaction_t,
        entry: *mut scf_transaction_entry_t,
        name: *const c_char,
        proptype: scf_type_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn scf_entry_create(
        handle: *mut scf_handle_t,
    ) -> *mut scf_transaction_entry_t {
        unimplemented!()
    }
    pub unsafe fn scf_entry_destroy(tran: *mut scf_transaction_entry_t) {
        unimplemented!()
    }
    pub unsafe fn scf_entry_destroy_children(
        tran: *mut scf_transaction_entry_t,
    ) {
        unimplemented!()
    }
    pub unsafe fn scf_entry_reset(tran: *mut scf_transaction_entry_t) {
        unimplemented!()
    }

    pub unsafe fn scf_entry_add_value(
        tran: *mut scf_transaction_entry_t,
        value: *mut scf_value_t,
    ) -> c_int {
        unimplemented!()
    }

    pub unsafe fn smf_disable_instance(
        instance: *const c_char,
        flags: c_int,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn smf_enable_instance(
        instance: *const c_char,
        flags: c_int,
    ) -> c_int {
        unimplemented!()
    }
    pub unsafe fn smf_refresh_instance(instance: *const c_char) -> c_int {
        unimplemented!()
    }

    #[allow(clippy::too_many_arguments)]
    pub unsafe fn scf_handle_decode_fmri(
        handle: *mut scf_handle_t,
        fmri: *const c_char,
        out_scope: *mut scf_scope_t,
        out_service: *mut scf_service_t,
        out_instance: *mut scf_instance_t,
        out_pg: *mut scf_propertygroup_t,
        out_prop: *mut scf_property_t,
        flags: c_int,
    ) -> c_int {
        unimplemented!()
    }
}

pub use stubs::*;

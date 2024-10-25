# libscf-sys

This crate contains hand-crafted native bindings for
[libscf(3LIB)](https://illumos.org/man/3LIB/libscf) that do not require
bindgen, depending only on [Committed](https://illumos.org/man/7/attributes)
parts of the library API and ABI.  This library allows programmatic access to
the [Service Management Facility (SMF)](https://illumos.org/man/7/smf) on
illumos systems.

## Licence

Unless otherwise noted, all components are dual-licenced under either the [MIT
Licence](./LICENSE-MIT) or the [Apache License Version 2.0](./LICENSE-APACHE).

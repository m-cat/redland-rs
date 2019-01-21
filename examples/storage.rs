// TODO: Delete commented-out use statements.
// TODO: Get rid of unwraps.
// TODO: Disallow unused_variables.

#![feature(custom_attribute, extern_types, libc)]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_attributes,
    // TODO: Remove this.
    unused_imports,
    unused_mut
)]

extern crate ffi_utils;
extern crate libc;
extern crate rand;
extern crate rasqal_rs;
extern crate redland_rs;
extern crate routing;
extern crate rust_sodium;
#[macro_use]
extern crate unwrap;

use libc::c_char;
use redland_rs::*;
use routing::{EntryActions, MutableData, Value};
use rust_sodium::crypto::sign;
use rust_sodium::crypto::sign::PublicKey as PublicSignKey;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::{CStr, CString};
use std::str::FromStr;
use std::{io, mem, ptr};

//////////////////////////
// Start header imports //
//////////////////////////

#[header_src = "vararg"]
pub mod vararg {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
    use super::libc;
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __off_t = libc::c_long;
    pub type __off64_t = libc::c_long;
    use super::libc;
}
#[header_src = "/home/vagrant/C2Rust/dependencies/llvm-6.0.1/build.vagrant/lib/clang/6.0.1/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::libc;
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types/__FILE.h"]
pub mod __FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub __pad1: *mut libc::c_void,
        pub __pad2: *mut libc::c_void,
        pub __pad3: *mut libc::c_void,
        pub __pad4: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    use super::libc;
    use super::libio_h::{_IO_lock_t, _IO_marker};
    use super::stddef_h::size_t;
    use super::types_h::{__off64_t, __off_t};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/libio.h"]
pub mod libio_h {
    pub type _IO_lock_t = ();
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _IO_marker {
        pub _next: *mut _IO_marker,
        pub _sbuf: *mut _IO_FILE,
        pub _pos: libc::c_int,
    }
    use super::__FILE_h::_IO_FILE;
    use super::libc;
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types/FILE.h"]
pub mod FILE_h {
    pub type FILE = _IO_FILE;
    use super::__FILE_h::_IO_FILE;
}
// #[header_src = "/usr/local/include/raptor2/raptor2.h"]
// pub mod raptor2_h {
//     pub type raptor_uri = raptor_uri_s;
//     pub type raptor_world = raptor_world_s;
//     pub type raptor_iostream = raptor_iostream_s;
//     /* *
//      * raptor_term_type:
//      * @RAPTOR_TERM_TYPE_URI: RDF URI
//      * @RAPTOR_TERM_TYPE_LITERAL: RDF literal
//      * @RAPTOR_TERM_TYPE_BLANK: RDF blank node
//      * @RAPTOR_TERM_TYPE_UNKNOWN: Internal
//      *
//      * Type of term in a #raptor_statement
//      *
//      * Node type 3 is unused but exists to preserve numeric compatibility
//      * with librdf_node_type values.
//      */
//     pub type raptor_term_type = libc::c_uint;
//     /* unused type 3 */
//     pub const RAPTOR_TERM_TYPE_BLANK: raptor_term_type = 4;
//     pub const RAPTOR_TERM_TYPE_LITERAL: raptor_term_type = 2;
//     pub const RAPTOR_TERM_TYPE_URI: raptor_term_type = 1;
//     pub const RAPTOR_TERM_TYPE_UNKNOWN: raptor_term_type = 0;
//     /* *
//      * raptor_locator:
//      * @uri: URI of location (or NULL)
//      * @file: Filename of location (or NULL)
//      * @line: Line number of location (or <0 for no line)
//      * @column: Column number of location (or <0 for no column)
//      * @byte: Byte number of location (or <0 for no byte)
//      *
//      * Location information for an error, warning or information message.
//      */
//     #[derive(Copy, Clone)]
//     #[repr(C)]
//     pub struct raptor_locator {
//         /* *
//          * raptor_term_literal_value:
//          * @string: literal string
//          * @string_len: length of string
//          * @datatype: datatype URI (or NULL)
//          * @language: literal language (or NULL)
//          * @language_len: length of language
//          *
//          * Literal term value - this typedef exists solely for use in #raptor_term
//          *
//          * Either @datatype or @language may be non-NULL but not both.
//          */
//         /* *
//          * raptor_term_blank_value:
//          * @string: literal string
//          * @string_len: length of string
//          *
//          * Blank term value - this typedef exists solely for use in #raptor_term
//          *
//          */
//         /* *
//          * raptor_term_value:
//          * @uri: uri value when term type is #RAPTOR_TERM_TYPE_URI
//          * @literal: literal value when term type is #RAPTOR_TERM_TYPE_LITERAL
//          * @blank: blank value when term type is #RAPTOR_TERM_TYPE_BLANK
//          *
//          * Term value - this typedef exists solely for use in #raptor_term
//          *
//          **/
//         pub uri: *mut raptor_uri,
//         pub file: *const libc::c_char,
//         pub line: libc::c_int,
//         pub column: libc::c_int,
//         pub byte: libc::c_int,
//     }
//     #[derive(Copy, Clone)]
//     #[repr(C)]
//     pub struct raptor_term_literal_value {
//         /* *
//          * raptor_term:
//          * @world: world
//          * @usage: usage reference count (if >0)
//          * @type: term type
//          * @value: term values per type
//          *
//          * An RDF statement term
//          *
//          */
//         pub string: *mut libc::c_uchar,
//         pub string_len: libc::c_uint,
//         pub datatype: *mut raptor_uri,
//         pub language: *mut libc::c_uchar,
//         pub language_len: libc::c_uchar,
//     }
//     #[derive(Copy, Clone)]
//     #[repr(C)]
//     pub struct raptor_term_blank_value {
//         /* *
//          * raptor_statement:
//          * @world: world pointer
//          * @usage: usage count
//          * @subject: statement subject
//          * @predicate: statement predicate
//          * @object: statement object
//          * @graph: statement graph name (or NULL if not present)
//          *
//          * An RDF triple with optional graph name (quad)
//          *
//          * See #raptor_term for a description of how the fields may be used.
//          * As returned by a parser statement_handler.
//          */
//         pub string: *mut libc::c_uchar,
//         pub string_len: libc::c_uint,
//     }
//     #[derive(Copy, Clone)]
//     #[repr(C)]
//     pub union raptor_term_value {
//         /* -*- Mode: c; c-basic-offset: 2 -*-
//          *
//          * raptor.h - Redland Parser Toolkit for RDF (Raptor) - public API
//          *
//          * Copyright (C) 2000-2013, David Beckett http://www.dajobe.org/
//          * Copyright (C) 2000-2005, University of Bristol, UK http://www.bristol.ac.uk/
//          *
//          * This package is Free Software and part of Redland http://librdf.org/
//          *
//          * It is licensed under the following three licenses as alternatives:
//          *   1. GNU Lesser General Public License (LGPL) V2.1 or any newer version
//          *   2. GNU General Public License (GPL) V2 or any newer version
//          *   3. Apache License, V2.0 or any newer version
//          *
//          * You may not use this file except in compliance with at least one of
//          * the above three licenses.
//          *
//          * See LICENSE.html or LICENSE.txt at the top of this package for the
//          * complete terms and further detail along with the license texts for
//          * the licenses in COPYING.LIB, COPYING and LICENSE-2.0.txt respectively.
//          *
//          *
//          */
//         /* Required for va_list in raptor_vsnprintf */
//         /* *
//          * RAPTOR_V2_AVAILABLE
//          *
//          * Flag for marking raptor2 API availability.
//          */
//         /* *
//          * RAPTOR_VERSION:
//          *
//          * Raptor library version number
//          *
//          * Format: major * 10000 + minor * 100 + release
//          */
//         /* *
//          * RAPTOR_VERSION_STRING:
//          *
//          * Raptor library version string
//          */
//         /* *
//          * RAPTOR_VERSION_MAJOR:
//          *
//          * Raptor library major version
//          */
//         /* *
//          * RAPTOR_VERSION_MINOR:
//          *
//          * Raptor library minor version
//          */
//         /* *
//          * RAPTOR_VERSION_RELEASE:
//          *
//          * Raptor library release
//          */
//         /* *
//          * RAPTOR_API:
//          *
//          * Macro for wrapping API function call declarations.
//          *
//          */
//         /* Use gcc 3.1+ feature to allow marking of deprecated API calls.
//          * This gives a warning during compiling.
//          */
//         /* *
//          * RAPTOR_PRINTF_FORMAT:
//          * @string_index: ignore me
//          * @first_to_check_index: ignore me
//          *
//          * Internal macro
//          */
//         /* *
//          * raptor_uri:
//          *
//          * Raptor URI Class.
//          */
//         pub uri: *mut raptor_uri,
//         pub literal: raptor_term_literal_value,
//         pub blank: raptor_term_blank_value,
//     }
//     #[derive(Copy, Clone)]
//     #[repr(C)]
//     pub struct raptor_term {
//         /* Public structure */
//         /**
//          * raptor_world:
//          *
//          * Raptor world class.
//          */
//         pub world: *mut raptor_world,
//         pub usage: libc::c_int,
//         pub type_0: raptor_term_type,
//         pub value: raptor_term_value,
//     }
//     #[derive(Copy, Clone)]
//     #[repr(C)]
//     pub struct raptor_statement {
//         /* *
//          * raptor_iostream:
//          *
//          * Raptor I/O Stream class
//          */
//         pub world: *mut raptor_world,
//         pub usage: libc::c_int,
//         pub subject: *mut raptor_term,
//         pub predicate: *mut raptor_term,
//         pub object: *mut raptor_term,
//         pub graph: *mut raptor_term,
//     }
//     pub type raptor_sequence = raptor_sequence_s;
//     use super::libc;
//     extern "C" {
//         pub type raptor_uri_s;
//         pub type raptor_world_s;
//         pub type raptor_iostream_s;
//         /* Sequence class */
//         /**
//          * raptor_sequence:
//          *
//          * Raptor sequence class
//          */
//         pub type raptor_sequence_s;
//     }
// }
// #[header_src = "/usr/local/include/rasqal/rasqal.h"]
// pub mod rasqal_h {
//     pub type rasqal_world = rasqal_world_s;
//     pub type rasqal_query_results_formatter = rasqal_query_results_formatter_s;
//     extern "C" {
//         /* *
//          * RASQAL_RAPTOR_VERSION:
//          *
//          * Version of Raptor that Rasqal was configured against.
//          */
//         /* Public structures */
//         /* *
//          * rasqal_world:
//          *
//          * Rasqal world class.
//          */
//         pub type rasqal_world_s;
//         /* *
//          * rasqal_query_results_formatter:
//          *
//          * Rasqal query results formatter class.
//          */
//         pub type rasqal_query_results_formatter_s;
//     }
// }
#[header_src = "/home/vagrant/C2Rust/redland-1.0.17/src/librdf.h"]
pub mod librdf_h {
    use rasqal_rs::{raptor_iostream, rasqal_query_results_formatter};
    use redland_rs::{librdf_model_s, librdf_world_s, raptor_statement, raptor_term, raptor_uri_s};
    /* -*- Mode: c; c-basic-offset: 2 -*-
     *
     * redland.h - Redland RDF Application Framework public API
     *
     * Copyright (C) 2000-2011, David Beckett http://www.dajobe.org/
     * Copyright (C) 2000-2005, University of Bristol, UK http://www.bristol.ac.uk/
     *
     * This package is Free Software and part of Redland http://librdf.org/
     *
     * It is licensed under the following three licenses as alternatives:
     *   1. GNU Lesser General Public License (LGPL) V2.1 or any newer version
     *   2. GNU General Public License (GPL) V2 or any newer version
     *   3. Apache License, V2.0 or any newer version
     *
     * You may not use this file except in compliance with at least one of
     * the above three licenses.
     *
     * See LICENSE.html or LICENSE.txt at the top of this package for the
     * complete terms and further detail along with the license texts for
     * the licenses in COPYING.LIB, COPYING and LICENSE-2.0.txt respectively.
     *
     *
     */
    /* raptor */
    /* rasqal: uses raptor */
    /* librdf: uses rasqal and raptor */
    /* Use gcc 3.1+ feature to allow marking of deprecated API calls.
     * This gives a warning during compiling.
     */
    /* Public defines */
    /* *
     * LIBRDF_VERSION:
     *
     * Redland librdf library version number
     *
     * Format: major * 10000 + minor * 100 + release
     */
    /* *
     * LIBRDF_VERSION_STRING:
     *
     * Redland librdf library version string
     */
    /* *
     * LIBRDF_VERSION_MAJOR:
     *
     * Redland librdf library major version
     */
    /* *
     * LIBRDF_VERSION_MINOR:
     *
     * Redland librdf library minor version
     */
    /* *
     * LIBRDF_VERSION_RELEASE:
     *
     * Redland librdf library release
     */
    /* Public typedefs (references to private structures) */
    /* *
     * librdf_uri:
     *
     * Redland URI class.
     */
    pub type librdf_uri = raptor_uri_s;
    pub type librdf_hash = librdf_hash_s;
    /* *
     * librdf_hash:
     *
     * Redland hash class.
     */
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct librdf_hash_s {
        /* *
         * librdf_node:
         *
         * Redland node class.
         */
        pub world: *mut librdf_world,
        pub identifier: *mut libc::c_char,
        pub context: *mut libc::c_void,
        pub is_open: libc::c_int,
        pub factory: *mut librdf_hash_factory_s,
    }
    pub type librdf_world = librdf_world_s;
    pub type librdf_node = raptor_term;
    pub type librdf_digest_factory = librdf_digest_factory_s;
    /* *
     * librdf_digest_factory:
     *
     * Redland digest factory class.
     */
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct librdf_digest_factory_s {
        /* *
         * librdf_query_factory:
         *
         * Redland query factory class.
         */
        /* *
         * librdf_query_results:
         *
         * Redland query results class.
         */
        pub next: *mut librdf_digest_factory_s,
        pub name: *mut libc::c_char,
        pub context_length: size_t,
        pub digest_length: size_t,
        pub init: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub update: Option<
            unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_uchar, _: size_t) -> (),
        >,
        pub final_0: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub get_digest: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_uchar>,
    }
    pub type librdf_query_factory = librdf_query_factory_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct librdf_query_factory_s {
        /* *
         * librdf_query:
         *
         * Redland query class.
         */
        pub world: *mut librdf_world,
        pub next: *mut librdf_query_factory_s,
        pub name: *mut libc::c_char,
        pub uri: *mut librdf_uri,
        pub context_length: size_t,
        pub init: Option<
            unsafe extern "C" fn(
                _: *mut librdf_query,
                _: *const libc::c_char,
                _: *mut librdf_uri,
                _: *const libc::c_uchar,
                _: *mut librdf_uri,
            ) -> libc::c_int,
        >,
        pub clone:
            Option<unsafe extern "C" fn(_: *mut librdf_query, _: *mut librdf_query) -> libc::c_int>,
        pub terminate: Option<unsafe extern "C" fn(_: *mut librdf_query) -> ()>,
        pub execute: Option<
            unsafe extern "C" fn(
                _: *mut librdf_query,
                _: *mut librdf_model,
            ) -> *mut librdf_query_results,
        >,
        pub get_limit: Option<unsafe extern "C" fn(_: *mut librdf_query) -> libc::c_int>,
        pub set_limit:
            Option<unsafe extern "C" fn(_: *mut librdf_query, _: libc::c_int) -> libc::c_int>,
        pub get_offset: Option<unsafe extern "C" fn(_: *mut librdf_query) -> libc::c_int>,
        pub set_offset:
            Option<unsafe extern "C" fn(_: *mut librdf_query, _: libc::c_int) -> libc::c_int>,
        pub results_as_stream:
            Option<unsafe extern "C" fn(_: *mut librdf_query_results) -> *mut librdf_stream>,
        pub results_get_count:
            Option<unsafe extern "C" fn(_: *mut librdf_query_results) -> libc::c_int>,
        pub results_next: Option<unsafe extern "C" fn(_: *mut librdf_query_results) -> libc::c_int>,
        pub results_finished:
            Option<unsafe extern "C" fn(_: *mut librdf_query_results) -> libc::c_int>,
        pub results_get_bindings: Option<
            unsafe extern "C" fn(
                _: *mut librdf_query_results,
                _: *mut *mut *const libc::c_char,
                _: *mut *mut librdf_node,
            ) -> libc::c_int,
        >,
        pub results_get_binding_value: Option<
            unsafe extern "C" fn(_: *mut librdf_query_results, _: libc::c_int) -> *mut librdf_node,
        >,
        pub results_get_binding_name: Option<
            unsafe extern "C" fn(
                _: *mut librdf_query_results,
                _: libc::c_int,
            ) -> *const libc::c_char,
        >,
        pub results_get_binding_value_by_name: Option<
            unsafe extern "C" fn(
                _: *mut librdf_query_results,
                _: *const libc::c_char,
            ) -> *mut librdf_node,
        >,
        pub results_get_bindings_count:
            Option<unsafe extern "C" fn(_: *mut librdf_query_results) -> libc::c_int>,
        pub free_results: Option<unsafe extern "C" fn(_: *mut librdf_query_results) -> ()>,
        pub results_is_bindings:
            Option<unsafe extern "C" fn(_: *mut librdf_query_results) -> libc::c_int>,
        pub results_is_boolean:
            Option<unsafe extern "C" fn(_: *mut librdf_query_results) -> libc::c_int>,
        pub results_is_graph:
            Option<unsafe extern "C" fn(_: *mut librdf_query_results) -> libc::c_int>,
        pub results_is_syntax:
            Option<unsafe extern "C" fn(_: *mut librdf_query_results) -> libc::c_int>,
        pub results_get_boolean:
            Option<unsafe extern "C" fn(_: *mut librdf_query_results) -> libc::c_int>,
        pub new_results_formatter: Option<
            unsafe extern "C" fn(
                _: *mut librdf_query_results,
                _: *const libc::c_char,
                _: *const libc::c_char,
                _: *mut librdf_uri,
            ) -> *mut librdf_query_results_formatter,
        >,
        pub free_results_formatter:
            Option<unsafe extern "C" fn(_: *mut librdf_query_results_formatter) -> ()>,
        pub results_formatter_write: Option<
            unsafe extern "C" fn(
                _: *mut raptor_iostream,
                _: *mut librdf_query_results_formatter,
                _: *mut librdf_query_results,
                _: *mut librdf_uri,
            ) -> libc::c_int,
        >,
    }
    pub type librdf_query_results = librdf_query_results_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct librdf_query_results_s {
        /* *
         * librdf_query_results_formatter:
         *
         * Redland query results formatter class.
         */
        pub query: *mut librdf_query,
        pub next: *mut librdf_query_results,
    }
    pub type librdf_query = librdf_query_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct librdf_query_s {
        /* *
         * librdf_stream:
         *
         * Redland stream class.
         */
        pub world: *mut librdf_world,
        pub usage: libc::c_int,
        pub context: *mut libc::c_void,
        pub factory: *mut librdf_query_factory_s,
        pub results: *mut librdf_query_results,
    }
    pub type librdf_query_results_formatter = librdf_query_results_formatter_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct librdf_query_results_formatter_s {
        /* *
         * librdf_statement:
         *
         * Redland statement class.
         */
        pub query_results: *mut librdf_query_results,
        pub formatter: *mut rasqal_query_results_formatter,
    }
    pub type librdf_stream = librdf_stream_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct librdf_stream_s {
        pub world: *mut librdf_world,
        pub context: *mut libc::c_void,
        pub is_finished: libc::c_int,
        pub is_updated: libc::c_int,
        pub is_updating: libc::c_int,
        pub current: *mut librdf_statement,
        pub map_list: *mut librdf_list,
        pub is_end_method: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
        pub next_method: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
        pub get_method:
            Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> *mut libc::c_void>,
        pub finished_method: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    }
    pub type librdf_list = librdf_list_s;
    pub type librdf_statement = raptor_statement;
    pub type librdf_model = librdf_model_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct librdf_model_factory_s {
        /* *
         * librdf_storage_factory:
         *
         * Redland storage factory class.
         */
        pub name: *mut libc::c_char,
        pub label: *mut libc::c_char,
        pub context_length: size_t,
        pub init: Option<unsafe extern "C" fn() -> ()>,
        pub terminate: Option<unsafe extern "C" fn() -> ()>,
        pub create: Option<
            unsafe extern "C" fn(
                _: *mut librdf_model,
                _: *mut librdf_storage,
                _: *mut librdf_hash,
            ) -> libc::c_int,
        >,
        pub clone: Option<unsafe extern "C" fn(_: *mut librdf_model) -> *mut librdf_model>,
        pub destroy: Option<unsafe extern "C" fn(_: *mut librdf_model) -> ()>,
        pub size: Option<unsafe extern "C" fn(_: *mut librdf_model) -> libc::c_int>,
        pub add_statement: Option<
            unsafe extern "C" fn(_: *mut librdf_model, _: *mut librdf_statement) -> libc::c_int,
        >,
        pub add_statements: Option<
            unsafe extern "C" fn(_: *mut librdf_model, _: *mut librdf_stream) -> libc::c_int,
        >,
        pub remove_statement: Option<
            unsafe extern "C" fn(_: *mut librdf_model, _: *mut librdf_statement) -> libc::c_int,
        >,
        pub contains_statement: Option<
            unsafe extern "C" fn(_: *mut librdf_model, _: *mut librdf_statement) -> libc::c_int,
        >,
        pub has_arc_in: Option<
            unsafe extern "C" fn(
                _: *mut librdf_model,
                _: *mut librdf_node,
                _: *mut librdf_node,
            ) -> libc::c_int,
        >,
        pub has_arc_out: Option<
            unsafe extern "C" fn(
                _: *mut librdf_model,
                _: *mut librdf_node,
                _: *mut librdf_node,
            ) -> libc::c_int,
        >,
        pub serialise: Option<unsafe extern "C" fn(_: *mut librdf_model) -> *mut librdf_stream>,
        pub find_statements: Option<
            unsafe extern "C" fn(
                _: *mut librdf_model,
                _: *mut librdf_statement,
            ) -> *mut librdf_stream,
        >,
        pub find_statements_with_options: Option<
            unsafe extern "C" fn(
                _: *mut librdf_model,
                _: *mut librdf_statement,
                _: *mut librdf_node,
                _: *mut librdf_hash,
            ) -> *mut librdf_stream,
        >,
        pub get_sources: Option<
            unsafe extern "C" fn(
                _: *mut librdf_model,
                _: *mut librdf_node,
                _: *mut librdf_node,
            ) -> *mut librdf_iterator,
        >,
        pub get_arcs: Option<
            unsafe extern "C" fn(
                _: *mut librdf_model,
                _: *mut librdf_node,
                _: *mut librdf_node,
            ) -> *mut librdf_iterator,
        >,
        pub get_targets: Option<
            unsafe extern "C" fn(
                _: *mut librdf_model,
                _: *mut librdf_node,
                _: *mut librdf_node,
            ) -> *mut librdf_iterator,
        >,
        pub get_arcs_in: Option<
            unsafe extern "C" fn(_: *mut librdf_model, _: *mut librdf_node) -> *mut librdf_iterator,
        >,
        pub get_arcs_out: Option<
            unsafe extern "C" fn(_: *mut librdf_model, _: *mut librdf_node) -> *mut librdf_iterator,
        >,
        pub context_add_statement: Option<
            unsafe extern "C" fn(
                _: *mut librdf_model,
                _: *mut librdf_node,
                _: *mut librdf_statement,
            ) -> libc::c_int,
        >,
        pub context_remove_statement: Option<
            unsafe extern "C" fn(
                _: *mut librdf_model,
                _: *mut librdf_node,
                _: *mut librdf_statement,
            ) -> libc::c_int,
        >,
        pub context_serialize: Option<
            unsafe extern "C" fn(_: *mut librdf_model, _: *mut librdf_node) -> *mut librdf_stream,
        >,
        pub query_execute: Option<
            unsafe extern "C" fn(
                _: *mut librdf_model,
                _: *mut librdf_query,
            ) -> *mut librdf_query_results,
        >,
        pub sync: Option<unsafe extern "C" fn(_: *mut librdf_model) -> libc::c_int>,
        pub context_add_statements: Option<
            unsafe extern "C" fn(
                _: *mut librdf_model,
                _: *mut librdf_node,
                _: *mut librdf_stream,
            ) -> libc::c_int,
        >,
        pub context_remove_statements:
            Option<unsafe extern "C" fn(_: *mut librdf_model, _: *mut librdf_node) -> libc::c_int>,
        pub get_storage: Option<unsafe extern "C" fn(_: *mut librdf_model) -> *mut librdf_storage>,
        pub find_statements_in_context: Option<
            unsafe extern "C" fn(
                _: *mut librdf_model,
                _: *mut librdf_statement,
                _: *mut librdf_node,
            ) -> *mut librdf_stream,
        >,
        pub get_contexts:
            Option<unsafe extern "C" fn(_: *mut librdf_model) -> *mut librdf_iterator>,
        pub get_feature: Option<
            unsafe extern "C" fn(_: *mut librdf_model, _: *mut librdf_uri) -> *mut librdf_node,
        >,
        pub set_feature: Option<
            unsafe extern "C" fn(
                _: *mut librdf_model,
                _: *mut librdf_uri,
                _: *mut librdf_node,
            ) -> libc::c_int,
        >,
        pub transaction_start: Option<unsafe extern "C" fn(_: *mut librdf_model) -> libc::c_int>,
        pub transaction_start_with_handle:
            Option<unsafe extern "C" fn(_: *mut librdf_model, _: *mut libc::c_void) -> libc::c_int>,
        pub transaction_commit: Option<unsafe extern "C" fn(_: *mut librdf_model) -> libc::c_int>,
        pub transaction_rollback: Option<unsafe extern "C" fn(_: *mut librdf_model) -> libc::c_int>,
        pub transaction_get_handle:
            Option<unsafe extern "C" fn(_: *mut librdf_model) -> *mut libc::c_void>,
    }
    pub type librdf_iterator = librdf_iterator_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct librdf_iterator_s {
        /* *
         * librdf_list:
         *
         * Redland list class.
         */
        pub world: *mut librdf_world,
        pub context: *mut libc::c_void,
        pub is_finished: libc::c_int,
        pub is_updated: libc::c_int,
        pub is_updating: libc::c_int,
        pub current: *mut libc::c_void,
        pub map_list: *mut librdf_list,
        pub is_end_method: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
        pub next_method: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
        pub get_method:
            Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> *mut libc::c_void>,
        pub finished_method: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    }
    pub type librdf_storage = librdf_storage_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct librdf_storage_s {
        /* -*- Mode: c; c-basic-offset: 2 -*-
         *
         * rdf_init.h - Overall library initialisation / termination and memory
         *              management prototypes
         *
         * Copyright (C) 2000-2008, David Beckett http://www.dajobe.org/
         * Copyright (C) 2000-2004, University of Bristol, UK http://www.bristol.ac.uk/
         *
         * This package is Free Software and part of Redland http://librdf.org/
         *
         * It is licensed under the following three licenses as alternatives:
         *   1. GNU Lesser General Public License (LGPL) V2.1 or any newer version
         *   2. GNU General Public License (GPL) V2 or any newer version
         *   3. Apache License, V2.0 or any newer version
         *
         * You may not use this file except in compliance with at least one of
         * the above three licenses.
         *
         * See LICENSE.html or LICENSE.txt at the top of this package for the
         * complete terms and further detail along with the license texts for
         * the licenses in COPYING.LIB, COPYING and LICENSE-2.0.txt respectively.
         *
         *
         */
        pub world: *mut librdf_world,
        pub usage: libc::c_int,
        pub model: *mut librdf_model,
        pub instance: *mut libc::c_void,
        pub index_contexts: libc::c_int,
        pub factory: *mut librdf_storage_factory_s,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct librdf_storage_factory_s {
        /* -*- Mode: c; c-basic-offset: 2 -*-
         *
         * rdf_hash_internal.h - Internal RDF Hash Factory and Hash definitions
         *
         * Copyright (C) 2000-2008, David Beckett http://www.dajobe.org/
         * Copyright (C) 2000-2005, University of Bristol, UK http://www.bristol.ac.uk/
         *
         * This package is Free Software and part of Redland http://librdf.org/
         *
         * It is licensed under the following three licenses as alternatives:
         *   1. GNU Lesser General Public License (LGPL) V2.1 or any newer version
         *   2. GNU General Public License (GPL) V2 or any newer version
         *   3. Apache License, V2.0 or any newer version
         *
         * You may not use this file except in compliance with at least one of
         * the above three licenses.
         *
         * See LICENSE.html or LICENSE.txt at the top of this package for the
         * complete terms and further detail along with the license texts for
         * the licenses in COPYING.LIB, COPYING and LICENSE-2.0.txt respectively.
         *
         *
         */
        /* * data type used to describe hash key and data */
        pub version: libc::c_int,
        pub name: *mut libc::c_char,
        pub label: *mut libc::c_char,
        pub init: Option<
            unsafe extern "C" fn(
                _: *mut librdf_storage,
                _: *const libc::c_char,
                _: *mut librdf_hash,
            ) -> libc::c_int,
        >,
        pub clone: Option<
            unsafe extern "C" fn(_: *mut librdf_storage, _: *mut librdf_storage) -> libc::c_int,
        >,
        pub terminate: Option<unsafe extern "C" fn(_: *mut librdf_storage) -> ()>,
        pub open: Option<
            unsafe extern "C" fn(_: *mut librdf_storage, _: *mut librdf_model) -> libc::c_int,
        >,
        pub close: Option<unsafe extern "C" fn(_: *mut librdf_storage) -> libc::c_int>,
        pub size: Option<unsafe extern "C" fn(_: *mut librdf_storage) -> libc::c_int>,
        pub add_statement: Option<
            unsafe extern "C" fn(_: *mut librdf_storage, _: *mut librdf_statement) -> libc::c_int,
        >,
        pub add_statements: Option<
            unsafe extern "C" fn(_: *mut librdf_storage, _: *mut librdf_stream) -> libc::c_int,
        >,
        pub remove_statement: Option<
            unsafe extern "C" fn(_: *mut librdf_storage, _: *mut librdf_statement) -> libc::c_int,
        >,
        pub contains_statement: Option<
            unsafe extern "C" fn(_: *mut librdf_storage, _: *mut librdf_statement) -> libc::c_int,
        >,
        pub has_arc_in: Option<
            unsafe extern "C" fn(
                _: *mut librdf_storage,
                _: *mut librdf_node,
                _: *mut librdf_node,
            ) -> libc::c_int,
        >,
        pub has_arc_out: Option<
            unsafe extern "C" fn(
                _: *mut librdf_storage,
                _: *mut librdf_node,
                _: *mut librdf_node,
            ) -> libc::c_int,
        >,
        pub serialise: Option<unsafe extern "C" fn(_: *mut librdf_storage) -> *mut librdf_stream>,
        pub find_statements: Option<
            unsafe extern "C" fn(
                _: *mut librdf_storage,
                _: *mut librdf_statement,
            ) -> *mut librdf_stream,
        >,
        pub find_statements_with_options: Option<
            unsafe extern "C" fn(
                _: *mut librdf_storage,
                _: *mut librdf_statement,
                _: *mut librdf_node,
                _: *mut librdf_hash,
            ) -> *mut librdf_stream,
        >,
        pub find_sources: Option<
            unsafe extern "C" fn(
                _: *mut librdf_storage,
                _: *mut librdf_node,
                _: *mut librdf_node,
            ) -> *mut librdf_iterator,
        >,
        pub find_arcs: Option<
            unsafe extern "C" fn(
                _: *mut librdf_storage,
                _: *mut librdf_node,
                _: *mut librdf_node,
            ) -> *mut librdf_iterator,
        >,
        pub find_targets: Option<
            unsafe extern "C" fn(
                _: *mut librdf_storage,
                _: *mut librdf_node,
                _: *mut librdf_node,
            ) -> *mut librdf_iterator,
        >,
        pub get_arcs_in: Option<
            unsafe extern "C" fn(
                _: *mut librdf_storage,
                _: *mut librdf_node,
            ) -> *mut librdf_iterator,
        >,
        pub get_arcs_out: Option<
            unsafe extern "C" fn(
                _: *mut librdf_storage,
                _: *mut librdf_node,
            ) -> *mut librdf_iterator,
        >,
        pub context_add_statement: Option<
            unsafe extern "C" fn(
                _: *mut librdf_storage,
                _: *mut librdf_node,
                _: *mut librdf_statement,
            ) -> libc::c_int,
        >,
        pub context_remove_statement: Option<
            unsafe extern "C" fn(
                _: *mut librdf_storage,
                _: *mut librdf_node,
                _: *mut librdf_statement,
            ) -> libc::c_int,
        >,
        pub context_serialise: Option<
            unsafe extern "C" fn(_: *mut librdf_storage, _: *mut librdf_node) -> *mut librdf_stream,
        >,
        pub sync: Option<unsafe extern "C" fn(_: *mut librdf_storage) -> libc::c_int>,
        pub context_add_statements: Option<
            unsafe extern "C" fn(
                _: *mut librdf_storage,
                _: *mut librdf_node,
                _: *mut librdf_stream,
            ) -> libc::c_int,
        >,
        pub context_remove_statements: Option<
            unsafe extern "C" fn(_: *mut librdf_storage, _: *mut librdf_node) -> libc::c_int,
        >,
        pub find_statements_in_context: Option<
            unsafe extern "C" fn(
                _: *mut librdf_storage,
                _: *mut librdf_statement,
                _: *mut librdf_node,
            ) -> *mut librdf_stream,
        >,
        pub get_contexts:
            Option<unsafe extern "C" fn(_: *mut librdf_storage) -> *mut librdf_iterator>,
        pub get_feature: Option<
            unsafe extern "C" fn(_: *mut librdf_storage, _: *mut librdf_uri) -> *mut librdf_node,
        >,
        pub set_feature: Option<
            unsafe extern "C" fn(
                _: *mut librdf_storage,
                _: *mut librdf_uri,
                _: *mut librdf_node,
            ) -> libc::c_int,
        >,
        pub transaction_start: Option<unsafe extern "C" fn(_: *mut librdf_storage) -> libc::c_int>,
        pub transaction_start_with_handle: Option<
            unsafe extern "C" fn(_: *mut librdf_storage, _: *mut libc::c_void) -> libc::c_int,
        >,
        pub transaction_commit: Option<unsafe extern "C" fn(_: *mut librdf_storage) -> libc::c_int>,
        pub transaction_rollback:
            Option<unsafe extern "C" fn(_: *mut librdf_storage) -> libc::c_int>,
        pub transaction_get_handle:
            Option<unsafe extern "C" fn(_: *mut librdf_storage) -> *mut libc::c_void>,
        pub supports_query: Option<
            unsafe extern "C" fn(_: *mut librdf_storage, _: *mut librdf_query) -> libc::c_int,
        >,
        pub query_execute: Option<
            unsafe extern "C" fn(
                _: *mut librdf_storage,
                _: *mut librdf_query,
            ) -> *mut librdf_query_results,
        >,
    }
    pub type librdf_storage_factory = librdf_storage_factory_s;
    use super::libc;
    // use super::raptor2_h::{
    //     raptor_iostream, raptor_sequence, raptor_statement, raptor_term, raptor_uri_s, raptor_world,
    // };
    // use super::rasqal_h::{rasqal_query_results_formatter, rasqal_world};
    use super::rdf_hash_internal_h::{
        librdf_hash_datum, librdf_hash_factory, librdf_hash_factory_s,
    };
    use super::rdf_init_h::{librdf_raptor_init_handler, librdf_rasqal_init_handler};
    use super::rdf_log_h::{librdf_log_func, librdf_log_level_func, librdf_log_message};
    use super::stddef_h::size_t;
    extern "C" {
        pub type librdf_list_s;
    }
}
#[header_src = "/home/vagrant/C2Rust/redland-1.0.17/src/rdf_init.h"]
pub mod rdf_init_h {
    pub type librdf_rasqal_init_handler =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut rasqal_world) -> ()>;
    pub type librdf_raptor_init_handler =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut raptor_world) -> ()>;
    use super::libc;
    use rasqal_rs::raptor_world;
    use rasqal_rs::rasqal_world;
    // use super::rasqal_h::rasqal_world;
}
#[header_src = "/home/vagrant/C2Rust/redland-1.0.17/src/rdf_hash_internal.h"]
pub mod rdf_hash_internal_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct librdf_hash_factory_s {
        pub next: *mut librdf_hash_factory_s,
        pub name: *mut libc::c_char,
        pub context_length: size_t,
        pub cursor_context_length: size_t,
        pub clone: Option<
            unsafe extern "C" fn(
                _: *mut librdf_hash,
                _: *mut libc::c_void,
                _: *mut libc::c_char,
                _: *mut libc::c_void,
            ) -> libc::c_int,
        >,
        pub create:
            Option<unsafe extern "C" fn(_: *mut librdf_hash, _: *mut libc::c_void) -> libc::c_int>,
        pub destroy: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
        pub open: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: *const libc::c_char,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
                _: *mut librdf_hash,
            ) -> libc::c_int,
        >,
        pub close: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
        pub values_count: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
        pub put: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: *mut librdf_hash_datum,
                _: *mut librdf_hash_datum,
            ) -> libc::c_int,
        >,
        pub exists: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: *mut librdf_hash_datum,
                _: *mut librdf_hash_datum,
            ) -> libc::c_int,
        >,
        pub delete_key: Option<
            unsafe extern "C" fn(_: *mut libc::c_void, _: *mut librdf_hash_datum) -> libc::c_int,
        >,
        pub delete_key_value: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: *mut librdf_hash_datum,
                _: *mut librdf_hash_datum,
            ) -> libc::c_int,
        >,
        pub sync: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
        pub get_fd: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
        pub cursor_init:
            Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_int>,
        pub cursor_get: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: *mut librdf_hash_datum,
                _: *mut librdf_hash_datum,
                _: libc::c_uint,
            ) -> libc::c_int,
        >,
        pub cursor_finish: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    }
    pub type librdf_hash_datum = librdf_hash_datum_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct librdf_hash_datum_s {
        pub world: *mut librdf_world,
        pub data: *mut libc::c_void,
        pub size: size_t,
        pub next: *mut librdf_hash_datum_s,
    }
    pub type librdf_hash_factory = librdf_hash_factory_s;
    use super::libc;
    use super::librdf_h::{librdf_hash, librdf_iterator, librdf_world};
    use super::stddef_h::size_t;
    extern "C" {
        /* constructor / destructor for above */
        #[no_mangle]
        pub fn librdf_new_hash_datum(
            world: *mut librdf_world,
            data: *mut libc::c_void,
            size: size_t,
        ) -> *mut librdf_hash_datum;
        #[no_mangle]
        pub fn librdf_free_hash_datum(ptr: *mut librdf_hash_datum);
        /* methods */
        /* open/create hash with identifier and options  */
        #[no_mangle]
        pub fn librdf_hash_open(
            hash: *mut librdf_hash,
            identifier: *const libc::c_char,
            mode: libc::c_int,
            is_writable: libc::c_int,
            is_new: libc::c_int,
            options: *mut librdf_hash,
        ) -> libc::c_int;
        /* end hash association */
        #[no_mangle]
        pub fn librdf_hash_close(hash: *mut librdf_hash) -> libc::c_int;
        /* how many values */
        #[no_mangle]
        pub fn librdf_hash_values_count(hash: *mut librdf_hash) -> libc::c_int;
        /* retrieve all values for a given hash key according to flags */
        #[no_mangle]
        pub fn librdf_hash_get_all(
            hash: *mut librdf_hash,
            key: *mut librdf_hash_datum,
            value: *mut librdf_hash_datum,
        ) -> *mut librdf_iterator;
        /* insert a key/value pair */
        #[no_mangle]
        pub fn librdf_hash_put(
            hash: *mut librdf_hash,
            key: *mut librdf_hash_datum,
            value: *mut librdf_hash_datum,
        ) -> libc::c_int;
        /* returns true if key exists in hash, without returning value */
        #[no_mangle]
        pub fn librdf_hash_exists(
            hash: *mut librdf_hash,
            key: *mut librdf_hash_datum,
            value: *mut librdf_hash_datum,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn librdf_hash_delete(
            hash: *mut librdf_hash,
            key: *mut librdf_hash_datum,
            value: *mut librdf_hash_datum,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn librdf_hash_keys(
            hash: *mut librdf_hash,
            key: *mut librdf_hash_datum,
        ) -> *mut librdf_iterator;
        /* flush any cached information to disk */
        #[no_mangle]
        pub fn librdf_hash_sync(hash: *mut librdf_hash) -> libc::c_int;
    }
}
#[header_src = "/home/vagrant/C2Rust/redland-1.0.17/src/rdf_log.h"]
pub mod rdf_log_h {
    /* *
     * librdf_log_message:
     *
     * Structure for storing parts of a log message generated by Redland.
     */
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct librdf_log_message {
        pub code: libc::c_int,
        pub level: librdf_log_level,
        pub facility: librdf_log_facility,
        pub message: *const libc::c_char,
        pub locator: *mut raptor_locator,
    }
    /* *
     * librdf_log_facility:
     * @LIBRDF_FROM_CONCEPTS: Concepts
     * @LIBRDF_FROM_DIGEST: Digest
     * @LIBRDF_FROM_FILES: Files
     * @LIBRDF_FROM_HASH: Hash
     * @LIBRDF_FROM_INIT: Init
     * @LIBRDF_FROM_ITERATOR: Iterator
     * @LIBRDF_FROM_LIST: List
     * @LIBRDF_FROM_MODEL: Model
     * @LIBRDF_FROM_NODE: Node
     * @LIBRDF_FROM_PARSER: Parser
     * @LIBRDF_FROM_QUERY: Query
     * @LIBRDF_FROM_SERIALIZER: Serializer
     * @LIBRDF_FROM_STATEMENT: Statement
     * @LIBRDF_FROM_STORAGE: Storage
     * @LIBRDF_FROM_STREAM: Stream
     * @LIBRDF_FROM_URI: URI
     * @LIBRDF_FROM_UTF8: UTF8
     * @LIBRDF_FROM_MEMORY: Memory
     * @LIBRDF_FROM_NONE: Associated with no part.
     * @LIBRDF_FROM_RAPTOR: Raptor library (parser or serializer; Raptor 2.0.0+).
     * @LIBRDF_FROM_LAST: Internal, never returned.
     *
     * Indicates the part of the system that generated the log message.
     */
    pub type librdf_log_facility = libc::c_uint;
    pub const LIBRDF_FROM_LAST: librdf_log_facility = 19;
    pub const LIBRDF_FROM_RAPTOR: librdf_log_facility = 19;
    pub const LIBRDF_FROM_MEMORY: librdf_log_facility = 18;
    pub const LIBRDF_FROM_UTF8: librdf_log_facility = 17;
    pub const LIBRDF_FROM_URI: librdf_log_facility = 16;
    pub const LIBRDF_FROM_STREAM: librdf_log_facility = 15;
    pub const LIBRDF_FROM_STORAGE: librdf_log_facility = 14;
    pub const LIBRDF_FROM_STATEMENT: librdf_log_facility = 13;
    pub const LIBRDF_FROM_SERIALIZER: librdf_log_facility = 12;
    pub const LIBRDF_FROM_QUERY: librdf_log_facility = 11;
    pub const LIBRDF_FROM_PARSER: librdf_log_facility = 10;
    pub const LIBRDF_FROM_NODE: librdf_log_facility = 9;
    pub const LIBRDF_FROM_MODEL: librdf_log_facility = 8;
    pub const LIBRDF_FROM_LIST: librdf_log_facility = 7;
    pub const LIBRDF_FROM_ITERATOR: librdf_log_facility = 6;
    pub const LIBRDF_FROM_INIT: librdf_log_facility = 5;
    pub const LIBRDF_FROM_HASH: librdf_log_facility = 4;
    pub const LIBRDF_FROM_FILES: librdf_log_facility = 3;
    pub const LIBRDF_FROM_DIGEST: librdf_log_facility = 2;
    pub const LIBRDF_FROM_CONCEPTS: librdf_log_facility = 1;
    pub const LIBRDF_FROM_NONE: librdf_log_facility = 0;
    /* -*- Mode: c; c-basic-offset: 2 -*-
     *
     * rdf_log.h - RDF logging interfaces
     *
     * Copyright (C) 2004-2008, David Beckett http://www.dajobe.org/
     * Copyright (C) 2004-2005, University of Bristol, UK http://www.bristol.ac.uk/
     *
     * This package is Free Software and part of Redland http://librdf.org/
     *
     * It is licensed under the following three licenses as alternatives:
     *   1. GNU Lesser General Public License (LGPL) V2.1 or any newer version
     *   2. GNU General Public License (GPL) V2 or any newer version
     *   3. Apache License, V2.0 or any newer version
     *
     * You may not use this file except in compliance with at least one of
     * the above three licenses.
     *
     * See LICENSE.html or LICENSE.txt at the top of this package for the
     * complete terms and further detail along with the license texts for
     * the licenses in COPYING.LIB, COPYING and LICENSE-2.0.txt respectively.
     *
     *
     */
    /* *
     * librdf_log_level:
     * @LIBRDF_LOG_NONE: No level
     * @LIBRDF_LOG_DEBUG: Debug.
     * @LIBRDF_LOG_INFO: Information.
     * @LIBRDF_LOG_WARN: Warning.
     * @LIBRDF_LOG_ERROR: Recoverable error.  Program can continue.
     * @LIBRDF_LOG_FATAL: Fatal error.  Program will abort if this is not caught.
     * @LIBRDF_LOG_LAST: Internal, never returned.
     *
     * Indicates the level of the log message.
     */
    pub type librdf_log_level = libc::c_uint;
    pub const LIBRDF_LOG_LAST: librdf_log_level = 5;
    pub const LIBRDF_LOG_FATAL: librdf_log_level = 5;
    pub const LIBRDF_LOG_ERROR: librdf_log_level = 4;
    pub const LIBRDF_LOG_WARN: librdf_log_level = 3;
    pub const LIBRDF_LOG_INFO: librdf_log_level = 2;
    pub const LIBRDF_LOG_DEBUG: librdf_log_level = 1;
    pub const LIBRDF_LOG_NONE: librdf_log_level = 0;
    /* *
     * librdf_log_func:
     * @user_data: User data pointer
     * @message: Log message structure pointer.
     *
     * Handler for all log levels.
     *
     * Return value: non-zero to indicate log message has been handled
     */
    pub type librdf_log_func = Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: *mut librdf_log_message) -> libc::c_int,
    >;
    /* *
     * librdf_log_level_func:
     * @user_data: User data pointer
     * @message: Log message.
     * @arguments: Message arguments.
     *
     * Handler for one log level, for the warning and error levels ONLY.
     * Used by #librdf_world_set_warning and #librdf_world_set_error.
     *
     * Return value: non-zero to indicate log message has been handled
     */
    pub type librdf_log_level_func = Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const libc::c_char,
            _: *mut __va_list_tag,
        ) -> libc::c_int,
    >;
    use super::libc;
    use super::librdf_h::librdf_world;
    use super::vararg::__va_list_tag;
    use rasqal_rs::raptor_locator;
    extern "C" {
        #[no_mangle]
        pub fn librdf_log(
            world: *mut librdf_world,
            code: libc::c_int,
            level: librdf_log_level,
            facility: librdf_log_facility,
            locator: *mut libc::c_void,
            message: *const libc::c_char,
            ...
        );
    }
}
#[header_src = "/home/vagrant/C2Rust/redland-1.0.17/src/rdf_iterator.h"]
pub mod rdf_iterator_h {
    /* *
     * librdf_iterator_get_method_flags:
     * @LIBRDF_ITERATOR_GET_METHOD_GET_CONTEXT: get context from iterator - implementing librdf_iterator_get_object()
     * @LIBRDF_ITERATOR_GET_METHOD_GET_OBJECT: get object from iterator - implementing librdf_iterator_get_context()
     * @LIBRDF_ITERATOR_GET_METHOD_GET_KEY: get iterator key object from iterator - implementing librdf_iterator_get_key()
     * @LIBRDF_ITERATOR_GET_METHOD_GET_VALUE: get iterator value from iterator - implementing librdf_iterator_get_value()
     *
     * Flags for librdf_new_iterator() get_method function pointer.
     */
    /* iterator get_method flags */
    pub type unnamed = libc::c_uint;
    pub const LIBRDF_ITERATOR_GET_METHOD_GET_VALUE: unnamed = 3;
    pub const LIBRDF_ITERATOR_GET_METHOD_GET_KEY: unnamed = 2;
    pub const LIBRDF_ITERATOR_GET_METHOD_GET_CONTEXT: unnamed = 1;
    pub const LIBRDF_ITERATOR_GET_METHOD_GET_OBJECT: unnamed = 0;
    use super::libc;
    use super::librdf_h::{librdf_iterator, librdf_world};
    extern "C" {
        #[no_mangle]
        pub fn librdf_new_iterator(
            world: *mut librdf_world,
            context: *mut libc::c_void,
            is_end_method: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
            next_method: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
            get_method: Option<
                unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> *mut libc::c_void,
            >,
            finished_method: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> *mut librdf_iterator;
        #[no_mangle]
        pub fn librdf_free_iterator(iterator: *mut librdf_iterator);
        #[no_mangle]
        pub fn librdf_iterator_end(iterator: *mut librdf_iterator) -> libc::c_int;
        #[no_mangle]
        pub fn librdf_iterator_next(iterator: *mut librdf_iterator) -> libc::c_int;
        #[no_mangle]
        pub fn librdf_iterator_get_object(iterator: *mut librdf_iterator) -> *mut libc::c_void;
        #[no_mangle]
        pub fn librdf_iterator_get_context(iterator: *mut librdf_iterator) -> *mut libc::c_void;
        #[no_mangle]
        pub fn librdf_iterator_get_key(iterator: *mut librdf_iterator) -> *mut libc::c_void;
        #[no_mangle]
        pub fn librdf_iterator_get_value(iterator: *mut librdf_iterator) -> *mut libc::c_void;
        #[no_mangle]
        pub fn librdf_new_empty_iterator(world: *mut librdf_world) -> *mut librdf_iterator;
    }
}
#[header_src = "/home/vagrant/C2Rust/redland-1.0.17/src/rdf_statement.h"]
pub mod rdf_statement_h {
    /* -*- Mode: c; c-basic-offset: 2 -*-
     *
     * rdf_statement.h - RDF Statement definition
     *
     * Copyright (C) 2000-2008, David Beckett http://www.dajobe.org/
     * Copyright (C) 2000-2004, University of Bristol, UK http://www.bristol.ac.uk/
     *
     * This package is Free Software and part of Redland http://librdf.org/
     *
     * It is licensed under the following three licenses as alternatives:
     *   1. GNU Lesser General Public License (LGPL) V2.1 or any newer version
     *   2. GNU General Public License (GPL) V2 or any newer version
     *   3. Apache License, V2.0 or any newer version
     *
     * You may not use this file except in compliance with at least one of
     * the above three licenses.
     *
     * See LICENSE.html or LICENSE.txt at the top of this package for the
     * complete terms and further detail along with the license texts for
     * the licenses in COPYING.LIB, COPYING and LICENSE-2.0.txt respectively.
     *
     *
     */
    /* *
     * librdf_statement_part:
     * @LIBRDF_STATEMENT_SUBJECT: Subject of a statement.
     * @LIBRDF_STATEMENT_PREDICATE: Predicate of a statement.
     * @LIBRDF_STATEMENT_OBJECT: Object of a statement.
     * @LIBRDF_STATEMENT_ALL: All parts of a statement.
     *
     * Flags that are or-ed to indicate statement parts.
     *
     * Used in fields arguments to methods such as the public
     * librdf_statement_encode_parts() librdf_statement_decode_parts()
     * librdf_new_stream_from_node_iterator().
     */
    pub type librdf_statement_part = libc::c_uint;
    /* must be a combination of all of the above */
    pub const LIBRDF_STATEMENT_ALL: librdf_statement_part = 7;
    pub const LIBRDF_STATEMENT_OBJECT: librdf_statement_part = 4;
    pub const LIBRDF_STATEMENT_PREDICATE: librdf_statement_part = 2;
    pub const LIBRDF_STATEMENT_SUBJECT: librdf_statement_part = 1;
    use super::libc;
    use super::librdf_h::{librdf_node, librdf_statement, librdf_world};
    use super::stddef_h::size_t;
    extern "C" {
        /* Create a new Statement from an existing Statement - DEEP CLONE */
        #[no_mangle]
        pub fn librdf_new_statement_from_statement(
            statement: *mut librdf_statement,
        ) -> *mut librdf_statement;
        /* Init a statically allocated statement */
        #[no_mangle]
        pub fn librdf_statement_init(world: *mut librdf_world, statement: *mut librdf_statement);
        /* Clear a statically allocated statement */
        #[no_mangle]
        pub fn librdf_statement_clear(statement: *mut librdf_statement);
        /* destructor */
        #[no_mangle]
        pub fn librdf_free_statement(statement: *mut librdf_statement);
        /* functions / methods */
        #[no_mangle]
        pub fn librdf_statement_get_subject(statement: *mut librdf_statement) -> *mut librdf_node;
        #[no_mangle]
        pub fn librdf_statement_set_subject(
            statement: *mut librdf_statement,
            node: *mut librdf_node,
        );
        #[no_mangle]
        pub fn librdf_statement_get_predicate(statement: *mut librdf_statement)
            -> *mut librdf_node;
        #[no_mangle]
        pub fn librdf_statement_set_predicate(
            statement: *mut librdf_statement,
            node: *mut librdf_node,
        );
        #[no_mangle]
        pub fn librdf_statement_get_object(statement: *mut librdf_statement) -> *mut librdf_node;
        #[no_mangle]
        pub fn librdf_statement_set_object(
            statement: *mut librdf_statement,
            node: *mut librdf_node,
        );
        #[no_mangle]
        pub fn librdf_statement_encode2(
            world: *mut librdf_world,
            statement: *mut librdf_statement,
            buffer: *mut libc::c_uchar,
            length: size_t,
        ) -> size_t;
        #[no_mangle]
        pub fn librdf_statement_encode_parts2(
            world: *mut librdf_world,
            statement: *mut librdf_statement,
            context_node: *mut librdf_node,
            buffer: *mut libc::c_uchar,
            length: size_t,
            fields: librdf_statement_part,
        ) -> size_t;
        #[no_mangle]
        pub fn librdf_statement_decode2(
            world: *mut librdf_world,
            statement: *mut librdf_statement,
            context_node: *mut *mut librdf_node,
            buffer: *mut libc::c_uchar,
            length: size_t,
        ) -> size_t;
    }
}
#[header_src = "/home/vagrant/C2Rust/redland-1.0.17/src/rdf_storage_module.h"]
pub mod rdf_storage_module_h {
    /* -*- Mode: c; c-basic-offset: 2 -*-
     *
     * librdf_storage_module.h - Interface for a Redland storage module
     *
     * Copyright (C) 2000-2008, David Beckett http://www.dajobe.org/
     * Copyright (C) 2000-2005, University of Bristol, UK http://www.bristol.ac.uk/
     *
     * This package is Free Software and part of Redland http://librdf.org/
     *
     * It is licensed under the following three licenses as alternatives:
     *   1. GNU Lesser General Public License (LGPL) V2.1 or any newer version
     *   2. GNU General Public License (GPL) V2 or any newer version
     *   3. Apache License, V2.0 or any newer version
     *
     * You may not use this file except in compliance with at least one of
     * the above three licenses.
     *
     * See LICENSE.html or LICENSE.txt at the top of this package for the
     * complete terms and further detail along with the license texts for
     * the licenses in COPYING.LIB, COPYING and LICENSE-2.0.txt respectively.
     */
    /* *
     * librdf_storage_instance:
     *
     * Opaque storage module instance handle.
     *
     * For use with a storage module and the librdf_storage_get_instance()
     * and librdf_storage_set_instance() functions.  The instance handle
     * should be set in the #librdf_storage_factory init factory method.
     */
    pub type librdf_storage_instance = *mut libc::c_void;
    use super::libc;
}
#[header_src = "/home/vagrant/C2Rust/redland-1.0.17/src/rdf_stream.h"]
pub mod rdf_stream_h {
    /* *
     * librdf_stream_map_free_context_handler:
     * @map_context: Map data context pointer.
     *
     * Free handler function for a #librdf_stream map operation.
     *
     * See librdf_stream_add_map().
     */
    pub type librdf_stream_map_free_context_handler =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
    /* -*- Mode: c; c-basic-offset: 2 -*-
     *
     * rdf_stream.h - RDF Stream interface and definitions
     *
     * Copyright (C) 2000-2008, David Beckett http://www.dajobe.org/
     * Copyright (C) 2000-2004, University of Bristol, UK http://www.bristol.ac.uk/
     *
     * This package is Free Software and part of Redland http://librdf.org/
     *
     * It is licensed under the following three licenses as alternatives:
     *   1. GNU Lesser General Public License (LGPL) V2.1 or any newer version
     *   2. GNU General Public License (GPL) V2 or any newer version
     *   3. Apache License, V2.0 or any newer version
     *
     * You may not use this file except in compliance with at least one of
     * the above three licenses.
     *
     * See LICENSE.html or LICENSE.txt at the top of this package for the
     * complete terms and further detail along with the license texts for
     * the licenses in COPYING.LIB, COPYING and LICENSE-2.0.txt respectively.
     *
     *
     */
    /* *
     * librdf_stream_map_handler:
     * @stream: Stream that this map is operating over.
     * @map_context: Map data context pointer.
     * @item: Pointer to the current item in the iteration.
     *
     * Map function for a #librdf_stream map operation.
     *
     * See librdf_stream_add_map().
     *
     * Returns: item in keep the iteration or NULL to remove it
     */
    pub type librdf_stream_map_handler = Option<
        unsafe extern "C" fn(
            _: *mut librdf_stream,
            _: *mut libc::c_void,
            _: *mut librdf_statement,
        ) -> *mut librdf_statement,
    >;
    use super::libc;
    use super::librdf_h::{librdf_statement, librdf_stream, librdf_world};
    extern "C" {
        /* constructor */
        #[no_mangle]
        pub fn librdf_new_stream(
            world: *mut librdf_world,
            context: *mut libc::c_void,
            is_end_method: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
            next_method: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
            get_method: Option<
                unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> *mut libc::c_void,
            >,
            finished_method: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> *mut librdf_stream;
        #[no_mangle]
        pub fn librdf_new_empty_stream(world: *mut librdf_world) -> *mut librdf_stream;
        #[no_mangle]
        pub fn librdf_stream_add_map(
            stream: *mut librdf_stream,
            map_function: librdf_stream_map_handler,
            free_context: librdf_stream_map_free_context_handler,
            map_context: *mut libc::c_void,
        ) -> libc::c_int;
        /* destructor */
        #[no_mangle]
        pub fn librdf_free_stream(stream: *mut librdf_stream);
        /* methods */
        #[no_mangle]
        pub fn librdf_stream_end(stream: *mut librdf_stream) -> libc::c_int;
        #[no_mangle]
        pub fn librdf_stream_next(stream: *mut librdf_stream) -> libc::c_int;
        #[no_mangle]
        pub fn librdf_stream_get_object(stream: *mut librdf_stream) -> *mut librdf_statement;
    }
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    use super::__FILE_h::_IO_FILE;
    use super::libc;
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
        #[no_mangle]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, ...) -> libc::c_int;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::libc;
    extern "C" {
        #[no_mangle]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::libc;
    extern "C" {
        #[no_mangle]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[header_src = "/home/vagrant/C2Rust/redland-1.0.17/src/rdf_heuristics.h"]
pub mod rdf_heuristics_h {
    use super::libc;
    extern "C" {
        /* -*- Mode: c; c-basic-offset: 2 -*-
         *
         * rdf_heuristics.h - Heuristic routines to guess things about RDF prototypes
         *
         * Copyright (C) 2000-2008, David Beckett http://www.dajobe.org/
         * Copyright (C) 2000-2004, University of Bristol, UK http://www.bristol.ac.uk/
         *
         * This package is Free Software and part of Redland http://librdf.org/
         *
         * It is licensed under the following three licenses as alternatives:
         *   1. GNU Lesser General Public License (LGPL) V2.1 or any newer version
         *   2. GNU General Public License (GPL) V2 or any newer version
         *   3. Apache License, V2.0 or any newer version
         *
         * You may not use this file except in compliance with at least one of
         * the above three licenses.
         *
         * See LICENSE.html or LICENSE.txt at the top of this package for the
         * complete terms and further detail along with the license texts for
         * the licenses in COPYING.LIB, COPYING and LICENSE-2.0.txt respectively.
         *
         *
         */
        #[no_mangle]
        pub fn librdf_heuristic_gen_name(name: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[header_src = "/home/vagrant/C2Rust/redland-1.0.17/src/rdf_hash.h"]
pub mod rdf_hash_h {
    use super::libc;
    use super::librdf_h::{librdf_hash, librdf_world};
    extern "C" {
        /* -*- Mode: c; c-basic-offset: 2 -*-
         *
         * rdf_hash.h - RDF Hash Factory and Hash interfaces and definitions
         *
         * Copyright (C) 2000-2008, David Beckett http://www.dajobe.org/
         * Copyright (C) 2000-2005, University of Bristol, UK http://www.bristol.ac.uk/
         *
         * This package is Free Software and part of Redland http://librdf.org/
         *
         * It is licensed under the following three licenses as alternatives:
         *   1. GNU Lesser General Public License (LGPL) V2.1 or any newer version
         *   2. GNU General Public License (GPL) V2 or any newer version
         *   3. Apache License, V2.0 or any newer version
         *
         * You may not use this file except in compliance with at least one of
         * the above three licenses.
         *
         * See LICENSE.html or LICENSE.txt at the top of this package for the
         * complete terms and further detail along with the license texts for
         * the licenses in COPYING.LIB, COPYING and LICENSE-2.0.txt respectively.
         *
         *
         */
        /* public constructors */
        #[no_mangle]
        pub fn librdf_new_hash(
            world: *mut librdf_world,
            name: *const libc::c_char,
        ) -> *mut librdf_hash;
        /* public copy constructor */
        #[no_mangle]
        pub fn librdf_new_hash_from_hash(old_hash: *mut librdf_hash) -> *mut librdf_hash;
        /* public destructor */
        #[no_mangle]
        pub fn librdf_free_hash(hash: *mut librdf_hash);
        /* lookup a hash key and decode value as a boolean */
        #[no_mangle]
        pub fn librdf_hash_get_as_boolean(
            hash: *mut librdf_hash,
            key: *const libc::c_char,
        ) -> libc::c_int;
        /* lookup a hash key and decode value as a long */
        #[no_mangle]
        pub fn librdf_hash_get_as_long(
            hash: *mut librdf_hash,
            key: *const libc::c_char,
        ) -> libc::c_long;
        /* retrieve one value for key and delete from hash all other values */
        #[no_mangle]
        pub fn librdf_hash_get_del(
            hash: *mut librdf_hash,
            key: *const libc::c_char,
        ) -> *mut libc::c_char;
    }
}
#[header_src = "/home/vagrant/C2Rust/redland-1.0.17/src/rdf_uri.h"]
pub mod rdf_uri_h {
    use super::libc;
    use super::librdf_h::librdf_uri;
    extern "C" {
        /* methods */
        #[no_mangle]
        pub fn librdf_uri_as_string(uri: *mut librdf_uri) -> *mut libc::c_uchar;
    }
}
#[header_src = "/home/vagrant/C2Rust/redland-1.0.17/src/rdf_node.h"]
pub mod rdf_node_h {
    use super::libc;
    use super::librdf_h::{librdf_node, librdf_uri, librdf_world};
    use super::stddef_h::size_t;
    extern "C" {
        /* Create a new Node from a typed literal string / language. */
        #[no_mangle]
        pub fn librdf_new_node_from_typed_literal(
            world: *mut librdf_world,
            value: *const libc::c_uchar,
            xml_language: *const libc::c_char,
            datatype_uri: *mut librdf_uri,
        ) -> *mut librdf_node;
        /* Create a new Node from an existing Node - CLONE */
        #[no_mangle]
        pub fn librdf_new_node_from_node(node: *mut librdf_node) -> *mut librdf_node;
        /* destructor */
        #[no_mangle]
        pub fn librdf_free_node(node: *mut librdf_node);
        /* serialise / deserialise */
        #[no_mangle]
        pub fn librdf_node_encode(
            node: *mut librdf_node,
            buffer: *mut libc::c_uchar,
            length: size_t,
        ) -> size_t;
        #[no_mangle]
        pub fn librdf_node_decode(
            world: *mut librdf_world,
            size_p: *mut size_t,
            buffer: *mut libc::c_uchar,
            length: size_t,
        ) -> *mut librdf_node;
    }
}
#[header_src = "/home/vagrant/C2Rust/redland-1.0.17/src/rdf_storage_internal.h"]
pub mod rdf_storage_internal_h {
    use super::librdf_h::librdf_world;
}
#[header_src = "/home/vagrant/C2Rust/redland-1.0.17/src/rdf_storage.h"]
pub mod rdf_storage_h {
    use super::libc;
    use super::librdf_h::{librdf_storage, librdf_storage_factory, librdf_world};
    use super::rdf_storage_module_h::librdf_storage_instance;
    extern "C" {
        #[no_mangle]
        pub fn librdf_storage_remove_reference(storage: *mut librdf_storage);
        /* methods */
        #[no_mangle]
        pub fn librdf_storage_add_reference(storage: *mut librdf_storage);
        #[no_mangle]
        pub fn librdf_storage_set_instance(
            storage: *mut librdf_storage,
            instance: librdf_storage_instance,
        );
        /* -*- Mode: c; c-basic-offset: 2 -*-
         *
         * rdf_storage.h - RDF Storage Factory and Storage interfaces and definitions
         *
         * Copyright (C) 2000-2008, David Beckett http://www.dajobe.org/
         * Copyright (C) 2000-2005, University of Bristol, UK http://www.bristol.ac.uk/
         *
         * This package is Free Software and part of Redland http://librdf.org/
         *
         * It is licensed under the following three licenses as alternatives:
         *   1. GNU Lesser General Public License (LGPL) V2.1 or any newer version
         *   2. GNU General Public License (GPL) V2 or any newer version
         *   3. Apache License, V2.0 or any newer version
         *
         * You may not use this file except in compliance with at least one of
         * the above three licenses.
         *
         * See LICENSE.html or LICENSE.txt at the top of this package for the
         * complete terms and further detail along with the license texts for
         * the licenses in COPYING.LIB, COPYING and LICENSE-2.0.txt respectively.
         *
         *
         */
        /* class methods */
        #[no_mangle]
        pub fn librdf_storage_register_factory(
            world: *mut librdf_world,
            name: *const libc::c_char,
            label: *const libc::c_char,
            factory: Option<unsafe extern "C" fn(_: *mut librdf_storage_factory) -> ()>,
        ) -> libc::c_int;
    }
}
#[header_src = "/home/vagrant/C2Rust/redland-1.0.17/src/rdf_stream_internal.h"]
pub mod rdf_stream_internal_h {
    use super::libc;
    use super::librdf_h::{librdf_statement, librdf_stream};
    extern "C" {
        #[no_mangle]
        pub fn librdf_stream_statement_find_map(
            stream: *mut librdf_stream,
            context: *mut libc::c_void,
            statement: *mut librdf_statement,
        ) -> *mut librdf_statement;
    }
}
use self::__FILE_h::_IO_FILE;
use self::libio_h::{_IO_lock_t, _IO_marker};
use self::librdf_h::{
    librdf_digest_factory, librdf_digest_factory_s, librdf_hash, librdf_hash_s, librdf_iterator,
    librdf_iterator_s, librdf_list, librdf_list_s, librdf_model, librdf_model_factory_s,
    librdf_node, librdf_query, librdf_query_factory, librdf_query_factory_s, librdf_query_results,
    librdf_query_results_formatter, librdf_query_results_formatter_s, librdf_query_results_s,
    librdf_query_s, librdf_statement, librdf_storage, librdf_storage_factory,
    librdf_storage_factory_s, librdf_storage_s, librdf_stream, librdf_stream_s, librdf_uri,
    librdf_world,
};
// use self::raptor2_h::{
//     raptor_iostream, raptor_iostream_s, raptor_locator, raptor_sequence, raptor_sequence_s,
//     raptor_statement, raptor_term, raptor_term_blank_value, raptor_term_literal_value,
//     raptor_term_type, raptor_term_value, raptor_uri, raptor_uri_s, raptor_world, raptor_world_s,
//     RAPTOR_TERM_TYPE_BLANK, RAPTOR_TERM_TYPE_LITERAL, RAPTOR_TERM_TYPE_UNKNOWN,
//     RAPTOR_TERM_TYPE_URI,
// };
// use self::rasqal_h::{
//     rasqal_query_results_formatter, rasqal_query_results_formatter_s, rasqal_world, rasqal_world_s,
// };
use self::rdf_hash_h::{
    librdf_free_hash, librdf_hash_get_as_boolean, librdf_hash_get_as_long, librdf_hash_get_del,
    librdf_new_hash, librdf_new_hash_from_hash,
};
use self::rdf_hash_internal_h::{
    librdf_free_hash_datum, librdf_hash_close, librdf_hash_datum, librdf_hash_datum_s,
    librdf_hash_delete, librdf_hash_exists, librdf_hash_factory, librdf_hash_factory_s,
    librdf_hash_get_all, librdf_hash_keys, librdf_hash_open, librdf_hash_put, librdf_hash_sync,
    librdf_hash_values_count, librdf_new_hash_datum,
};
use self::rdf_heuristics_h::librdf_heuristic_gen_name;
use self::rdf_init_h::{librdf_raptor_init_handler, librdf_rasqal_init_handler};
use self::rdf_iterator_h::{
    librdf_free_iterator, librdf_iterator_end, librdf_iterator_get_context,
    librdf_iterator_get_key, librdf_iterator_get_object, librdf_iterator_get_value,
    librdf_iterator_next, librdf_new_empty_iterator, librdf_new_iterator, unnamed,
    LIBRDF_ITERATOR_GET_METHOD_GET_CONTEXT, LIBRDF_ITERATOR_GET_METHOD_GET_KEY,
    LIBRDF_ITERATOR_GET_METHOD_GET_OBJECT, LIBRDF_ITERATOR_GET_METHOD_GET_VALUE,
};
use self::rdf_log_h::{
    librdf_log, librdf_log_facility, librdf_log_func, librdf_log_level, librdf_log_level_func,
    librdf_log_message, LIBRDF_FROM_CONCEPTS, LIBRDF_FROM_DIGEST, LIBRDF_FROM_FILES,
    LIBRDF_FROM_HASH, LIBRDF_FROM_INIT, LIBRDF_FROM_ITERATOR, LIBRDF_FROM_LAST, LIBRDF_FROM_LIST,
    LIBRDF_FROM_MEMORY, LIBRDF_FROM_MODEL, LIBRDF_FROM_NODE, LIBRDF_FROM_NONE, LIBRDF_FROM_PARSER,
    LIBRDF_FROM_QUERY, LIBRDF_FROM_RAPTOR, LIBRDF_FROM_SERIALIZER, LIBRDF_FROM_STATEMENT,
    LIBRDF_FROM_STORAGE, LIBRDF_FROM_STREAM, LIBRDF_FROM_URI, LIBRDF_FROM_UTF8, LIBRDF_LOG_DEBUG,
    LIBRDF_LOG_ERROR, LIBRDF_LOG_FATAL, LIBRDF_LOG_INFO, LIBRDF_LOG_LAST, LIBRDF_LOG_NONE,
    LIBRDF_LOG_WARN,
};
use self::rdf_node_h::{
    librdf_free_node, librdf_new_node_from_node, librdf_new_node_from_typed_literal,
    librdf_node_decode, librdf_node_encode,
};
use self::rdf_statement_h::{
    librdf_free_statement, librdf_new_statement_from_statement, librdf_statement_clear,
    librdf_statement_decode2, librdf_statement_encode2, librdf_statement_encode_parts2,
    librdf_statement_get_object, librdf_statement_get_predicate, librdf_statement_get_subject,
    librdf_statement_init, librdf_statement_part, librdf_statement_set_object,
    librdf_statement_set_predicate, librdf_statement_set_subject, LIBRDF_STATEMENT_ALL,
    LIBRDF_STATEMENT_OBJECT, LIBRDF_STATEMENT_PREDICATE, LIBRDF_STATEMENT_SUBJECT,
};
use self::rdf_storage_h::{
    librdf_storage_add_reference, librdf_storage_register_factory, librdf_storage_remove_reference,
    librdf_storage_set_instance,
};
use self::rdf_storage_module_h::librdf_storage_instance;
use self::rdf_stream_h::{
    librdf_free_stream, librdf_new_empty_stream, librdf_new_stream, librdf_stream_add_map,
    librdf_stream_end, librdf_stream_get_object, librdf_stream_map_free_context_handler,
    librdf_stream_map_handler, librdf_stream_next,
};
use self::rdf_stream_internal_h::librdf_stream_statement_find_map;
use self::rdf_uri_h::librdf_uri_as_string;
use self::stddef_h::size_t;
use self::stdio_h::{fprintf, sprintf};
use self::stdlib_h::{calloc, free, malloc};
use self::string_h::{memcpy, strcmp, strcpy, strdup, strlen};
use self::types_h::{__off64_t, __off_t};
use self::vararg::__va_list_tag;
use self::FILE_h::FILE;

#[derive(Clone)]
#[repr(C)]
pub struct librdf_storage_mdata_instance {
    pub mdatas: Vec<MutableData>,
    pub owner: PublicSignKey,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct librdf_storage_mdata_serialise_stream_context {
    pub storage: *mut librdf_storage,
    pub iterator: *mut librdf_iterator,
}

#[derive(Clone)]
#[repr(C)]
pub struct librdf_mdatas_iterator_context {
    pub iterator: *mut librdf_iterator,
    pub current: usize, // current index
    pub mdatas_ptrs: Vec<*mut MutableData>,
}

// TODO: Pass owner in from tests/examples.
fn gen_mdata(owner: PublicSignKey) -> MutableData {
    let mut owners = BTreeSet::new();
    let _ = owners.insert(owner);

    unwrap!(MutableData::new(
        rand::random(),
        0,
        BTreeMap::new(),
        BTreeMap::new(),
        owners,
    ))
}

// Copy a statement to an MData with subject, predicate, object keys.
unsafe fn copy_statement_to_mdata(
    mut statement: *mut librdf_statement,
    owner: PublicSignKey,
) -> MutableData {
    unsafe fn insert_key_from_node(
        node: *mut librdf_node,
        actions: EntryActions,
        key: Vec<u8>,
    ) -> EntryActions {
        if 0 != librdf_node_is_blank(node) {
            // TODO: Blank node, how to handle this?
            return actions;
        }

        let content = if 0 != librdf_node_is_resource(node) {
            // Node is a URI.
            let uri = librdf_node_get_uri(node);
            librdf_uri_as_string(uri)
        } else if 0 != librdf_node_is_literal(node) {
            // Node is a literal value.
            librdf_node_get_literal_value(node)
        } else {
            panic!("Unexpected: librdf_node has no type!");
        };

        let content = unwrap!(CStr::from_ptr(content as *const c_char).to_str());

        println!("... Inserting '{}'", content);

        actions.ins(key, content.as_bytes().to_vec(), 1)
    }

    println!("Copying statement to mdata");

    let mut actions = EntryActions::new();

    let node = librdf_statement_get_subject(statement);
    if !node.is_null() {
        actions = insert_key_from_node(node, actions, b"S".to_vec());
    }

    let node = librdf_statement_get_predicate(statement);
    if !node.is_null() {
        actions = insert_key_from_node(node, actions, b"P".to_vec());
    }

    let node = librdf_statement_get_object(statement);
    if !node.is_null() {
        actions = insert_key_from_node(node, actions, b"O".to_vec());
    }

    let mut mdata = gen_mdata(owner);
    unwrap!(mdata.mutate_entries(actions.into(), owner));

    mdata
}

// Copy an MData and its subject, predicate, object keys to a statement.
// NOTE: The returned statement pointer must be freed.
unsafe fn copy_mdata_to_statement(
    mdata: &MutableData,
    world: *mut librdf_world,
) -> *mut librdf_statement {
    let mut statement: *mut librdf_statement = librdf_new_statement(world);

    if let Some(entry) = mdata.get(b"S") {
        let content = unwrap!(CString::new(entry.content.clone())).into_bytes_with_nul();
        let node = librdf_new_node_from_literal(world, content.as_ptr(), ptr::null(), 0);

        librdf_statement_set_subject(statement, node);
    }

    if let Some(entry) = mdata.get(b"P") {
        let content = unwrap!(CString::new(entry.content.clone())).into_bytes_with_nul();
        let node = librdf_new_node_from_literal(world, content.as_ptr(), ptr::null(), 0);

        librdf_statement_set_predicate(statement, node);
    }

    if let Some(entry) = mdata.get(b"O") {
        let content = unwrap!(CString::new(entry.content.clone())).into_bytes_with_nul();
        let node = librdf_new_node_from_literal(world, content.as_ptr(), ptr::null(), 0);

        librdf_statement_set_object(statement, node);
    }

    return statement;
}

unsafe fn mdatas_get_iterator(
    mut context: *mut librdf_storage_mdata_instance,
    storage: *mut librdf_storage,
) -> *mut librdf_iterator {
    let mut iterator_context: *mut librdf_mdatas_iterator_context =
        0 as *mut librdf_mdatas_iterator_context;
    let mut iterator: *mut librdf_iterator = 0 as *mut librdf_iterator;

    iterator_context = calloc(
        1 as libc::c_ulong,
        ::std::mem::size_of::<librdf_mdatas_iterator_context>() as libc::c_ulong,
    ) as *mut librdf_mdatas_iterator_context;
    if iterator_context.is_null() {
        return 0 as *mut librdf_iterator;
    }

    (*iterator_context).current = 0;
    (*iterator_context).mdatas_ptrs = (*context)
        .mdatas
        .iter_mut()
        .map(|mdata| mdata as *mut MutableData)
        .collect();
    println!(
        "Creating iterator: len = {}",
        (*iterator_context).mdatas_ptrs.len()
    );

    iterator = librdf_new_iterator(
        (*storage).world,
        iterator_context as *mut libc::c_void,
        Some(librdf_mdatas_iterator_is_end),
        Some(librdf_mdatas_iterator_next_method),
        Some(librdf_mdatas_iterator_get_method),
        Some(librdf_mdatas_iterator_finished),
    );

    if iterator.is_null() {
        librdf_mdatas_iterator_finished(iterator_context as *mut libc::c_void);
    } else {
        (*iterator_context).iterator = iterator
    }

    return iterator;
}

unsafe extern "C" fn librdf_mdatas_iterator_is_end(mut iterator: *mut libc::c_void) -> libc::c_int {
    println!("Mdata iterator is end...");

    let mut context: *mut librdf_mdatas_iterator_context =
        iterator as *mut librdf_mdatas_iterator_context;

    let index = (*context).current;
    let len = (*context).mdatas_ptrs.len();

    return (index >= len) as libc::c_int;
}

unsafe extern "C" fn librdf_mdatas_iterator_next_method(
    mut iterator: *mut libc::c_void,
) -> libc::c_int {
    println!("Mdata iterator next...");

    let mut context: *mut librdf_mdatas_iterator_context =
        iterator as *mut librdf_mdatas_iterator_context;

    let len = (*context).mdatas_ptrs.len();
    if (*context).current >= len {
        return 1;
    }

    (*context).current += 1;

    return ((*context).current >= len) as libc::c_int;
}

unsafe extern "C" fn librdf_mdatas_iterator_get_method(
    mut iterator: *mut libc::c_void,
    mut flags: libc::c_int,
) -> *mut libc::c_void {
    println!("Mdata iterator get...");

    let mut context: *mut librdf_mdatas_iterator_context =
        iterator as *mut librdf_mdatas_iterator_context;

    let index = (*context).current;
    println!(
        "Getting index {}, len {}",
        index,
        (*context).mdatas_ptrs.len()
    );

    match flags as u32 {
        LIBRDF_ITERATOR_GET_METHOD_GET_OBJECT | LIBRDF_ITERATOR_GET_METHOD_GET_CONTEXT =>
        // Return a reference (casted to void) to the MutableData.
        {
            return (*context).mdatas_ptrs[index] as *mut libc::c_void;
        }
        _ => {
            println!("Unsupported iterator method flag {}", flags);

            return 0 as *mut libc::c_void;
        }
    }
}

unsafe extern "C" fn librdf_mdatas_iterator_finished(mut iterator: *mut libc::c_void) {
    println!("Mdata iterator finished...");

    let mut context: *mut librdf_mdatas_iterator_context =
        iterator as *mut librdf_mdatas_iterator_context;
    if context.is_null() {
        return;
    }

    free(context as *mut libc::c_void);
    return;
}

#[no_mangle]
pub unsafe extern "C" fn librdf_init_storage_mdata(mut world: *mut librdf_world) {
    librdf_storage_register_factory(
        world,
        b"mdata\x00" as *const u8 as *const libc::c_char,
        b"Mutable Data\x00" as *const u8 as *const libc::c_char,
        Some(librdf_storage_mdata_register_factory),
    );
}

unsafe extern "C" fn librdf_storage_mdata_register_factory(
    mut factory: *mut librdf_storage_factory,
) {
    if 0 != strcmp(
        (*factory).name,
        b"mdata\x00" as *const u8 as *const libc::c_char,
    ) {
        // fprintf(stderr
        //         b"%s:%d: (%s) assertion failed: assertion !strcmp(factory->name, \"mdata\") failed.\n\x00"
        //             as *const u8 as *const libc::c_char,
        //         b"storage.rs\x00" as *const u8 as
        //             *const libc::c_char, 1937i32,
        //         (*::std::mem::transmute::<&[u8; 39],
        //                                   &[libc::c_char; 39]>(b"librdf_storage_mdata_register_factory\x00")).as_ptr());
        eprintln!("Assertion failed.");
        return;
    }

    (*factory).version = 1;
    (*factory).init = Some(librdf_storage_mdata_init);
    (*factory).clone = None; //Some(librdf_storage_mdata_clone);
    (*factory).terminate = Some(librdf_storage_mdata_terminate);
    (*factory).open = Some(librdf_storage_mdata_open);
    (*factory).close = Some(librdf_storage_mdata_close);
    (*factory).size = Some(librdf_storage_mdata_size);
    (*factory).add_statement = Some(librdf_storage_mdata_add_statement);
    (*factory).add_statements = Some(librdf_storage_mdata_add_statements);
    (*factory).remove_statement = Some(librdf_storage_mdata_remove_statement);
    (*factory).contains_statement = Some(librdf_storage_mdata_contains_statement);
    (*factory).serialise = Some(librdf_storage_mdata_serialise);
    (*factory).find_statements = Some(librdf_storage_mdata_find_statements);
    (*factory).find_sources = None; //Some(librdf_storage_mdata_find_sources);
    (*factory).find_arcs = None; //Some(librdf_storage_mdata_find_arcs);
    (*factory).find_targets = None; //Some(librdf_storage_mdata_find_targets);
    (*factory).context_add_statement = None; //Some(librdf_storage_mdata_context_add_statement);
    (*factory).context_remove_statement = None; //Some(librdf_storage_mdata_context_remove_statement);
    (*factory).context_serialise = Some(librdf_storage_mdata_context_serialise);
    (*factory).sync = None; //Some(librdf_storage_mdata_sync);
    (*factory).get_contexts = None; //Some(librdf_storage_mdata_get_contexts);
    (*factory).get_feature = None; //Some(librdf_storage_mdata_get_feature);
}

unsafe extern "C" fn librdf_storage_mdata_serialise(
    mut storage: *mut librdf_storage,
) -> *mut librdf_stream {
    println!("Serialising mdata storage...");

    let mut context: *mut librdf_storage_mdata_instance =
        (*storage).instance as *mut librdf_storage_mdata_instance;
    let mut scontext: *mut librdf_storage_mdata_serialise_stream_context =
        0 as *mut librdf_storage_mdata_serialise_stream_context;
    let mut stream: *mut librdf_stream = 0 as *mut librdf_stream;

    scontext = calloc(
        1 as libc::c_ulong,
        ::std::mem::size_of::<librdf_storage_mdata_serialise_stream_context>() as libc::c_ulong,
    ) as *mut librdf_storage_mdata_serialise_stream_context;
    if scontext.is_null() {
        return 0 as *mut librdf_stream;
    }

    println!("1");

    (*scontext).iterator = mdatas_get_iterator(context, storage);
    if (*scontext).iterator.is_null() {
        free(scontext as *mut libc::c_void);
        return librdf_new_empty_stream((*storage).world);
    }

    (*scontext).storage = storage;
    librdf_storage_add_reference((*scontext).storage);

    println!("2");

    stream = librdf_new_stream(
        (*storage).world,
        scontext as *mut libc::c_void,
        Some(librdf_storage_mdata_serialise_end_of_stream),
        Some(librdf_storage_mdata_serialise_next_statement),
        Some(librdf_storage_mdata_serialise_get_statement),
        Some(librdf_storage_mdata_serialise_finished),
    );

    println!("3");

    if stream.is_null() {
        librdf_storage_mdata_serialise_finished(scontext as *mut libc::c_void);
        return 0 as *mut librdf_stream;
    }

    println!("4");

    return stream;
}

unsafe extern "C" fn librdf_storage_mdata_serialise_end_of_stream(
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut scontext: *mut librdf_storage_mdata_serialise_stream_context =
        context as *mut librdf_storage_mdata_serialise_stream_context;

    return librdf_iterator_end((*scontext).iterator);
}

unsafe extern "C" fn librdf_storage_mdata_serialise_next_statement(
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut scontext: *mut librdf_storage_mdata_serialise_stream_context =
        context as *mut librdf_storage_mdata_serialise_stream_context;

    return librdf_iterator_next((*scontext).iterator);
}

unsafe extern "C" fn librdf_storage_mdata_serialise_get_statement(
    mut context: *mut libc::c_void,
    mut flags: libc::c_int,
) -> *mut libc::c_void {
    let mut scontext: *mut librdf_storage_mdata_serialise_stream_context =
        context as *mut librdf_storage_mdata_serialise_stream_context;

    let mut mdata: *mut MutableData =
        librdf_iterator_get_object((*scontext).iterator) as *mut MutableData;

    match flags as u32 {
        // TODO: The statement produced below doesn't get freed. We need to rearchitect the iterator
        // so that it holds the statement at each iteration and frees it on the next.
        LIBRDF_ITERATOR_GET_METHOD_GET_OBJECT | LIBRDF_ITERATOR_GET_METHOD_GET_CONTEXT => {
            return copy_mdata_to_statement(&*mdata, (*(*scontext).storage).world)
                as *mut libc::c_void;
        }
        _ => {
            println!("Unknown iterator method flag {}", flags);
            return 0 as *mut libc::c_void;
        }
    };
}

unsafe extern "C" fn librdf_storage_mdata_serialise_finished(mut context: *mut libc::c_void) {
    let mut scontext: *mut librdf_storage_mdata_serialise_stream_context =
        context as *mut librdf_storage_mdata_serialise_stream_context;

    if !(*scontext).storage.is_null() {
        librdf_storage_remove_reference((*scontext).storage);
    }
    if !(*scontext).iterator.is_null() {
        librdf_free_iterator((*scontext).iterator);
    }

    free(scontext as *mut libc::c_void);
}

unsafe extern "C" fn librdf_storage_mdata_find_statements(
    mut storage: *mut librdf_storage,
    mut statement: *mut librdf_statement,
) -> *mut librdf_stream {
    println!("Finding mdata storage statements...");

    unimplemented!()
}

unsafe extern "C" fn librdf_storage_mdata_context_serialise(
    mut storage: *mut librdf_storage,
    mut context_node: *mut librdf_node,
) -> *mut librdf_stream {
    println!("Serialising mdata storage context...");

    unimplemented!()
}

unsafe extern "C" fn librdf_storage_mdata_contains_statement(
    mut storage: *mut librdf_storage,
    mut statement: *mut librdf_statement,
) -> libc::c_int {
    println!("Checking if mdata storage contains statement...");

    let mut context: *mut librdf_storage_mdata_instance =
        (*storage).instance as *mut librdf_storage_mdata_instance;

    let statement_mdata = copy_statement_to_mdata(statement, (*context).owner);

    return (*context).mdatas.contains(&statement_mdata) as libc::c_int;
}

unsafe extern "C" fn librdf_storage_mdata_add_statement(
    mut storage: *mut librdf_storage,
    mut statement: *mut librdf_statement,
) -> libc::c_int {
    println!("Adding statement...");

    let mut context: *mut librdf_storage_mdata_instance =
        (*storage).instance as *mut librdf_storage_mdata_instance;

    /* Do not add duplicate statements */
    if 0 != librdf_storage_mdata_contains_statement(storage, statement) {
        return 0;
    }

    let statement_mdata = copy_statement_to_mdata(statement, (*context).owner);

    (*context).mdatas.push(statement_mdata);

    return 0;
}

unsafe extern "C" fn librdf_storage_mdata_add_statements(
    mut storage: *mut librdf_storage,
    mut statement_stream: *mut librdf_stream,
) -> libc::c_int {
    println!("Adding statements...");

    let mut status: libc::c_int = 0;

    while 0 == librdf_stream_end(statement_stream) {
        let mut statement: *mut librdf_statement = librdf_stream_get_object(statement_stream);
        if statement.is_null() {
            status = 1;
            break;
        }

        librdf_storage_mdata_add_statement(storage, statement);

        librdf_stream_next(statement_stream);
    }

    return status;
}

unsafe extern "C" fn librdf_storage_mdata_remove_statement(
    mut storage: *mut librdf_storage,
    mut statement: *mut librdf_statement,
) -> libc::c_int {
    println!("Removing mdata storage statement...");

    unimplemented!()
}

unsafe extern "C" fn librdf_storage_mdata_terminate(mut storage: *mut librdf_storage) {
    println!("Terminating mdata storage...");

    if (*storage).instance.is_null() {
        return;
    } else {
        free((*storage).instance);
        return;
    };
}

#[allow(unused_variables)]
unsafe extern "C" fn librdf_storage_mdata_open(
    mut storage: *mut librdf_storage,
    mut model: *mut librdf_model,
) -> libc::c_int {
    println!("Opening mdata storage...");

    let mut context: *mut librdf_storage_mdata_instance =
        (*storage).instance as *mut librdf_storage_mdata_instance;

    (*context).mdatas = Vec::new();

    return 0;
}

unsafe extern "C" fn librdf_storage_mdata_close(mut storage: *mut librdf_storage) -> libc::c_int {
    println!("Closing mdata storage...");

    unimplemented!()
}

unsafe extern "C" fn librdf_storage_mdata_size(mut storage: *mut librdf_storage) -> libc::c_int {
    println!("Getting mdata storage size...");

    let mut context: *mut librdf_storage_mdata_instance =
        (*storage).instance as *mut librdf_storage_mdata_instance;
    return (*context).mdatas.len() as libc::c_int;
}

#[allow(unused_variables)]
unsafe extern "C" fn librdf_storage_mdata_init(
    mut storage: *mut librdf_storage,
    mut name: *const libc::c_char,
    mut options: *mut librdf_hash,
) -> libc::c_int {
    println!("Initializing mdata storage...");

    if options.is_null() {
        return 1;
    }

    println!("... Fetching the owner from the options string");
    let owner = librdf_hash_get_del(options, b"owner\x00" as *const u8 as *const libc::c_char);

    if owner.is_null() {
        println!("... No owner found in options string");
        return 1;
    }

    // We need to parse the owner key from its human-readable UTF8 form into bytes.
    let owner_formatted = unwrap!(ffi_utils::from_c_str(owner));
    let owner = parse_formatted_key(&owner_formatted);
    println!("... Owner public key: {:?}", owner.0);

    let mut context: *mut librdf_storage_mdata_instance = 0 as *mut librdf_storage_mdata_instance;

    context = calloc(
        1 as libc::c_ulong,
        ::std::mem::size_of::<librdf_storage_mdata_instance>() as libc::c_ulong,
    ) as *mut librdf_storage_mdata_instance;
    if context.is_null() {
        return 1;
    }

    (*context).owner = owner;
    librdf_storage_set_instance(storage, context as librdf_storage_instance);

    /* no more options, might as well free them now */
    if !options.is_null() {
        librdf_free_hash(options);
    }

    return 0;
}

fn parse_formatted_key(key: &str) -> PublicSignKey {
    let key_bytes: Vec<u8> = key
        .split(|del| "[], ".contains(del))
        .filter(|byte| !byte.is_empty())
        .map(|byte| unwrap!(u8::from_str(byte)))
        .collect();

    // println!("... Constructing key: {:?}", key_bytes);

    let mut key_array: [u8; 32] = Default::default();
    key_array.copy_from_slice(&key_bytes);

    PublicSignKey(key_array)
}

struct World(*mut librdf_world);

impl Drop for World {
    fn drop(&mut self) {
        unsafe {
            librdf_free_world(self.0);
        }
    }
}

struct Model(*mut librdf_model);

impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            librdf_free_model(self.0);
        }
    }
}

unsafe fn list_storages(world: &World) {
    let mut i = 0;
    loop {
        let mut name: *const c_char = mem::zeroed();
        let mut label: *const c_char = mem::zeroed();

        if librdf_storage_enumerate(world.0, i, &mut name, &mut label) != 0 {
            break;
        }

        let name = if !name.is_null() {
            unwrap!(ffi_utils::from_c_str(name))
        } else {
            "none".into()
        };
        let label = if !label.is_null() {
            unwrap!(ffi_utils::from_c_str(label))
        } else {
            "none".into()
        };

        println!("Name: {}\nLabel: {}", name, label);

        println!();
        i += 1;
    }
}

fn main() {
    unsafe {
        println!("Creating the world");

        let world = World(librdf_new_world());
        if world.0.is_null() {
            println!("ERROR: an error occurred in librdf_new_world");
            return;
        }

        println!("Initiating Mutable Data storage");

        let (owner, _) = sign::gen_keypair();
        println!("Owner public key: {:?}", owner.0);

        librdf_init_storage_mdata(world.0);

        // {
        //     println!("Listing available storage methods");
        //     println!();
        //     list_storages(&world);
        // }

        println!("Creating storage");

        // Options syntax is: key1='value1',key2='value2'.
        // We use the owner key's debug representation so it can be passed in as UTF-8.
        // We have to parse it back to its native representation in `librdf_storage_mdata_init`.
        // FIXME: Pass the owner key through the storage `name` field instead?
        // FIXME: Pass this option to `librdf_new_model`, parse in `librdf_storage_mdata_open`?
        let mut options = format!("owner='{:?}'", owner.0);
        let options = unwrap!(CString::new(options));

        // println!("Using options string \"{}\"", unwrap!(options.to_str()));

        let name = unwrap!(CString::new("mdata"));
        let storage = librdf_new_storage(world.0, name.as_ptr(), name.as_ptr(), options.as_ptr());
        if storage.is_null() {
            println!("ERROR: an error occurred in librdf_new_storage");
            return;
        }

        println!("Creating serializer");

        let turtle = unwrap!(CString::new("turtle"));
        let serializer =
            librdf_new_serializer(world.0, turtle.as_ptr(), ptr::null(), ptr::null_mut());
        let maidsafe = unwrap!(CString::new("http://maidsafe.net/")).into_bytes_with_nul();
        let ms_schema = librdf_new_uri(world.0, maidsafe.as_ptr());
        let ms = unwrap!(CString::new("ms"));
        librdf_serializer_set_namespace(serializer, ms_schema, ms.as_ptr());

        println!("Creating new nodes");

        let subject = librdf_new_node_from_uri_local_name(world.0, ms_schema, maidsafe.as_ptr());
        let location = unwrap!(CString::new("location")).into_bytes_with_nul();
        let predicate = librdf_new_node_from_uri_local_name(world.0, ms_schema, location.as_ptr());

        println!("Creating new model");

        let model = Model(librdf_new_model(world.0, storage, ptr::null()));
        let ayr = unwrap!(CString::new("Ayr")).into_bytes_with_nul();
        librdf_model_add_string_literal_statement(
            model.0,
            subject,
            predicate,
            ayr.as_ptr(),
            ptr::null(),
            0,
        );

        println!("Serializing model to string");

        let result =
            librdf_serializer_serialize_model_to_string(serializer, ptr::null_mut(), model.0);
        println!("5");
        println!(
            "\nRetrieved model:{}\n",
            CStr::from_ptr(result as *const c_char).to_str().unwrap()
        );

        println!("Freeing objects");

        librdf_free_memory(result as *mut _);

        librdf_free_storage(storage);
        librdf_free_serializer(serializer);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        copy_mdata_to_statement, copy_statement_to_mdata, gen_mdata, parse_formatted_key, World,
    };
    use redland_rs::*;
    use routing::{EntryActions, Value};
    use rust_sodium::crypto::sign;

    #[test]
    fn mdata_to_statement_conversion() {
        // Create the librdf world.

        let world = unsafe { World(librdf_new_world()) };

        // Create the mdata.

        let (owner, _) = sign::gen_keypair();
        let mut mdata = gen_mdata(owner);

        // Add test S, P, O.

        let mut actions = EntryActions::new();
        actions = actions.ins(b"S".to_vec(), "subject".into(), 1);
        actions = actions.ins(b"P".to_vec(), "predicate".into(), 1);
        actions = actions.ins(b"O".to_vec(), "object".into(), 1);

        let _ = unwrap!(mdata.mutate_entries(actions.into(), owner));

        unsafe {
            let statement = copy_mdata_to_statement(&mdata, world.0);
            let mdata2 = copy_statement_to_mdata(statement, owner);

            assert_eq!(unwrap!(mdata2.get(b"S")).content, b"subject".to_vec());
            assert_eq!(mdata.entries(), mdata2.entries());
        }
    }

    #[test]
    fn owner_key_conversion() {
        let (owner, _) = sign::gen_keypair();

        let owner_formatted = format!("{:?}", owner.0);

        let owner2 = parse_formatted_key(&owner_formatted);

        assert_eq!(owner, owner2);
    }
}

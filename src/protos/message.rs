// This file is generated by rust-protobuf 2.10.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `message.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_10_2;

#[derive(PartialEq,Clone,Default)]
pub struct RegisterPeer {
    // message fields
    pub hbb_addr: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RegisterPeer {
    fn default() -> &'a RegisterPeer {
        <RegisterPeer as ::protobuf::Message>::default_instance()
    }
}

impl RegisterPeer {
    pub fn new() -> RegisterPeer {
        ::std::default::Default::default()
    }

    // string hbb_addr = 1;


    pub fn get_hbb_addr(&self) -> &str {
        &self.hbb_addr
    }
    pub fn clear_hbb_addr(&mut self) {
        self.hbb_addr.clear();
    }

    // Param is passed by value, moved
    pub fn set_hbb_addr(&mut self, v: ::std::string::String) {
        self.hbb_addr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hbb_addr(&mut self) -> &mut ::std::string::String {
        &mut self.hbb_addr
    }

    // Take field
    pub fn take_hbb_addr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.hbb_addr, ::std::string::String::new())
    }
}

impl ::protobuf::Message for RegisterPeer {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.hbb_addr)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.hbb_addr.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.hbb_addr);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.hbb_addr.is_empty() {
            os.write_string(1, &self.hbb_addr)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> RegisterPeer {
        RegisterPeer::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hbb_addr",
                    |m: &RegisterPeer| { &m.hbb_addr },
                    |m: &mut RegisterPeer| { &mut m.hbb_addr },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegisterPeer>(
                    "RegisterPeer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static RegisterPeer {
        static mut instance: ::protobuf::lazy::Lazy<RegisterPeer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegisterPeer,
        };
        unsafe {
            instance.get(RegisterPeer::new)
        }
    }
}

impl ::protobuf::Clear for RegisterPeer {
    fn clear(&mut self) {
        self.hbb_addr.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegisterPeer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegisterPeer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PeekPeer {
    // message fields
    pub hbb_addr: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PeekPeer {
    fn default() -> &'a PeekPeer {
        <PeekPeer as ::protobuf::Message>::default_instance()
    }
}

impl PeekPeer {
    pub fn new() -> PeekPeer {
        ::std::default::Default::default()
    }

    // string hbb_addr = 1;


    pub fn get_hbb_addr(&self) -> &str {
        &self.hbb_addr
    }
    pub fn clear_hbb_addr(&mut self) {
        self.hbb_addr.clear();
    }

    // Param is passed by value, moved
    pub fn set_hbb_addr(&mut self, v: ::std::string::String) {
        self.hbb_addr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hbb_addr(&mut self) -> &mut ::std::string::String {
        &mut self.hbb_addr
    }

    // Take field
    pub fn take_hbb_addr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.hbb_addr, ::std::string::String::new())
    }
}

impl ::protobuf::Message for PeekPeer {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.hbb_addr)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.hbb_addr.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.hbb_addr);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.hbb_addr.is_empty() {
            os.write_string(1, &self.hbb_addr)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> PeekPeer {
        PeekPeer::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hbb_addr",
                    |m: &PeekPeer| { &m.hbb_addr },
                    |m: &mut PeekPeer| { &mut m.hbb_addr },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PeekPeer>(
                    "PeekPeer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static PeekPeer {
        static mut instance: ::protobuf::lazy::Lazy<PeekPeer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PeekPeer,
        };
        unsafe {
            instance.get(PeekPeer::new)
        }
    }
}

impl ::protobuf::Clear for PeekPeer {
    fn clear(&mut self) {
        self.hbb_addr.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PeekPeer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PeekPeer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PeekPeerResponse {
    // message fields
    pub socket_addr: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PeekPeerResponse {
    fn default() -> &'a PeekPeerResponse {
        <PeekPeerResponse as ::protobuf::Message>::default_instance()
    }
}

impl PeekPeerResponse {
    pub fn new() -> PeekPeerResponse {
        ::std::default::Default::default()
    }

    // bytes socket_addr = 1;


    pub fn get_socket_addr(&self) -> &[u8] {
        &self.socket_addr
    }
    pub fn clear_socket_addr(&mut self) {
        self.socket_addr.clear();
    }

    // Param is passed by value, moved
    pub fn set_socket_addr(&mut self, v: ::std::vec::Vec<u8>) {
        self.socket_addr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_socket_addr(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.socket_addr
    }

    // Take field
    pub fn take_socket_addr(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.socket_addr, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for PeekPeerResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.socket_addr)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.socket_addr.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.socket_addr);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.socket_addr.is_empty() {
            os.write_bytes(1, &self.socket_addr)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> PeekPeerResponse {
        PeekPeerResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "socket_addr",
                    |m: &PeekPeerResponse| { &m.socket_addr },
                    |m: &mut PeekPeerResponse| { &mut m.socket_addr },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PeekPeerResponse>(
                    "PeekPeerResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static PeekPeerResponse {
        static mut instance: ::protobuf::lazy::Lazy<PeekPeerResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PeekPeerResponse,
        };
        unsafe {
            instance.get(PeekPeerResponse::new)
        }
    }
}

impl ::protobuf::Clear for PeekPeerResponse {
    fn clear(&mut self) {
        self.socket_addr.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PeekPeerResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PeekPeerResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Message {
    // message oneof groups
    pub union: ::std::option::Option<Message_oneof_union>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Message {
    fn default() -> &'a Message {
        <Message as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum Message_oneof_union {
    register_peer(RegisterPeer),
    peek_peer(PeekPeer),
    peek_peer_response(PeekPeerResponse),
}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    // .hbb.RegisterPeer register_peer = 6;


    pub fn get_register_peer(&self) -> &RegisterPeer {
        match self.union {
            ::std::option::Option::Some(Message_oneof_union::register_peer(ref v)) => v,
            _ => RegisterPeer::default_instance(),
        }
    }
    pub fn clear_register_peer(&mut self) {
        self.union = ::std::option::Option::None;
    }

    pub fn has_register_peer(&self) -> bool {
        match self.union {
            ::std::option::Option::Some(Message_oneof_union::register_peer(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_register_peer(&mut self, v: RegisterPeer) {
        self.union = ::std::option::Option::Some(Message_oneof_union::register_peer(v))
    }

    // Mutable pointer to the field.
    pub fn mut_register_peer(&mut self) -> &mut RegisterPeer {
        if let ::std::option::Option::Some(Message_oneof_union::register_peer(_)) = self.union {
        } else {
            self.union = ::std::option::Option::Some(Message_oneof_union::register_peer(RegisterPeer::new()));
        }
        match self.union {
            ::std::option::Option::Some(Message_oneof_union::register_peer(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_register_peer(&mut self) -> RegisterPeer {
        if self.has_register_peer() {
            match self.union.take() {
                ::std::option::Option::Some(Message_oneof_union::register_peer(v)) => v,
                _ => panic!(),
            }
        } else {
            RegisterPeer::new()
        }
    }

    // .hbb.PeekPeer peek_peer = 7;


    pub fn get_peek_peer(&self) -> &PeekPeer {
        match self.union {
            ::std::option::Option::Some(Message_oneof_union::peek_peer(ref v)) => v,
            _ => PeekPeer::default_instance(),
        }
    }
    pub fn clear_peek_peer(&mut self) {
        self.union = ::std::option::Option::None;
    }

    pub fn has_peek_peer(&self) -> bool {
        match self.union {
            ::std::option::Option::Some(Message_oneof_union::peek_peer(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_peek_peer(&mut self, v: PeekPeer) {
        self.union = ::std::option::Option::Some(Message_oneof_union::peek_peer(v))
    }

    // Mutable pointer to the field.
    pub fn mut_peek_peer(&mut self) -> &mut PeekPeer {
        if let ::std::option::Option::Some(Message_oneof_union::peek_peer(_)) = self.union {
        } else {
            self.union = ::std::option::Option::Some(Message_oneof_union::peek_peer(PeekPeer::new()));
        }
        match self.union {
            ::std::option::Option::Some(Message_oneof_union::peek_peer(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_peek_peer(&mut self) -> PeekPeer {
        if self.has_peek_peer() {
            match self.union.take() {
                ::std::option::Option::Some(Message_oneof_union::peek_peer(v)) => v,
                _ => panic!(),
            }
        } else {
            PeekPeer::new()
        }
    }

    // .hbb.PeekPeerResponse peek_peer_response = 8;


    pub fn get_peek_peer_response(&self) -> &PeekPeerResponse {
        match self.union {
            ::std::option::Option::Some(Message_oneof_union::peek_peer_response(ref v)) => v,
            _ => PeekPeerResponse::default_instance(),
        }
    }
    pub fn clear_peek_peer_response(&mut self) {
        self.union = ::std::option::Option::None;
    }

    pub fn has_peek_peer_response(&self) -> bool {
        match self.union {
            ::std::option::Option::Some(Message_oneof_union::peek_peer_response(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_peek_peer_response(&mut self, v: PeekPeerResponse) {
        self.union = ::std::option::Option::Some(Message_oneof_union::peek_peer_response(v))
    }

    // Mutable pointer to the field.
    pub fn mut_peek_peer_response(&mut self) -> &mut PeekPeerResponse {
        if let ::std::option::Option::Some(Message_oneof_union::peek_peer_response(_)) = self.union {
        } else {
            self.union = ::std::option::Option::Some(Message_oneof_union::peek_peer_response(PeekPeerResponse::new()));
        }
        match self.union {
            ::std::option::Option::Some(Message_oneof_union::peek_peer_response(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_peek_peer_response(&mut self) -> PeekPeerResponse {
        if self.has_peek_peer_response() {
            match self.union.take() {
                ::std::option::Option::Some(Message_oneof_union::peek_peer_response(v)) => v,
                _ => panic!(),
            }
        } else {
            PeekPeerResponse::new()
        }
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        if let Some(Message_oneof_union::register_peer(ref v)) = self.union {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_union::peek_peer(ref v)) = self.union {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_union::peek_peer_response(ref v)) = self.union {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.union = ::std::option::Option::Some(Message_oneof_union::register_peer(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.union = ::std::option::Option::Some(Message_oneof_union::peek_peer(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.union = ::std::option::Option::Some(Message_oneof_union::peek_peer_response(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.union {
            match v {
                &Message_oneof_union::register_peer(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_union::peek_peer(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_union::peek_peer_response(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.union {
            match v {
                &Message_oneof_union::register_peer(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_union::peek_peer(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_union::peek_peer_response(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RegisterPeer>(
                    "register_peer",
                    Message::has_register_peer,
                    Message::get_register_peer,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, PeekPeer>(
                    "peek_peer",
                    Message::has_peek_peer,
                    Message::get_peek_peer,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, PeekPeerResponse>(
                    "peek_peer_response",
                    Message::has_peek_peer_response,
                    Message::get_peek_peer_response,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message>(
                    "Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Message {
        static mut instance: ::protobuf::lazy::Lazy<Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message,
        };
        unsafe {
            instance.get(Message::new)
        }
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.union = ::std::option::Option::None;
        self.union = ::std::option::Option::None;
        self.union = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rmessage.proto\x12\x03hbb\"$\n\x0cRegisterPeer\x12\x12\n\x08hbb_addr\
    \x18\x01\x20\x01(\tB\0:\0\"\x20\n\x08PeekPeer\x12\x12\n\x08hbb_addr\x18\
    \x01\x20\x01(\tB\0:\0\"+\n\x10PeekPeerResponse\x12\x15\n\x0bsocket_addr\
    \x18\x01\x20\x01(\x0cB\0:\0\"\x9f\x01\n\x07Message\x12,\n\rregister_peer\
    \x18\x06\x20\x01(\x0b2\x11.hbb.RegisterPeerH\0B\0\x12$\n\tpeek_peer\x18\
    \x07\x20\x01(\x0b2\r.hbb.PeekPeerH\0B\0\x125\n\x12peek_peer_response\x18\
    \x08\x20\x01(\x0b2\x15.hbb.PeekPeerResponseH\0B\0B\x07\n\x05union:\0B\0b\
    \x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
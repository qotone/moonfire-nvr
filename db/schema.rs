// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct DirMeta {
    // message fields
    pub db_uuid: ::std::vec::Vec<u8>,
    pub dir_uuid: ::std::vec::Vec<u8>,
    pub last_complete_open: ::protobuf::SingularPtrField<DirMeta_Open>,
    pub in_progress_open: ::protobuf::SingularPtrField<DirMeta_Open>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DirMeta {}

impl DirMeta {
    pub fn new() -> DirMeta {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DirMeta {
        static mut instance: ::protobuf::lazy::Lazy<DirMeta> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DirMeta,
        };
        unsafe {
            instance.get(DirMeta::new)
        }
    }

    // bytes db_uuid = 1;

    pub fn clear_db_uuid(&mut self) {
        self.db_uuid.clear();
    }

    // Param is passed by value, moved
    pub fn set_db_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.db_uuid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_db_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.db_uuid
    }

    // Take field
    pub fn take_db_uuid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.db_uuid, ::std::vec::Vec::new())
    }

    pub fn get_db_uuid(&self) -> &[u8] {
        &self.db_uuid
    }

    fn get_db_uuid_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.db_uuid
    }

    fn mut_db_uuid_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.db_uuid
    }

    // bytes dir_uuid = 2;

    pub fn clear_dir_uuid(&mut self) {
        self.dir_uuid.clear();
    }

    // Param is passed by value, moved
    pub fn set_dir_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.dir_uuid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dir_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.dir_uuid
    }

    // Take field
    pub fn take_dir_uuid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.dir_uuid, ::std::vec::Vec::new())
    }

    pub fn get_dir_uuid(&self) -> &[u8] {
        &self.dir_uuid
    }

    fn get_dir_uuid_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.dir_uuid
    }

    fn mut_dir_uuid_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.dir_uuid
    }

    // .DirMeta.Open last_complete_open = 3;

    pub fn clear_last_complete_open(&mut self) {
        self.last_complete_open.clear();
    }

    pub fn has_last_complete_open(&self) -> bool {
        self.last_complete_open.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_complete_open(&mut self, v: DirMeta_Open) {
        self.last_complete_open = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_complete_open(&mut self) -> &mut DirMeta_Open {
        if self.last_complete_open.is_none() {
            self.last_complete_open.set_default();
        }
        self.last_complete_open.as_mut().unwrap()
    }

    // Take field
    pub fn take_last_complete_open(&mut self) -> DirMeta_Open {
        self.last_complete_open.take().unwrap_or_else(|| DirMeta_Open::new())
    }

    pub fn get_last_complete_open(&self) -> &DirMeta_Open {
        self.last_complete_open.as_ref().unwrap_or_else(|| DirMeta_Open::default_instance())
    }

    fn get_last_complete_open_for_reflect(&self) -> &::protobuf::SingularPtrField<DirMeta_Open> {
        &self.last_complete_open
    }

    fn mut_last_complete_open_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DirMeta_Open> {
        &mut self.last_complete_open
    }

    // .DirMeta.Open in_progress_open = 4;

    pub fn clear_in_progress_open(&mut self) {
        self.in_progress_open.clear();
    }

    pub fn has_in_progress_open(&self) -> bool {
        self.in_progress_open.is_some()
    }

    // Param is passed by value, moved
    pub fn set_in_progress_open(&mut self, v: DirMeta_Open) {
        self.in_progress_open = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_in_progress_open(&mut self) -> &mut DirMeta_Open {
        if self.in_progress_open.is_none() {
            self.in_progress_open.set_default();
        }
        self.in_progress_open.as_mut().unwrap()
    }

    // Take field
    pub fn take_in_progress_open(&mut self) -> DirMeta_Open {
        self.in_progress_open.take().unwrap_or_else(|| DirMeta_Open::new())
    }

    pub fn get_in_progress_open(&self) -> &DirMeta_Open {
        self.in_progress_open.as_ref().unwrap_or_else(|| DirMeta_Open::default_instance())
    }

    fn get_in_progress_open_for_reflect(&self) -> &::protobuf::SingularPtrField<DirMeta_Open> {
        &self.in_progress_open
    }

    fn mut_in_progress_open_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DirMeta_Open> {
        &mut self.in_progress_open
    }
}

impl ::protobuf::Message for DirMeta {
    fn is_initialized(&self) -> bool {
        for v in &self.last_complete_open {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.in_progress_open {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.db_uuid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.dir_uuid)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.last_complete_open)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.in_progress_open)?;
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
        if !self.db_uuid.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.db_uuid);
        }
        if !self.dir_uuid.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.dir_uuid);
        }
        if let Some(ref v) = self.last_complete_open.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.in_progress_open.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.db_uuid.is_empty() {
            os.write_bytes(1, &self.db_uuid)?;
        }
        if !self.dir_uuid.is_empty() {
            os.write_bytes(2, &self.dir_uuid)?;
        }
        if let Some(ref v) = self.last_complete_open.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.in_progress_open.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DirMeta {
    fn new() -> DirMeta {
        DirMeta::new()
    }

    fn descriptor_static(_: ::std::option::Option<DirMeta>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "db_uuid",
                    DirMeta::get_db_uuid_for_reflect,
                    DirMeta::mut_db_uuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "dir_uuid",
                    DirMeta::get_dir_uuid_for_reflect,
                    DirMeta::mut_dir_uuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DirMeta_Open>>(
                    "last_complete_open",
                    DirMeta::get_last_complete_open_for_reflect,
                    DirMeta::mut_last_complete_open_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DirMeta_Open>>(
                    "in_progress_open",
                    DirMeta::get_in_progress_open_for_reflect,
                    DirMeta::mut_in_progress_open_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DirMeta>(
                    "DirMeta",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DirMeta {
    fn clear(&mut self) {
        self.clear_db_uuid();
        self.clear_dir_uuid();
        self.clear_last_complete_open();
        self.clear_in_progress_open();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DirMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DirMeta {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DirMeta_Open {
    // message fields
    pub id: u32,
    pub uuid: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DirMeta_Open {}

impl DirMeta_Open {
    pub fn new() -> DirMeta_Open {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DirMeta_Open {
        static mut instance: ::protobuf::lazy::Lazy<DirMeta_Open> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DirMeta_Open,
        };
        unsafe {
            instance.get(DirMeta_Open::new)
        }
    }

    // uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }

    // bytes uuid = 2;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
    }

    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }

    fn get_uuid_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }
}

impl ::protobuf::Message for DirMeta_Open {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.uuid)?;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.uuid.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.uuid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_uint32(1, self.id)?;
        }
        if !self.uuid.is_empty() {
            os.write_bytes(2, &self.uuid)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DirMeta_Open {
    fn new() -> DirMeta_Open {
        DirMeta_Open::new()
    }

    fn descriptor_static(_: ::std::option::Option<DirMeta_Open>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    DirMeta_Open::get_id_for_reflect,
                    DirMeta_Open::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "uuid",
                    DirMeta_Open::get_uuid_for_reflect,
                    DirMeta_Open::mut_uuid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DirMeta_Open>(
                    "DirMeta_Open",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DirMeta_Open {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_uuid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DirMeta_Open {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DirMeta_Open {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cschema.proto\"\xdf\x01\n\x07DirMeta\x12\x17\n\x07db_uuid\x18\x01\
    \x20\x01(\x0cR\x06dbUuid\x12\x19\n\x08dir_uuid\x18\x02\x20\x01(\x0cR\x07\
    dirUuid\x12;\n\x12last_complete_open\x18\x03\x20\x01(\x0b2\r.DirMeta.Ope\
    nR\x10lastCompleteOpen\x127\n\x10in_progress_open\x18\x04\x20\x01(\x0b2\
    \r.DirMeta.OpenR\x0einProgressOpen\x1a*\n\x04Open\x12\x0e\n\x02id\x18\
    \x01\x20\x01(\rR\x02id\x12\x12\n\x04uuid\x18\x02\x20\x01(\x0cR\x04uuidJ\
    \xdf\x1b\n\x06\x12\x04\x1e\0H\x01\n\xc2\x0b\n\x01\x0c\x12\x03\x1e\0\x122\
    \xb7\x0b\x20This\x20file\x20is\x20part\x20of\x20Moonfire\x20NVR,\x20a\
    \x20security\x20camera\x20digital\x20video\x20recorder.\n\x20Copyright\
    \x20(C)\x202018\x20Scott\x20Lamb\x20<slamb@slamb.org>\n\n\x20This\x20pro\
    gram\x20is\x20free\x20software:\x20you\x20can\x20redistribute\x20it\x20a\
    nd/or\x20modify\n\x20it\x20under\x20the\x20terms\x20of\x20the\x20GNU\x20\
    General\x20Public\x20License\x20as\x20published\x20by\n\x20the\x20Free\
    \x20Software\x20Foundation,\x20either\x20version\x203\x20of\x20the\x20Li\
    cense,\x20or\n\x20(at\x20your\x20option)\x20any\x20later\x20version.\n\n\
    \x20In\x20addition,\x20as\x20a\x20special\x20exception,\x20the\x20copyri\
    ght\x20holders\x20give\n\x20permission\x20to\x20link\x20the\x20code\x20o\
    f\x20portions\x20of\x20this\x20program\x20with\x20the\n\x20OpenSSL\x20li\
    brary\x20under\x20certain\x20conditions\x20as\x20described\x20in\x20each\
    \n\x20individual\x20source\x20file,\x20and\x20distribute\x20linked\x20co\
    mbinations\x20including\n\x20the\x20two.\n\n\x20You\x20must\x20obey\x20t\
    he\x20GNU\x20General\x20Public\x20License\x20in\x20all\x20respects\x20fo\
    r\x20all\n\x20of\x20the\x20code\x20used\x20other\x20than\x20OpenSSL.\x20\
    If\x20you\x20modify\x20file(s)\x20with\x20this\n\x20exception,\x20you\
    \x20may\x20extend\x20this\x20exception\x20to\x20your\x20version\x20of\
    \x20the\n\x20file(s),\x20but\x20you\x20are\x20not\x20obligated\x20to\x20\
    do\x20so.\x20If\x20you\x20do\x20not\x20wish\x20to\x20do\n\x20so,\x20dele\
    te\x20this\x20exception\x20statement\x20from\x20your\x20version.\x20If\
    \x20you\x20delete\n\x20this\x20exception\x20statement\x20from\x20all\x20\
    source\x20files\x20in\x20the\x20program,\x20then\n\x20also\x20delete\x20\
    it\x20here.\n\n\x20This\x20program\x20is\x20distributed\x20in\x20the\x20\
    hope\x20that\x20it\x20will\x20be\x20useful,\n\x20but\x20WITHOUT\x20ANY\
    \x20WARRANTY;\x20without\x20even\x20the\x20implied\x20warranty\x20of\n\
    \x20MERCHANTABILITY\x20or\x20FITNESS\x20FOR\x20A\x20PARTICULAR\x20PURPOS\
    E.\x20\x20See\x20the\n\x20GNU\x20General\x20Public\x20License\x20for\x20\
    more\x20details.\n\n\x20You\x20should\x20have\x20received\x20a\x20copy\
    \x20of\x20the\x20GNU\x20General\x20Public\x20License\n\x20along\x20with\
    \x20this\x20program.\x20\x20If\x20not,\x20see\x20<http://www.gnu.org/lic\
    enses/>.\n\n\xc4\x07\n\x02\x04\0\x12\x041\0H\x01\x1a\xb7\x07\x20Metadata\
    \x20stored\x20in\x20sample\x20file\x20dirs\x20as\x20\"<dir>/meta\".\x20T\
    his\x20is\x20checked\n\x20against\x20the\x20metadata\x20stored\x20within\
    \x20the\x20database\x20to\x20detect\x20inconsistencies\n\x20between\x20t\
    he\x20directory\x20and\x20database,\x20including\x20the\x20following:\n\
    \n\x20*\x20sample\x20file\x20directory's\x20disk\x20not\x20being\x20moun\
    ted.\n\x20*\x20mixing\x20up\x20mount\x20points\x20of\x20two\x20sample\
    \x20file\x20directories\x20belonging\x20to\x20the\n\x20\x20\x20same\x20d\
    atabase.\n\x20*\x20directory\x20renames\x20not\x20properly\x20recorded\
    \x20in\x20the\x20database.\n\x20*\x20restoration\x20of\x20the\x20databas\
    e\x20from\x20backup\x20but\x20not\x20the\x20sample\x20file\n\x20\x20\x20\
    directory.\n\x20*\x20restoration\x20of\x20the\x20sample\x20file\x20direc\
    tory\x20but\x20not\x20the\x20database.\n\x20*\x20two\x20sample\x20file\
    \x20directory\x20paths\x20pointed\x20at\x20the\x20same\x20inode\x20via\
    \x20symlinks\n\x20\x20\x20or\x20non-canonical\x20paths.\x20(Note\x20that\
    \x20flock(2)\x20has\x20a\x20design\x20flaw\x20in\x20which\n\x20\x20\x20m\
    ultiple\x20file\x20descriptors\x20can\x20share\x20a\x20lock,\x20so\x20th\
    e\x20current\x20locking\x20scheme\n\x20\x20\x20is\x20not\x20sufficient\
    \x20to\x20detect\x20this\x20otherwise.)\n\x20*\x20database\x20and\x20sam\
    ple\x20file\x20directories\x20forked\x20from\x20the\x20same\x20version,\
    \x20opened\n\x20\x20\x20the\x20same\x20number\x20of\x20times,\x20then\
    \x20crossed.\n\n\n\n\x03\x04\0\x01\x12\x031\x08\x0f\n\xcf\x01\n\x04\x04\
    \0\x02\0\x12\x035\x02\x14\x1a\xc1\x01\x20A\x20uuid\x20associated\x20with\
    \x20the\x20database,\x20in\x20binary\x20form.\x20dir_uuid\x20is\x20stric\
    tly\n\x20more\x20powerful,\x20but\x20it\x20improves\x20diagnostics\x20to\
    \x20know\x20if\x20the\x20directory\n\x20belongs\x20to\x20the\x20expected\
    \x20database\x20at\x20all\x20or\x20not.\n\n\r\n\x05\x04\0\x02\0\x04\x12\
    \x045\x021\x11\n\x0c\n\x05\x04\0\x02\0\x05\x12\x035\x02\x07\n\x0c\n\x05\
    \x04\0\x02\0\x01\x12\x035\x08\x0f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x035\
    \x12\x13\n;\n\x04\x04\0\x02\x01\x12\x038\x02\x15\x1a.\x20A\x20uuid\x20as\
    sociated\x20with\x20the\x20directory\x20itself.\n\n\r\n\x05\x04\0\x02\
    \x01\x04\x12\x048\x025\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x038\x02\
    \x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x038\x08\x10\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x038\x13\x14\nE\n\x04\x04\0\x03\0\x12\x04;\x02>\x03\x1a\
    7\x20Corresponds\x20to\x20an\x20entry\x20in\x20the\x20`open`\x20database\
    \x20table.\n\n\x0c\n\x05\x04\0\x03\0\x01\x12\x03;\n\x0e\n\r\n\x06\x04\0\
    \x03\0\x02\0\x12\x03<\x04\x12\n\x0f\n\x07\x04\0\x03\0\x02\0\x04\x12\x04<\
    \x04;\x10\n\x0e\n\x07\x04\0\x03\0\x02\0\x05\x12\x03<\x04\n\n\x0e\n\x07\
    \x04\0\x03\0\x02\0\x01\x12\x03<\x0b\r\n\x0e\n\x07\x04\0\x03\0\x02\0\x03\
    \x12\x03<\x10\x11\n\r\n\x06\x04\0\x03\0\x02\x01\x12\x03=\x04\x13\n\x0f\n\
    \x07\x04\0\x03\0\x02\x01\x04\x12\x04=\x04<\x12\n\x0e\n\x07\x04\0\x03\0\
    \x02\x01\x05\x12\x03=\x04\t\n\x0e\n\x07\x04\0\x03\0\x02\x01\x01\x12\x03=\
    \n\x0e\n\x0e\n\x07\x04\0\x03\0\x02\x01\x03\x12\x03=\x11\x12\n|\n\x04\x04\
    \0\x02\x02\x12\x03B\x02\x1e\x1ao\x20The\x20last\x20open\x20that\x20was\
    \x20known\x20to\x20be\x20recorded\x20in\x20the\x20database\x20as\x20comp\
    leted.\n\x20Absent\x20if\x20this\x20has\x20never\x20happened.\n\n\r\n\
    \x05\x04\0\x02\x02\x04\x12\x04B\x02>\x03\n\x0c\n\x05\x04\0\x02\x02\x06\
    \x12\x03B\x02\x06\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03B\x07\x19\n\x0c\n\
    \x05\x04\0\x02\x02\x03\x12\x03B\x1c\x1d\n\xd6\x01\n\x04\x04\0\x02\x03\
    \x12\x03G\x02\x1c\x1a\xc8\x01\x20The\x20last\x20run\x20which\x20is\x20in\
    \x20progress,\x20if\x20different\x20from\x20last_complete_open.\n\x20Thi\
    s\x20may\x20or\x20may\x20not\x20have\x20been\x20recorded\x20in\x20the\
    \x20database,\x20but\x20it's\n\x20guaranteed\x20that\x20no\x20data\x20ha\
    s\x20yet\x20been\x20written\x20by\x20this\x20open.\n\n\r\n\x05\x04\0\x02\
    \x03\x04\x12\x04G\x02B\x1e\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03G\x02\
    \x06\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03G\x07\x17\n\x0c\n\x05\x04\0\
    \x02\x03\x03\x12\x03G\x1a\x1bb\x06proto3\
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

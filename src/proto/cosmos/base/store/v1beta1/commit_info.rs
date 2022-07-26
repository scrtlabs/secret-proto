// This file is generated by rust-protobuf 3.1.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `cosmos/base/store/v1beta1/commit_info.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

///  CommitInfo defines commit information used by the multi-store when committing
///  a version/height.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.base.store.v1beta1.CommitInfo)
pub struct CommitInfo {
    // message fields
    // @@protoc_insertion_point(field:cosmos.base.store.v1beta1.CommitInfo.version)
    pub version: i64,
    // @@protoc_insertion_point(field:cosmos.base.store.v1beta1.CommitInfo.store_infos)
    pub store_infos: ::std::vec::Vec<StoreInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.base.store.v1beta1.CommitInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CommitInfo {
    fn default() -> &'a CommitInfo {
        <CommitInfo as ::protobuf::Message>::default_instance()
    }
}

impl CommitInfo {
    pub fn new() -> CommitInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "version",
            |m: &CommitInfo| { &m.version },
            |m: &mut CommitInfo| { &mut m.version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "store_infos",
            |m: &CommitInfo| { &m.store_infos },
            |m: &mut CommitInfo| { &mut m.store_infos },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CommitInfo>(
            "CommitInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CommitInfo {
    const NAME: &'static str = "CommitInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.version = is.read_int64()?;
                },
                18 => {
                    self.store_infos.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.version != 0 {
            my_size += ::protobuf::rt::int64_size(1, self.version);
        }
        for value in &self.store_infos {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.version != 0 {
            os.write_int64(1, self.version)?;
        }
        for v in &self.store_infos {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CommitInfo {
        CommitInfo::new()
    }

    fn clear(&mut self) {
        self.version = 0;
        self.store_infos.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CommitInfo {
        static instance: CommitInfo = CommitInfo {
            version: 0,
            store_infos: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CommitInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CommitInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CommitInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommitInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  StoreInfo defines store-specific commit information. It contains a reference
///  between a store name and the commit ID.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.base.store.v1beta1.StoreInfo)
pub struct StoreInfo {
    // message fields
    // @@protoc_insertion_point(field:cosmos.base.store.v1beta1.StoreInfo.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:cosmos.base.store.v1beta1.StoreInfo.commit_id)
    pub commit_id: ::protobuf::MessageField<CommitID>,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.base.store.v1beta1.StoreInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StoreInfo {
    fn default() -> &'a StoreInfo {
        <StoreInfo as ::protobuf::Message>::default_instance()
    }
}

impl StoreInfo {
    pub fn new() -> StoreInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &StoreInfo| { &m.name },
            |m: &mut StoreInfo| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, CommitID>(
            "commit_id",
            |m: &StoreInfo| { &m.commit_id },
            |m: &mut StoreInfo| { &mut m.commit_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StoreInfo>(
            "StoreInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StoreInfo {
    const NAME: &'static str = "StoreInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.name = is.read_string()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.commit_id)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if let Some(v) = self.commit_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(v) = self.commit_id.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> StoreInfo {
        StoreInfo::new()
    }

    fn clear(&mut self) {
        self.name.clear();
        self.commit_id.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StoreInfo {
        static instance: StoreInfo = StoreInfo {
            name: ::std::string::String::new(),
            commit_id: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StoreInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StoreInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StoreInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StoreInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  CommitID defines the committment information when a specific store is
///  committed.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.base.store.v1beta1.CommitID)
pub struct CommitID {
    // message fields
    // @@protoc_insertion_point(field:cosmos.base.store.v1beta1.CommitID.version)
    pub version: i64,
    // @@protoc_insertion_point(field:cosmos.base.store.v1beta1.CommitID.hash)
    pub hash: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.base.store.v1beta1.CommitID.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CommitID {
    fn default() -> &'a CommitID {
        <CommitID as ::protobuf::Message>::default_instance()
    }
}

impl CommitID {
    pub fn new() -> CommitID {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "version",
            |m: &CommitID| { &m.version },
            |m: &mut CommitID| { &mut m.version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hash",
            |m: &CommitID| { &m.hash },
            |m: &mut CommitID| { &mut m.hash },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CommitID>(
            "CommitID",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CommitID {
    const NAME: &'static str = "CommitID";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.version = is.read_int64()?;
                },
                18 => {
                    self.hash = is.read_bytes()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.version != 0 {
            my_size += ::protobuf::rt::int64_size(1, self.version);
        }
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.version != 0 {
            os.write_int64(1, self.version)?;
        }
        if !self.hash.is_empty() {
            os.write_bytes(2, &self.hash)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CommitID {
        CommitID::new()
    }

    fn clear(&mut self) {
        self.version = 0;
        self.hash.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CommitID {
        static instance: CommitID = CommitID {
            version: 0,
            hash: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CommitID {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CommitID").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CommitID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommitID {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n+cosmos/base/store/v1beta1/commit_info.proto\x12\x19cosmos.base.store.\
    v1beta1\x1a\x14gogoproto/gogo.proto\"s\n\nCommitInfo\x12\x18\n\x07versio\
    n\x18\x01\x20\x01(\x03R\x07version\x12K\n\x0bstore_infos\x18\x02\x20\x03\
    (\x0b2$.cosmos.base.store.v1beta1.StoreInfoR\nstoreInfosB\x04\xc8\xde\
    \x1f\0\"g\n\tStoreInfo\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\
    F\n\tcommit_id\x18\x02\x20\x01(\x0b2#.cosmos.base.store.v1beta1.CommitID\
    R\x08commitIdB\x04\xc8\xde\x1f\0\">\n\x08CommitID\x12\x18\n\x07version\
    \x18\x01\x20\x01(\x03R\x07version\x12\x12\n\x04hash\x18\x02\x20\x01(\x0c\
    R\x04hash:\x04\x98\xa0\x1f\0B*Z(github.com/cosmos/cosmos-sdk/store/types\
    J\xe8\x06\n\x06\x12\x04\0\0\x1c\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\
    \x08\n\x01\x02\x12\x03\x01\0\"\n\t\n\x02\x03\0\x12\x03\x03\0\x1e\n\x08\n\
    \x01\x08\x12\x03\x05\0?\n\t\n\x02\x08\x0b\x12\x03\x05\0?\nn\n\x02\x04\0\
    \x12\x04\t\0\x0c\x01\x1ab\x20CommitInfo\x20defines\x20commit\x20informat\
    ion\x20used\x20by\x20the\x20multi-store\x20when\x20committing\n\x20a\x20\
    version/height.\n\n\n\n\x03\x04\0\x01\x12\x03\t\x08\x12\n\x0b\n\x04\x04\
    \0\x02\0\x12\x03\n\x02%\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\n\x02\x07\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\x15\x1c\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\n#$\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0b\x02D\n\x0c\n\x05\x04\
    \0\x02\x01\x04\x12\x03\x0b\x02\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\
    \x0b\x0b\x14\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0b\x15\x20\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\x0b#$\n\x0c\n\x05\x04\0\x02\x01\x08\x12\
    \x03\x0b%C\n\x0f\n\x08\x04\0\x02\x01\x08\xe9\xfb\x03\x12\x03\x0b&B\n\x83\
    \x01\n\x02\x04\x01\x12\x04\x10\0\x13\x01\x1aw\x20StoreInfo\x20defines\
    \x20store-specific\x20commit\x20information.\x20It\x20contains\x20a\x20r\
    eference\n\x20between\x20a\x20store\x20name\x20and\x20the\x20commit\x20I\
    D.\n\n\n\n\x03\x04\x01\x01\x12\x03\x10\x08\x11\n\x0b\n\x04\x04\x01\x02\0\
    \x12\x03\x11\x02\x19\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x11\x02\x08\n\
    \x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x11\x0b\x0f\n\x0c\n\x05\x04\x01\x02\
    \0\x03\x12\x03\x11\x17\x18\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x12\x028\
    \n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\x12\x02\n\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03\x12\x0b\x14\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\
    \x12\x17\x18\n\x0c\n\x05\x04\x01\x02\x01\x08\x12\x03\x12\x197\n\x0f\n\
    \x08\x04\x01\x02\x01\x08\xe9\xfb\x03\x12\x03\x12\x1a6\n_\n\x02\x04\x02\
    \x12\x04\x17\0\x1c\x01\x1aS\x20CommitID\x20defines\x20the\x20committment\
    \x20information\x20when\x20a\x20specific\x20store\x20is\n\x20committed.\
    \n\n\n\n\x03\x04\x02\x01\x12\x03\x17\x08\x10\n\n\n\x03\x04\x02\x07\x12\
    \x03\x18\x02.\n\r\n\x06\x04\x02\x07\x83\xf4\x03\x12\x03\x18\x02.\n\x0b\n\
    \x04\x04\x02\x02\0\x12\x03\x1a\x02\x14\n\x0c\n\x05\x04\x02\x02\0\x05\x12\
    \x03\x1a\x02\x07\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x1a\x08\x0f\n\x0c\
    \n\x05\x04\x02\x02\0\x03\x12\x03\x1a\x12\x13\n\x0b\n\x04\x04\x02\x02\x01\
    \x12\x03\x1b\x02\x14\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x1b\x02\x07\
    \n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x1b\x08\x0c\n\x0c\n\x05\x04\x02\
    \x02\x01\x03\x12\x03\x1b\x12\x13b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::gogo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(CommitInfo::generated_message_descriptor_data());
            messages.push(StoreInfo::generated_message_descriptor_data());
            messages.push(CommitID::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}

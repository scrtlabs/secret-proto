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

//! Generated file from `tendermint/types/block.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:tendermint.types.Block)
pub struct Block {
    // message fields
    // @@protoc_insertion_point(field:tendermint.types.Block.header)
    pub header: ::protobuf::MessageField<super::types::Header>,
    // @@protoc_insertion_point(field:tendermint.types.Block.data)
    pub data: ::protobuf::MessageField<super::types::Data>,
    // @@protoc_insertion_point(field:tendermint.types.Block.evidence)
    pub evidence: ::protobuf::MessageField<super::evidence::EvidenceList>,
    // @@protoc_insertion_point(field:tendermint.types.Block.last_commit)
    pub last_commit: ::protobuf::MessageField<super::types::Commit>,
    // special fields
    // @@protoc_insertion_point(special_field:tendermint.types.Block.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Block {
    fn default() -> &'a Block {
        <Block as ::protobuf::Message>::default_instance()
    }
}

impl Block {
    pub fn new() -> Block {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::types::Header>(
            "header",
            |m: &Block| { &m.header },
            |m: &mut Block| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::types::Data>(
            "data",
            |m: &Block| { &m.data },
            |m: &mut Block| { &mut m.data },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::evidence::EvidenceList>(
            "evidence",
            |m: &Block| { &m.evidence },
            |m: &mut Block| { &mut m.evidence },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::types::Commit>(
            "last_commit",
            |m: &Block| { &m.last_commit },
            |m: &mut Block| { &mut m.last_commit },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Block>(
            "Block",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Block {
    const NAME: &'static str = "Block";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.header)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.data)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.evidence)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.last_commit)?;
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
        if let Some(v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.evidence.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.last_commit.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.data.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.evidence.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.last_commit.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> Block {
        Block::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.data.clear();
        self.evidence.clear();
        self.last_commit.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Block {
        static instance: Block = Block {
            header: ::protobuf::MessageField::none(),
            data: ::protobuf::MessageField::none(),
            evidence: ::protobuf::MessageField::none(),
            last_commit: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Block {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Block").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Block {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1ctendermint/types/block.proto\x12\x10tendermint.types\x1a\x14gogopr\
    oto/gogo.proto\x1a\x1ctendermint/types/types.proto\x1a\x1ftendermint/typ\
    es/evidence.proto\"\xee\x01\n\x05Block\x126\n\x06header\x18\x01\x20\x01(\
    \x0b2\x18.tendermint.types.HeaderR\x06headerB\x04\xc8\xde\x1f\0\x120\n\
    \x04data\x18\x02\x20\x01(\x0b2\x16.tendermint.types.DataR\x04dataB\x04\
    \xc8\xde\x1f\0\x12@\n\x08evidence\x18\x03\x20\x01(\x0b2\x1e.tendermint.t\
    ypes.EvidenceListR\x08evidenceB\x04\xc8\xde\x1f\0\x129\n\x0blast_commit\
    \x18\x04\x20\x01(\x0b2\x18.tendermint.types.CommitR\nlastCommitB9Z7githu\
    b.com/tendermint/tendermint/proto/tendermint/typesJ\xa3\x03\n\x06\x12\
    \x04\0\0\x0e\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\
    \x01\0\x19\n\x08\n\x01\x08\x12\x03\x03\0N\n\t\n\x02\x08\x0b\x12\x03\x03\
    \0N\n\t\n\x02\x03\0\x12\x03\x05\0\x1e\n\t\n\x02\x03\x01\x12\x03\x06\0&\n\
    \t\n\x02\x03\x02\x12\x03\x07\0)\n\n\n\x02\x04\0\x12\x04\t\0\x0e\x01\n\n\
    \n\x03\x04\0\x01\x12\x03\t\x08\r\n\x0b\n\x04\x04\0\x02\0\x12\x03\n\x02O\
    \n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\n\x02\x08\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\n\x20&\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\n./\n\x0c\n\x05\
    \x04\0\x02\0\x08\x12\x03\n0N\n\x0f\n\x08\x04\0\x02\0\x08\xe9\xfb\x03\x12\
    \x03\n1M\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0b\x02O\n\x0c\n\x05\x04\0\
    \x02\x01\x06\x12\x03\x0b\x02\x06\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\
    \x0b\x20$\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0b./\n\x0c\n\x05\x04\0\
    \x02\x01\x08\x12\x03\x0b0N\n\x0f\n\x08\x04\0\x02\x01\x08\xe9\xfb\x03\x12\
    \x03\x0b1M\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x0c\x02O\n\x0c\n\x05\x04\0\
    \x02\x02\x06\x12\x03\x0c\x02\x1f\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\
    \x0c\x20(\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0c./\n\x0c\n\x05\x04\0\
    \x02\x02\x08\x12\x03\x0c0N\n\x0f\n\x08\x04\0\x02\x02\x08\xe9\xfb\x03\x12\
    \x03\x0c1M\n\x0b\n\x04\x04\0\x02\x03\x12\x03\r\x020\n\x0c\n\x05\x04\0\
    \x02\x03\x06\x12\x03\r\x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\r\
    \x20+\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\r./b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::gogo::file_descriptor().clone());
            deps.push(super::types::file_descriptor().clone());
            deps.push(super::evidence::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Block::generated_message_descriptor_data());
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

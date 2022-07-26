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

//! Generated file from `cosmos/slashing/v1beta1/tx.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

///  MsgUnjail defines the Msg/Unjail request type
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.slashing.v1beta1.MsgUnjail)
pub struct MsgUnjail {
    // message fields
    // @@protoc_insertion_point(field:cosmos.slashing.v1beta1.MsgUnjail.validator_addr)
    pub validator_addr: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.slashing.v1beta1.MsgUnjail.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgUnjail {
    fn default() -> &'a MsgUnjail {
        <MsgUnjail as ::protobuf::Message>::default_instance()
    }
}

impl MsgUnjail {
    pub fn new() -> MsgUnjail {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "validator_addr",
            |m: &MsgUnjail| { &m.validator_addr },
            |m: &mut MsgUnjail| { &mut m.validator_addr },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgUnjail>(
            "MsgUnjail",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgUnjail {
    const NAME: &'static str = "MsgUnjail";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.validator_addr = is.read_string()?;
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
        if !self.validator_addr.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.validator_addr);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.validator_addr.is_empty() {
            os.write_string(1, &self.validator_addr)?;
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

    fn new() -> MsgUnjail {
        MsgUnjail::new()
    }

    fn clear(&mut self) {
        self.validator_addr.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgUnjail {
        static instance: MsgUnjail = MsgUnjail {
            validator_addr: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgUnjail {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgUnjail").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgUnjail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgUnjail {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  MsgUnjailResponse defines the Msg/Unjail response type
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.slashing.v1beta1.MsgUnjailResponse)
pub struct MsgUnjailResponse {
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.slashing.v1beta1.MsgUnjailResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgUnjailResponse {
    fn default() -> &'a MsgUnjailResponse {
        <MsgUnjailResponse as ::protobuf::Message>::default_instance()
    }
}

impl MsgUnjailResponse {
    pub fn new() -> MsgUnjailResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(0);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgUnjailResponse>(
            "MsgUnjailResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgUnjailResponse {
    const NAME: &'static str = "MsgUnjailResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MsgUnjailResponse {
        MsgUnjailResponse::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgUnjailResponse {
        static instance: MsgUnjailResponse = MsgUnjailResponse {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgUnjailResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgUnjailResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgUnjailResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgUnjailResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20cosmos/slashing/v1beta1/tx.proto\x12\x17cosmos.slashing.v1beta1\
    \x1a\x14gogoproto/gogo.proto\"[\n\tMsgUnjail\x12D\n\x0evalidator_addr\
    \x18\x01\x20\x01(\tR\rvalidatorAddrB\x1d\xf2\xde\x1f\x0eyaml:\"address\"\
    \xea\xde\x1f\x07address:\x08\x88\xa0\x1f\0\x98\xa0\x1f\x01\"\x13\n\x11Ms\
    gUnjailResponse2_\n\x03Msg\x12X\n\x06Unjail\x12\".cosmos.slashing.v1beta\
    1.MsgUnjail\x1a*.cosmos.slashing.v1beta1.MsgUnjailResponseB3Z-github.com\
    /cosmos/cosmos-sdk/x/slashing/types\xa8\xe2\x1e\x01J\xb0\x05\n\x06\x12\
    \x04\0\0\x19\x1c\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\
    \x01\0\x20\n\x08\n\x01\x08\x12\x03\x03\0O\n\t\n\x02\x08\x0b\x12\x03\x03\
    \0O\n\x08\n\x01\x08\x12\x03\x04\0$\n\x0b\n\x04\x08\xa5\xec\x03\x12\x03\
    \x04\0$\n\t\n\x02\x03\0\x12\x03\x06\0\x1e\n3\n\x02\x06\0\x12\x04\t\0\x0e\
    \x01\x1a'\x20Msg\x20defines\x20the\x20slashing\x20Msg\x20service.\n\n\n\
    \n\x03\x06\0\x01\x12\x03\t\x08\x0b\n\xb8\x01\n\x04\x06\0\x02\0\x12\x03\r\
    \x024\x1a\xaa\x01\x20Unjail\x20defines\x20a\x20method\x20for\x20unjailin\
    g\x20a\x20jailed\x20validator,\x20thus\x20returning\n\x20them\x20into\
    \x20the\x20bonded\x20validator\x20set,\x20so\x20they\x20can\x20begin\x20\
    receiving\x20provisions\n\x20and\x20rewards\x20again.\n\n\x0c\n\x05\x06\
    \0\x02\0\x01\x12\x03\r\x06\x0c\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\r\r\
    \x16\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\r!2\n;\n\x02\x04\0\x12\x04\x11\
    \0\x16\x01\x1a/\x20MsgUnjail\x20defines\x20the\x20Msg/Unjail\x20request\
    \x20type\n\n\n\n\x03\x04\0\x01\x12\x03\x11\x08\x11\n\n\n\x03\x04\0\x07\
    \x12\x03\x12\x02.\n\r\n\x06\x04\0\x07\x81\xf4\x03\x12\x03\x12\x02.\n\n\n\
    \x03\x04\0\x07\x12\x03\x13\x02-\n\r\n\x06\x04\0\x07\x83\xf4\x03\x12\x03\
    \x13\x02-\n\x0b\n\x04\x04\0\x02\0\x12\x03\x15\x02i\n\x0c\n\x05\x04\0\x02\
    \0\x05\x12\x03\x15\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x15\t\x17\
    \n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x15\x1a\x1b\n\x0c\n\x05\x04\0\x02\0\
    \x08\x12\x03\x15\x1ch\n\x0f\n\x08\x04\0\x02\0\x08\xee\xfb\x03\x12\x03\
    \x15\x1dF\n\x0f\n\x08\x04\0\x02\0\x08\xed\xfb\x03\x12\x03\x15Hg\nC\n\x02\
    \x04\x01\x12\x03\x19\0\x1c\x1a8\x20MsgUnjailResponse\x20defines\x20the\
    \x20Msg/Unjail\x20response\x20type\n\n\n\n\x03\x04\x01\x01\x12\x03\x19\
    \x08\x19b\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(MsgUnjail::generated_message_descriptor_data());
            messages.push(MsgUnjailResponse::generated_message_descriptor_data());
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

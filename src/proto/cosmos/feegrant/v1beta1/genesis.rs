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

//! Generated file from `cosmos/feegrant/v1beta1/genesis.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

///  GenesisState contains a set of fee allowances, persisted from the store
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.feegrant.v1beta1.GenesisState)
pub struct GenesisState {
    // message fields
    // @@protoc_insertion_point(field:cosmos.feegrant.v1beta1.GenesisState.allowances)
    pub allowances: ::std::vec::Vec<super::feegrant::Grant>,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.feegrant.v1beta1.GenesisState.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GenesisState {
    fn default() -> &'a GenesisState {
        <GenesisState as ::protobuf::Message>::default_instance()
    }
}

impl GenesisState {
    pub fn new() -> GenesisState {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "allowances",
            |m: &GenesisState| { &m.allowances },
            |m: &mut GenesisState| { &mut m.allowances },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GenesisState>(
            "GenesisState",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GenesisState {
    const NAME: &'static str = "GenesisState";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.allowances.push(is.read_message()?);
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
        for value in &self.allowances {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.allowances {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> GenesisState {
        GenesisState::new()
    }

    fn clear(&mut self) {
        self.allowances.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GenesisState {
        static instance: GenesisState = GenesisState {
            allowances: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GenesisState {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GenesisState").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GenesisState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GenesisState {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%cosmos/feegrant/v1beta1/genesis.proto\x12\x17cosmos.feegrant.v1beta1\
    \x1a\x14gogoproto/gogo.proto\x1a&cosmos/feegrant/v1beta1/feegrant.proto\
    \"T\n\x0cGenesisState\x12D\n\nallowances\x18\x01\x20\x03(\x0b2\x1e.cosmo\
    s.feegrant.v1beta1.GrantR\nallowancesB\x04\xc8\xde\x1f\0B)Z'github.com/c\
    osmos/cosmos-sdk/x/feegrantJ\xa8\x02\n\x06\x12\x04\x01\0\x0c\x01\n\"\n\
    \x01\x0c\x12\x03\x01\0\x12\x1a\x18\x20Since:\x20cosmos-sdk\x200.43\n\n\
    \x08\n\x01\x02\x12\x03\x02\0\x20\n\t\n\x02\x03\0\x12\x03\x04\0\x1e\n\t\n\
    \x02\x03\x01\x12\x03\x05\00\n\x08\n\x01\x08\x12\x03\x07\0>\n\t\n\x02\x08\
    \x0b\x12\x03\x07\0>\nU\n\x02\x04\0\x12\x04\n\0\x0c\x01\x1aI\x20GenesisSt\
    ate\x20contains\x20a\x20set\x20of\x20fee\x20allowances,\x20persisted\x20\
    from\x20the\x20store\n\n\n\n\x03\x04\0\x01\x12\x03\n\x08\x14\n\x0b\n\x04\
    \x04\0\x02\0\x12\x03\x0b\x02?\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x0b\
    \x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x0b\x0b\x10\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\x0b\x11\x1b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0b\
    \x1e\x1f\n\x0c\n\x05\x04\0\x02\0\x08\x12\x03\x0b\x20>\n\x0f\n\x08\x04\0\
    \x02\0\x08\xe9\xfb\x03\x12\x03\x0b!=b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::gogo::file_descriptor().clone());
            deps.push(super::feegrant::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GenesisState::generated_message_descriptor_data());
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

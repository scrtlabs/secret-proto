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

//! Generated file from `cosmos/crisis/v1beta1/genesis.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

///  GenesisState defines the crisis module's genesis state.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.crisis.v1beta1.GenesisState)
pub struct GenesisState {
    // message fields
    ///  constant_fee is the fee used to verify the invariant in the crisis
    ///  module.
    // @@protoc_insertion_point(field:cosmos.crisis.v1beta1.GenesisState.constant_fee)
    pub constant_fee: ::protobuf::MessageField<super::coin::Coin>,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.crisis.v1beta1.GenesisState.special_fields)
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
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::coin::Coin>(
            "constant_fee",
            |m: &GenesisState| { &m.constant_fee },
            |m: &mut GenesisState| { &mut m.constant_fee },
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
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.constant_fee)?;
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
        if let Some(v) = self.constant_fee.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.constant_fee.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> GenesisState {
        GenesisState::new()
    }

    fn clear(&mut self) {
        self.constant_fee.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GenesisState {
        static instance: GenesisState = GenesisState {
            constant_fee: ::protobuf::MessageField::none(),
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
    \n#cosmos/crisis/v1beta1/genesis.proto\x12\x15cosmos.crisis.v1beta1\x1a\
    \x14gogoproto/gogo.proto\x1a\x1ecosmos/base/v1beta1/coin.proto\"i\n\x0cG\
    enesisState\x12Y\n\x0cconstant_fee\x18\x03\x20\x01(\x0b2\x19.cosmos.base\
    .v1beta1.CoinR\x0bconstantFeeB\x1b\xf2\xde\x1f\x13yaml:\"constant_fee\"\
    \xc8\xde\x1f\0B-Z+github.com/cosmos/cosmos-sdk/x/crisis/typesJ\xd1\x02\n\
    \x06\x12\x04\0\0\x0e\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\
    \x12\x03\x01\0\x1e\n\x08\n\x01\x08\x12\x03\x03\0B\n\t\n\x02\x08\x0b\x12\
    \x03\x03\0B\n\t\n\x02\x03\0\x12\x03\x05\0\x1e\n\t\n\x02\x03\x01\x12\x03\
    \x06\0(\nE\n\x02\x04\0\x12\x04\t\0\x0e\x01\x1a9\x20GenesisState\x20defin\
    es\x20the\x20crisis\x20module's\x20genesis\x20state.\n\n\n\n\x03\x04\0\
    \x01\x12\x03\t\x08\x14\n[\n\x04\x04\0\x02\0\x12\x04\x0c\x02\rU\x1aM\x20c\
    onstant_fee\x20is\x20the\x20fee\x20used\x20to\x20verify\x20the\x20invari\
    ant\x20in\x20the\x20crisis\n\x20module.\n\n\x0c\n\x05\x04\0\x02\0\x06\
    \x12\x03\x0c\x02\x1a\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0c\x1b'\n\x0c\
    \n\x05\x04\0\x02\0\x03\x12\x03\x0c*+\n\x0c\n\x05\x04\0\x02\0\x08\x12\x03\
    \r\x06T\n\x0f\n\x08\x04\0\x02\0\x08\xe9\xfb\x03\x12\x03\r\x07#\n\x0f\n\
    \x08\x04\0\x02\0\x08\xee\xfb\x03\x12\x03\r%Sb\x06proto3\
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
            deps.push(super::coin::file_descriptor().clone());
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

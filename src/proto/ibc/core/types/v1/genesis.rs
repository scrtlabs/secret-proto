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

//! Generated file from `ibc/core/types/v1/genesis.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

///  GenesisState defines the ibc module's genesis state.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:ibc.core.types.v1.GenesisState)
pub struct GenesisState {
    // message fields
    ///  ICS002 - Clients genesis state
    // @@protoc_insertion_point(field:ibc.core.types.v1.GenesisState.client_genesis)
    pub client_genesis: ::protobuf::MessageField<super::genesis::GenesisState>,
    ///  ICS003 - Connections genesis state
    // @@protoc_insertion_point(field:ibc.core.types.v1.GenesisState.connection_genesis)
    pub connection_genesis: ::protobuf::MessageField<super::genesis::GenesisState>,
    ///  ICS004 - Channel genesis state
    // @@protoc_insertion_point(field:ibc.core.types.v1.GenesisState.channel_genesis)
    pub channel_genesis: ::protobuf::MessageField<super::genesis::GenesisState>,
    // special fields
    // @@protoc_insertion_point(special_field:ibc.core.types.v1.GenesisState.special_fields)
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
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::genesis::GenesisState>(
            "client_genesis",
            |m: &GenesisState| { &m.client_genesis },
            |m: &mut GenesisState| { &mut m.client_genesis },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::genesis::GenesisState>(
            "connection_genesis",
            |m: &GenesisState| { &m.connection_genesis },
            |m: &mut GenesisState| { &mut m.connection_genesis },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::genesis::GenesisState>(
            "channel_genesis",
            |m: &GenesisState| { &m.channel_genesis },
            |m: &mut GenesisState| { &mut m.channel_genesis },
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
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.client_genesis)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.connection_genesis)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.channel_genesis)?;
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
        if let Some(v) = self.client_genesis.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.connection_genesis.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.channel_genesis.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.client_genesis.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.connection_genesis.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.channel_genesis.as_ref() {
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
        self.client_genesis.clear();
        self.connection_genesis.clear();
        self.channel_genesis.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GenesisState {
        static instance: GenesisState = GenesisState {
            client_genesis: ::protobuf::MessageField::none(),
            connection_genesis: ::protobuf::MessageField::none(),
            channel_genesis: ::protobuf::MessageField::none(),
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
    \n\x1fibc/core/types/v1/genesis.proto\x12\x11ibc.core.types.v1\x1a\x14go\
    goproto/gogo.proto\x1a\x20ibc/core/client/v1/genesis.proto\x1a$ibc/core/\
    connection/v1/genesis.proto\x1a!ibc/core/channel/v1/genesis.proto\"\xda\
    \x02\n\x0cGenesisState\x12f\n\x0eclient_genesis\x18\x01\x20\x01(\x0b2\
    \x20.ibc.core.client.v1.GenesisStateR\rclientGenesisB\x1d\xf2\xde\x1f\
    \x15yaml:\"client_genesis\"\xc8\xde\x1f\0\x12v\n\x12connection_genesis\
    \x18\x02\x20\x01(\x0b2$.ibc.core.connection.v1.GenesisStateR\x11connecti\
    onGenesisB!\xf2\xde\x1f\x19yaml:\"connection_genesis\"\xc8\xde\x1f\0\x12\
    j\n\x0fchannel_genesis\x18\x03\x20\x01(\x0b2!.ibc.core.channel.v1.Genesi\
    sStateR\x0echannelGenesisB\x1e\xf2\xde\x1f\x16yaml:\"channel_genesis\"\
    \xc8\xde\x1f\0B0Z.github.com/cosmos/ibc-go/v3/modules/core/typesJ\xcf\
    \x04\n\x06\x12\x04\0\0\x16\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\
    \x01\x02\x12\x03\x02\0\x1a\n\x08\n\x01\x08\x12\x03\x04\0E\n\t\n\x02\x08\
    \x0b\x12\x03\x04\0E\n\t\n\x02\x03\0\x12\x03\x06\0\x1e\n\t\n\x02\x03\x01\
    \x12\x03\x07\0*\n\t\n\x02\x03\x02\x12\x03\x08\0.\n\t\n\x02\x03\x03\x12\
    \x03\t\0+\nB\n\x02\x04\0\x12\x04\x0c\0\x16\x01\x1a6\x20GenesisState\x20d\
    efines\x20the\x20ibc\x20module's\x20genesis\x20state.\n\n\n\n\x03\x04\0\
    \x01\x12\x03\x0c\x08\x14\n.\n\x04\x04\0\x02\0\x12\x04\x0e\x02\x0fW\x1a\
    \x20\x20ICS002\x20-\x20Clients\x20genesis\x20state\n\n\x0c\n\x05\x04\0\
    \x02\0\x06\x12\x03\x0e\x02!\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0e\"0\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0e34\n\x0c\n\x05\x04\0\x02\0\x08\x12\
    \x03\x0f\x06V\n\x0f\n\x08\x04\0\x02\0\x08\xe9\xfb\x03\x12\x03\x0f\x07#\n\
    \x0f\n\x08\x04\0\x02\0\x08\xee\xfb\x03\x12\x03\x0f%U\n2\n\x04\x04\0\x02\
    \x01\x12\x04\x11\x02\x12[\x1a$\x20ICS003\x20-\x20Connections\x20genesis\
    \x20state\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x11\x02%\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\x11&8\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\
    \x11;<\n\x0c\n\x05\x04\0\x02\x01\x08\x12\x03\x12\x06Z\n\x0f\n\x08\x04\0\
    \x02\x01\x08\xe9\xfb\x03\x12\x03\x12\x07#\n\x0f\n\x08\x04\0\x02\x01\x08\
    \xee\xfb\x03\x12\x03\x12%Y\n.\n\x04\x04\0\x02\x02\x12\x04\x14\x02\x15X\
    \x1a\x20\x20ICS004\x20-\x20Channel\x20genesis\x20state\n\n\x0c\n\x05\x04\
    \0\x02\x02\x06\x12\x03\x14\x02\"\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\
    \x14#2\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x1456\n\x0c\n\x05\x04\0\x02\
    \x02\x08\x12\x03\x15\x06W\n\x0f\n\x08\x04\0\x02\x02\x08\xe9\xfb\x03\x12\
    \x03\x15\x07#\n\x0f\n\x08\x04\0\x02\x02\x08\xee\xfb\x03\x12\x03\x15%Vb\
    \x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::gogo::file_descriptor().clone());
            deps.push(super::genesis::file_descriptor().clone());
            deps.push(super::genesis::file_descriptor().clone());
            deps.push(super::genesis::file_descriptor().clone());
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

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

//! Generated file from `ibc/core/connection/v1/genesis.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

///  GenesisState defines the ibc connection submodule's genesis state.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:ibc.core.connection.v1.GenesisState)
pub struct GenesisState {
    // message fields
    // @@protoc_insertion_point(field:ibc.core.connection.v1.GenesisState.connections)
    pub connections: ::std::vec::Vec<super::connection::IdentifiedConnection>,
    // @@protoc_insertion_point(field:ibc.core.connection.v1.GenesisState.client_connection_paths)
    pub client_connection_paths: ::std::vec::Vec<super::connection::ConnectionPaths>,
    ///  the sequence for the next generated connection identifier
    // @@protoc_insertion_point(field:ibc.core.connection.v1.GenesisState.next_connection_sequence)
    pub next_connection_sequence: u64,
    // @@protoc_insertion_point(field:ibc.core.connection.v1.GenesisState.params)
    pub params: ::protobuf::MessageField<super::connection::Params>,
    // special fields
    // @@protoc_insertion_point(special_field:ibc.core.connection.v1.GenesisState.special_fields)
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
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "connections",
            |m: &GenesisState| { &m.connections },
            |m: &mut GenesisState| { &mut m.connections },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "client_connection_paths",
            |m: &GenesisState| { &m.client_connection_paths },
            |m: &mut GenesisState| { &mut m.client_connection_paths },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "next_connection_sequence",
            |m: &GenesisState| { &m.next_connection_sequence },
            |m: &mut GenesisState| { &mut m.next_connection_sequence },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::connection::Params>(
            "params",
            |m: &GenesisState| { &m.params },
            |m: &mut GenesisState| { &mut m.params },
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
                    self.connections.push(is.read_message()?);
                },
                18 => {
                    self.client_connection_paths.push(is.read_message()?);
                },
                24 => {
                    self.next_connection_sequence = is.read_uint64()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.params)?;
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
        for value in &self.connections {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.client_connection_paths {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.next_connection_sequence != 0 {
            my_size += ::protobuf::rt::uint64_size(3, self.next_connection_sequence);
        }
        if let Some(v) = self.params.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.connections {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        for v in &self.client_connection_paths {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.next_connection_sequence != 0 {
            os.write_uint64(3, self.next_connection_sequence)?;
        }
        if let Some(v) = self.params.as_ref() {
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

    fn new() -> GenesisState {
        GenesisState::new()
    }

    fn clear(&mut self) {
        self.connections.clear();
        self.client_connection_paths.clear();
        self.next_connection_sequence = 0;
        self.params.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GenesisState {
        static instance: GenesisState = GenesisState {
            connections: ::std::vec::Vec::new(),
            client_connection_paths: ::std::vec::Vec::new(),
            next_connection_sequence: 0,
            params: ::protobuf::MessageField::none(),
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
    \n$ibc/core/connection/v1/genesis.proto\x12\x16ibc.core.connection.v1\
    \x1a\x14gogoproto/gogo.proto\x1a'ibc/core/connection/v1/connection.proto\
    \"\x8b\x03\n\x0cGenesisState\x12T\n\x0bconnections\x18\x01\x20\x03(\x0b2\
    ,.ibc.core.connection.v1.IdentifiedConnectionR\x0bconnectionsB\x04\xc8\
    \xde\x1f\0\x12\x87\x01\n\x17client_connection_paths\x18\x02\x20\x03(\x0b\
    2'.ibc.core.connection.v1.ConnectionPathsR\x15clientConnectionPathsB&\
    \xf2\xde\x1f\x1eyaml:\"client_connection_paths\"\xc8\xde\x1f\0\x12]\n\
    \x18next_connection_sequence\x18\x03\x20\x01(\x04R\x16nextConnectionSequ\
    enceB#\xf2\xde\x1f\x1fyaml:\"next_connection_sequence\"\x12<\n\x06params\
    \x18\x04\x20\x01(\x0b2\x1e.ibc.core.connection.v1.ParamsR\x06paramsB\x04\
    \xc8\xde\x1f\0B>Z<github.com/cosmos/ibc-go/v3/modules/core/03-connection\
    /typesJ\xe8\x04\n\x06\x12\x04\0\0\x11\x01\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\x08\n\x01\x02\x12\x03\x02\0\x1f\n\x08\n\x01\x08\x12\x03\x04\0S\n\
    \t\n\x02\x08\x0b\x12\x03\x04\0S\n\t\n\x02\x03\0\x12\x03\x06\0\x1e\n\t\n\
    \x02\x03\x01\x12\x03\x07\01\nP\n\x02\x04\0\x12\x04\n\0\x11\x01\x1aD\x20G\
    enesisState\x20defines\x20the\x20ibc\x20connection\x20submodule's\x20gen\
    esis\x20state.\n\n\n\n\x03\x04\0\x01\x12\x03\n\x08\x14\n\x0b\n\x04\x04\0\
    \x02\0\x12\x03\x0b\x02[\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x0b\x02\n\n\
    \x0c\n\x05\x04\0\x02\0\x06\x12\x03\x0b\x0b\x1f\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\x0b\x20+\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0b:;\n\x0c\n\
    \x05\x04\0\x02\0\x08\x12\x03\x0b<Z\n\x0f\n\x08\x04\0\x02\0\x08\xe9\xfb\
    \x03\x12\x03\x0b=Y\n\x0c\n\x04\x04\0\x02\x01\x12\x04\x0c\x02\r`\n\x0c\n\
    \x05\x04\0\x02\x01\x04\x12\x03\x0c\x02\n\n\x0c\n\x05\x04\0\x02\x01\x06\
    \x12\x03\x0c\x0b\x1a\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0c\x207\n\
    \x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0c:;\n\x0c\n\x05\x04\0\x02\x01\x08\
    \x12\x03\r\x06_\n\x0f\n\x08\x04\0\x02\x01\x08\xe9\xfb\x03\x12\x03\r\x07#\
    \n\x0f\n\x08\x04\0\x02\x01\x08\xee\xfb\x03\x12\x03\r%^\nH\n\x04\x04\0\
    \x02\x02\x12\x03\x0f\x02c\x1a;\x20the\x20sequence\x20for\x20the\x20next\
    \x20generated\x20connection\x20identifier\n\n\x0c\n\x05\x04\0\x02\x02\
    \x05\x12\x03\x0f\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0f\t!\n\
    \x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0f$%\n\x0c\n\x05\x04\0\x02\x02\x08\
    \x12\x03\x0f&b\n\x0f\n\x08\x04\0\x02\x02\x08\xee\xfb\x03\x12\x03\x0f'a\n\
    \x0b\n\x04\x04\0\x02\x03\x12\x03\x10\x02E\n\x0c\n\x05\x04\0\x02\x03\x06\
    \x12\x03\x10\x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x10\t\x0f\n\
    \x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x10$%\n\x0c\n\x05\x04\0\x02\x03\x08\
    \x12\x03\x10&D\n\x0f\n\x08\x04\0\x02\x03\x08\xe9\xfb\x03\x12\x03\x10'Cb\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::gogo::file_descriptor().clone());
            deps.push(super::connection::file_descriptor().clone());
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

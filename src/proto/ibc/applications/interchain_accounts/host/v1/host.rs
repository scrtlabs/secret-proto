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

//! Generated file from `ibc/applications/interchain_accounts/host/v1/host.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

///  Params defines the set of on-chain interchain accounts parameters.
///  The following parameters may be used to disable the host submodule.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:ibc.applications.interchain_accounts.host.v1.Params)
pub struct Params {
    // message fields
    ///  host_enabled enables or disables the host submodule.
    // @@protoc_insertion_point(field:ibc.applications.interchain_accounts.host.v1.Params.host_enabled)
    pub host_enabled: bool,
    ///  allow_messages defines a list of sdk message typeURLs allowed to be executed on a host chain.
    // @@protoc_insertion_point(field:ibc.applications.interchain_accounts.host.v1.Params.allow_messages)
    pub allow_messages: ::std::vec::Vec<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:ibc.applications.interchain_accounts.host.v1.Params.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Params {
    fn default() -> &'a Params {
        <Params as ::protobuf::Message>::default_instance()
    }
}

impl Params {
    pub fn new() -> Params {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "host_enabled",
            |m: &Params| { &m.host_enabled },
            |m: &mut Params| { &mut m.host_enabled },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "allow_messages",
            |m: &Params| { &m.allow_messages },
            |m: &mut Params| { &mut m.allow_messages },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Params>(
            "Params",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Params {
    const NAME: &'static str = "Params";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.host_enabled = is.read_bool()?;
                },
                18 => {
                    self.allow_messages.push(is.read_string()?);
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
        if self.host_enabled != false {
            my_size += 1 + 1;
        }
        for value in &self.allow_messages {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.host_enabled != false {
            os.write_bool(1, self.host_enabled)?;
        }
        for v in &self.allow_messages {
            os.write_string(2, &v)?;
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

    fn new() -> Params {
        Params::new()
    }

    fn clear(&mut self) {
        self.host_enabled = false;
        self.allow_messages.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Params {
        static instance: Params = Params {
            host_enabled: false,
            allow_messages: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Params {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Params").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Params {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Params {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n7ibc/applications/interchain_accounts/host/v1/host.proto\x12,ibc.appli\
    cations.interchain_accounts.host.v1\x1a\x14gogoproto/gogo.proto\"\x86\
    \x01\n\x06Params\x12:\n\x0chost_enabled\x18\x01\x20\x01(\x08R\x0bhostEna\
    bledB\x17\xf2\xde\x1f\x13yaml:\"host_enabled\"\x12@\n\x0eallow_messages\
    \x18\x02\x20\x03(\tR\rallowMessagesB\x19\xf2\xde\x1f\x15yaml:\"allow_mes\
    sages\"BLZJgithub.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accoun\
    ts/host/typesJ\xb4\x04\n\x06\x12\x04\0\0\x0f\x01\n\x08\n\x01\x0c\x12\x03\
    \0\0\x12\n\x08\n\x01\x02\x12\x03\x02\05\n\x08\n\x01\x08\x12\x03\x04\0a\n\
    \t\n\x02\x08\x0b\x12\x03\x04\0a\n\t\n\x02\x03\0\x12\x03\x06\0\x1e\n\x96\
    \x01\n\x02\x04\0\x12\x04\n\0\x0f\x01\x1a\x89\x01\x20Params\x20defines\
    \x20the\x20set\x20of\x20on-chain\x20interchain\x20accounts\x20parameters\
    .\n\x20The\x20following\x20parameters\x20may\x20be\x20used\x20to\x20disa\
    ble\x20the\x20host\x20submodule.\n\n\n\n\x03\x04\0\x01\x12\x03\n\x08\x0e\
    \nC\n\x04\x04\0\x02\0\x12\x03\x0c\x02I\x1a6\x20host_enabled\x20enables\
    \x20or\x20disables\x20the\x20host\x20submodule.\n\n\x0c\n\x05\x04\0\x02\
    \0\x05\x12\x03\x0c\x02\x06\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0c\x07\
    \x13\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0c\x16\x17\n\x0c\n\x05\x04\0\
    \x02\0\x08\x12\x03\x0c\x18H\n\x0f\n\x08\x04\0\x02\0\x08\xee\xfb\x03\x12\
    \x03\x0c\x19G\nl\n\x04\x04\0\x02\x01\x12\x03\x0e\x02X\x1a_\x20allow_mess\
    ages\x20defines\x20a\x20list\x20of\x20sdk\x20message\x20typeURLs\x20allo\
    wed\x20to\x20be\x20executed\x20on\x20a\x20host\x20chain.\n\n\x0c\n\x05\
    \x04\0\x02\x01\x04\x12\x03\x0e\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\
    \x03\x0e\x0b\x11\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0e\x12\x20\n\x0c\
    \n\x05\x04\0\x02\x01\x03\x12\x03\x0e#$\n\x0c\n\x05\x04\0\x02\x01\x08\x12\
    \x03\x0e%W\n\x0f\n\x08\x04\0\x02\x01\x08\xee\xfb\x03\x12\x03\x0e&Vb\x06p\
    roto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Params::generated_message_descriptor_data());
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

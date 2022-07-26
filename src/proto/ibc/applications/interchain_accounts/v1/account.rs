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

//! Generated file from `ibc/applications/interchain_accounts/v1/account.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

///  An InterchainAccount is defined as a BaseAccount & the address of the account owner on the controller chain
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:ibc.applications.interchain_accounts.v1.InterchainAccount)
pub struct InterchainAccount {
    // message fields
    // @@protoc_insertion_point(field:ibc.applications.interchain_accounts.v1.InterchainAccount.base_account)
    pub base_account: ::protobuf::MessageField<super::auth::BaseAccount>,
    // @@protoc_insertion_point(field:ibc.applications.interchain_accounts.v1.InterchainAccount.account_owner)
    pub account_owner: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:ibc.applications.interchain_accounts.v1.InterchainAccount.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a InterchainAccount {
    fn default() -> &'a InterchainAccount {
        <InterchainAccount as ::protobuf::Message>::default_instance()
    }
}

impl InterchainAccount {
    pub fn new() -> InterchainAccount {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::auth::BaseAccount>(
            "base_account",
            |m: &InterchainAccount| { &m.base_account },
            |m: &mut InterchainAccount| { &mut m.base_account },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "account_owner",
            |m: &InterchainAccount| { &m.account_owner },
            |m: &mut InterchainAccount| { &mut m.account_owner },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<InterchainAccount>(
            "InterchainAccount",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for InterchainAccount {
    const NAME: &'static str = "InterchainAccount";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.base_account)?;
                },
                18 => {
                    self.account_owner = is.read_string()?;
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
        if let Some(v) = self.base_account.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.account_owner.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.account_owner);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.base_account.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.account_owner.is_empty() {
            os.write_string(2, &self.account_owner)?;
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

    fn new() -> InterchainAccount {
        InterchainAccount::new()
    }

    fn clear(&mut self) {
        self.base_account.clear();
        self.account_owner.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static InterchainAccount {
        static instance: InterchainAccount = InterchainAccount {
            base_account: ::protobuf::MessageField::none(),
            account_owner: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for InterchainAccount {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("InterchainAccount").unwrap()).clone()
    }
}

impl ::std::fmt::Display for InterchainAccount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InterchainAccount {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n5ibc/applications/interchain_accounts/v1/account.proto\x12'ibc.applica\
    tions.interchain_accounts.v1\x1a\x19cosmos_proto/cosmos.proto\x1a\x14gog\
    oproto/gogo.proto\x1a\x1ecosmos/auth/v1beta1/auth.proto\"\xd4\x01\n\x11I\
    nterchainAccount\x12`\n\x0cbase_account\x18\x01\x20\x01(\x0b2\x20.cosmos\
    .auth.v1beta1.BaseAccountR\x0bbaseAccountB\x1b\xf2\xde\x1f\x13yaml:\"bas\
    e_account\"\xd0\xde\x1f\x01\x12=\n\raccount_owner\x18\x02\x20\x01(\tR\
    \x0caccountOwnerB\x18\xf2\xde\x1f\x14yaml:\"account_owner\":\x1e\xd2\xb4\
    -\x12InterchainAccountI\x88\xa0\x1f\0\x98\xa0\x1f\0BGZEgithub.com/cosmos\
    /ibc-go/v3/modules/apps/27-interchain-accounts/typesJ\xe8\x03\n\x06\x12\
    \x04\0\0\x13\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\
    \x02\00\n\x08\n\x01\x08\x12\x03\x04\0\\\n\t\n\x02\x08\x0b\x12\x03\x04\0\
    \\\n\t\n\x02\x03\0\x12\x03\x06\0#\n\t\n\x02\x03\x01\x12\x03\x07\0\x1e\n\
    \t\n\x02\x03\x02\x12\x03\x08\0(\ny\n\x02\x04\0\x12\x04\x0b\0\x13\x01\x1a\
    m\x20An\x20InterchainAccount\x20is\x20defined\x20as\x20a\x20BaseAccount\
    \x20&\x20the\x20address\x20of\x20the\x20account\x20owner\x20on\x20the\
    \x20controller\x20chain\n\n\n\n\x03\x04\0\x01\x12\x03\x0b\x08\x19\n\n\n\
    \x03\x04\0\x07\x12\x03\x0c\x025\n\r\n\x06\x04\0\x07\x81\xf4\x03\x12\x03\
    \x0c\x025\n\n\n\x03\x04\0\x07\x12\x03\r\x025\n\r\n\x06\x04\0\x07\x83\xf4\
    \x03\x12\x03\r\x025\n\n\n\x03\x04\0\x07\x12\x03\x0e\x02D\n\r\n\x06\x04\0\
    \x07\xca\xd6\x05\x12\x03\x0e\x02D\n\x0c\n\x04\x04\0\x02\0\x12\x04\x10\
    \x02\x11Q\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x10\x02!\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\x10\".\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x1012\n\
    \x0c\n\x05\x04\0\x02\0\x08\x12\x03\x11\x06P\n\x0f\n\x08\x04\0\x02\0\x08\
    \xea\xfb\x03\x12\x03\x11\x07\x1f\n\x0f\n\x08\x04\0\x02\0\x08\xee\xfb\x03\
    \x12\x03\x11!O\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x12\x02M\n\x0c\n\x05\
    \x04\0\x02\x01\x05\x12\x03\x12\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03\x12\t\x16\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x12\x19\x1a\n\x0c\n\
    \x05\x04\0\x02\x01\x08\x12\x03\x12\x1bL\n\x0f\n\x08\x04\0\x02\x01\x08\
    \xee\xfb\x03\x12\x03\x12\x1cKb\x06proto3\
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
            deps.push(super::cosmos::file_descriptor().clone());
            deps.push(super::gogo::file_descriptor().clone());
            deps.push(super::auth::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(InterchainAccount::generated_message_descriptor_data());
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

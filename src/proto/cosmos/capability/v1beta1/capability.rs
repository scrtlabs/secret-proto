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

//! Generated file from `cosmos/capability/v1beta1/capability.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

///  Capability defines an implementation of an object capability. The index
///  provided to a Capability must be globally unique.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.capability.v1beta1.Capability)
pub struct Capability {
    // message fields
    // @@protoc_insertion_point(field:cosmos.capability.v1beta1.Capability.index)
    pub index: u64,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.capability.v1beta1.Capability.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Capability {
    fn default() -> &'a Capability {
        <Capability as ::protobuf::Message>::default_instance()
    }
}

impl Capability {
    pub fn new() -> Capability {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "index",
            |m: &Capability| { &m.index },
            |m: &mut Capability| { &mut m.index },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Capability>(
            "Capability",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Capability {
    const NAME: &'static str = "Capability";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.index = is.read_uint64()?;
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
        if self.index != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.index);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.index != 0 {
            os.write_uint64(1, self.index)?;
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

    fn new() -> Capability {
        Capability::new()
    }

    fn clear(&mut self) {
        self.index = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Capability {
        static instance: Capability = Capability {
            index: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Capability {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Capability").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Capability {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Capability {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  Owner defines a single capability owner. An owner is defined by the name of
///  capability and the module name.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.capability.v1beta1.Owner)
pub struct Owner {
    // message fields
    // @@protoc_insertion_point(field:cosmos.capability.v1beta1.Owner.module)
    pub module: ::std::string::String,
    // @@protoc_insertion_point(field:cosmos.capability.v1beta1.Owner.name)
    pub name: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.capability.v1beta1.Owner.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Owner {
    fn default() -> &'a Owner {
        <Owner as ::protobuf::Message>::default_instance()
    }
}

impl Owner {
    pub fn new() -> Owner {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "module",
            |m: &Owner| { &m.module },
            |m: &mut Owner| { &mut m.module },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &Owner| { &m.name },
            |m: &mut Owner| { &mut m.name },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Owner>(
            "Owner",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Owner {
    const NAME: &'static str = "Owner";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.module = is.read_string()?;
                },
                18 => {
                    self.name = is.read_string()?;
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
        if !self.module.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.module);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.module.is_empty() {
            os.write_string(1, &self.module)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
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

    fn new() -> Owner {
        Owner::new()
    }

    fn clear(&mut self) {
        self.module.clear();
        self.name.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Owner {
        static instance: Owner = Owner {
            module: ::std::string::String::new(),
            name: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Owner {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Owner").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Owner {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Owner {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  CapabilityOwners defines a set of owners of a single Capability. The set of
///  owners must be unique.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.capability.v1beta1.CapabilityOwners)
pub struct CapabilityOwners {
    // message fields
    // @@protoc_insertion_point(field:cosmos.capability.v1beta1.CapabilityOwners.owners)
    pub owners: ::std::vec::Vec<Owner>,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.capability.v1beta1.CapabilityOwners.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CapabilityOwners {
    fn default() -> &'a CapabilityOwners {
        <CapabilityOwners as ::protobuf::Message>::default_instance()
    }
}

impl CapabilityOwners {
    pub fn new() -> CapabilityOwners {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "owners",
            |m: &CapabilityOwners| { &m.owners },
            |m: &mut CapabilityOwners| { &mut m.owners },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CapabilityOwners>(
            "CapabilityOwners",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CapabilityOwners {
    const NAME: &'static str = "CapabilityOwners";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.owners.push(is.read_message()?);
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
        for value in &self.owners {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.owners {
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

    fn new() -> CapabilityOwners {
        CapabilityOwners::new()
    }

    fn clear(&mut self) {
        self.owners.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CapabilityOwners {
        static instance: CapabilityOwners = CapabilityOwners {
            owners: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CapabilityOwners {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CapabilityOwners").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CapabilityOwners {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CapabilityOwners {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n*cosmos/capability/v1beta1/capability.proto\x12\x19cosmos.capability.v\
    1beta1\x1a\x14gogoproto/gogo.proto\":\n\nCapability\x12&\n\x05index\x18\
    \x01\x20\x01(\x04R\x05indexB\x10\xf2\xde\x1f\x0cyaml:\"index\":\x04\x98\
    \xa0\x1f\0\"a\n\x05Owner\x12)\n\x06module\x18\x01\x20\x01(\tR\x06moduleB\
    \x11\xf2\xde\x1f\ryaml:\"module\"\x12#\n\x04name\x18\x02\x20\x01(\tR\x04\
    nameB\x0f\xf2\xde\x1f\x0byaml:\"name\":\x08\x88\xa0\x1f\0\x98\xa0\x1f\0\
    \"R\n\x10CapabilityOwners\x12>\n\x06owners\x18\x01\x20\x03(\x0b2\x20.cos\
    mos.capability.v1beta1.OwnerR\x06ownersB\x04\xc8\xde\x1f\0B1Z/github.com\
    /cosmos/cosmos-sdk/x/capability/typesJ\x91\x07\n\x06\x12\x04\0\0\x1d\x01\
    \n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\0\"\n\x08\n\
    \x01\x08\x12\x03\x03\0F\n\t\n\x02\x08\x0b\x12\x03\x03\0F\n\t\n\x02\x03\0\
    \x12\x03\x05\0\x1e\n\x88\x01\n\x02\x04\0\x12\x04\t\0\r\x01\x1a|\x20Capab\
    ility\x20defines\x20an\x20implementation\x20of\x20an\x20object\x20capabi\
    lity.\x20The\x20index\n\x20provided\x20to\x20a\x20Capability\x20must\x20\
    be\x20globally\x20unique.\n\n\n\n\x03\x04\0\x01\x12\x03\t\x08\x12\n\n\n\
    \x03\x04\0\x07\x12\x03\n\x02.\n\r\n\x06\x04\0\x07\x83\xf4\x03\x12\x03\n\
    \x02.\n\x0b\n\x04\x04\0\x02\0\x12\x03\x0c\x02=\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x0c\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0c\t\x0e\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0c\x11\x12\n\x0c\n\x05\x04\0\x02\0\
    \x08\x12\x03\x0c\x13<\n\x0f\n\x08\x04\0\x02\0\x08\xee\xfb\x03\x12\x03\
    \x0c\x14;\nz\n\x02\x04\x01\x12\x04\x11\0\x17\x01\x1an\x20Owner\x20define\
    s\x20a\x20single\x20capability\x20owner.\x20An\x20owner\x20is\x20defined\
    \x20by\x20the\x20name\x20of\n\x20capability\x20and\x20the\x20module\x20n\
    ame.\n\n\n\n\x03\x04\x01\x01\x12\x03\x11\x08\r\n\n\n\x03\x04\x01\x07\x12\
    \x03\x12\x02.\n\r\n\x06\x04\x01\x07\x83\xf4\x03\x12\x03\x12\x02.\n\n\n\
    \x03\x04\x01\x07\x12\x03\x13\x02.\n\r\n\x06\x04\x01\x07\x81\xf4\x03\x12\
    \x03\x13\x02.\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x15\x02?\n\x0c\n\x05\x04\
    \x01\x02\0\x05\x12\x03\x15\x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\
    \x15\t\x0f\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x15\x12\x13\n\x0c\n\x05\
    \x04\x01\x02\0\x08\x12\x03\x15\x14>\n\x0f\n\x08\x04\x01\x02\0\x08\xee\
    \xfb\x03\x12\x03\x15\x15=\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x16\x02=\n\
    \x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x16\x02\x08\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03\x16\t\r\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x16\
    \x12\x13\n\x0c\n\x05\x04\x01\x02\x01\x08\x12\x03\x16\x14<\n\x0f\n\x08\
    \x04\x01\x02\x01\x08\xee\xfb\x03\x12\x03\x16\x15;\nq\n\x02\x04\x02\x12\
    \x04\x1b\0\x1d\x01\x1ae\x20CapabilityOwners\x20defines\x20a\x20set\x20of\
    \x20owners\x20of\x20a\x20single\x20Capability.\x20The\x20set\x20of\n\x20\
    owners\x20must\x20be\x20unique.\n\n\n\n\x03\x04\x02\x01\x12\x03\x1b\x08\
    \x18\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x1c\x02;\n\x0c\n\x05\x04\x02\x02\
    \0\x04\x12\x03\x1c\x02\n\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x1c\x0b\
    \x10\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x1c\x11\x17\n\x0c\n\x05\x04\
    \x02\x02\0\x03\x12\x03\x1c\x1a\x1b\n\x0c\n\x05\x04\x02\x02\0\x08\x12\x03\
    \x1c\x1c:\n\x0f\n\x08\x04\x02\x02\0\x08\xe9\xfb\x03\x12\x03\x1c\x1d9b\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::gogo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(Capability::generated_message_descriptor_data());
            messages.push(Owner::generated_message_descriptor_data());
            messages.push(CapabilityOwners::generated_message_descriptor_data());
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

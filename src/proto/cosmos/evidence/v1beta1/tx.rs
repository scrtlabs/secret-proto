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

//! Generated file from `cosmos/evidence/v1beta1/tx.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

///  MsgSubmitEvidence represents a message that supports submitting arbitrary
///  Evidence of misbehavior such as equivocation or counterfactual signing.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.evidence.v1beta1.MsgSubmitEvidence)
pub struct MsgSubmitEvidence {
    // message fields
    // @@protoc_insertion_point(field:cosmos.evidence.v1beta1.MsgSubmitEvidence.submitter)
    pub submitter: ::std::string::String,
    // @@protoc_insertion_point(field:cosmos.evidence.v1beta1.MsgSubmitEvidence.evidence)
    pub evidence: ::protobuf::MessageField<::protobuf::well_known_types::any::Any>,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.evidence.v1beta1.MsgSubmitEvidence.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgSubmitEvidence {
    fn default() -> &'a MsgSubmitEvidence {
        <MsgSubmitEvidence as ::protobuf::Message>::default_instance()
    }
}

impl MsgSubmitEvidence {
    pub fn new() -> MsgSubmitEvidence {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "submitter",
            |m: &MsgSubmitEvidence| { &m.submitter },
            |m: &mut MsgSubmitEvidence| { &mut m.submitter },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::any::Any>(
            "evidence",
            |m: &MsgSubmitEvidence| { &m.evidence },
            |m: &mut MsgSubmitEvidence| { &mut m.evidence },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgSubmitEvidence>(
            "MsgSubmitEvidence",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgSubmitEvidence {
    const NAME: &'static str = "MsgSubmitEvidence";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.submitter = is.read_string()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.evidence)?;
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
        if !self.submitter.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.submitter);
        }
        if let Some(v) = self.evidence.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.submitter.is_empty() {
            os.write_string(1, &self.submitter)?;
        }
        if let Some(v) = self.evidence.as_ref() {
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

    fn new() -> MsgSubmitEvidence {
        MsgSubmitEvidence::new()
    }

    fn clear(&mut self) {
        self.submitter.clear();
        self.evidence.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgSubmitEvidence {
        static instance: MsgSubmitEvidence = MsgSubmitEvidence {
            submitter: ::std::string::String::new(),
            evidence: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgSubmitEvidence {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgSubmitEvidence").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgSubmitEvidence {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgSubmitEvidence {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  MsgSubmitEvidenceResponse defines the Msg/SubmitEvidence response type.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.evidence.v1beta1.MsgSubmitEvidenceResponse)
pub struct MsgSubmitEvidenceResponse {
    // message fields
    ///  hash defines the hash of the evidence.
    // @@protoc_insertion_point(field:cosmos.evidence.v1beta1.MsgSubmitEvidenceResponse.hash)
    pub hash: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.evidence.v1beta1.MsgSubmitEvidenceResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgSubmitEvidenceResponse {
    fn default() -> &'a MsgSubmitEvidenceResponse {
        <MsgSubmitEvidenceResponse as ::protobuf::Message>::default_instance()
    }
}

impl MsgSubmitEvidenceResponse {
    pub fn new() -> MsgSubmitEvidenceResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hash",
            |m: &MsgSubmitEvidenceResponse| { &m.hash },
            |m: &mut MsgSubmitEvidenceResponse| { &mut m.hash },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgSubmitEvidenceResponse>(
            "MsgSubmitEvidenceResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgSubmitEvidenceResponse {
    const NAME: &'static str = "MsgSubmitEvidenceResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                34 => {
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
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.hash.is_empty() {
            os.write_bytes(4, &self.hash)?;
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

    fn new() -> MsgSubmitEvidenceResponse {
        MsgSubmitEvidenceResponse::new()
    }

    fn clear(&mut self) {
        self.hash.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgSubmitEvidenceResponse {
        static instance: MsgSubmitEvidenceResponse = MsgSubmitEvidenceResponse {
            hash: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgSubmitEvidenceResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgSubmitEvidenceResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgSubmitEvidenceResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgSubmitEvidenceResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20cosmos/evidence/v1beta1/tx.proto\x12\x17cosmos.evidence.v1beta1\
    \x1a\x14gogoproto/gogo.proto\x1a\x19google/protobuf/any.proto\x1a\x19cos\
    mos_proto/cosmos.proto\"{\n\x11MsgSubmitEvidence\x12\x1c\n\tsubmitter\
    \x18\x01\x20\x01(\tR\tsubmitter\x12>\n\x08evidence\x18\x02\x20\x01(\x0b2\
    \x14.google.protobuf.AnyR\x08evidenceB\x0c\xca\xb4-\x08Evidence:\x08\x88\
    \xa0\x1f\0\xe8\xa0\x1f\0\"/\n\x19MsgSubmitEvidenceResponse\x12\x12\n\x04\
    hash\x18\x04\x20\x01(\x0cR\x04hash2w\n\x03Msg\x12p\n\x0eSubmitEvidence\
    \x12*.cosmos.evidence.v1beta1.MsgSubmitEvidence\x1a2.cosmos.evidence.v1b\
    eta1.MsgSubmitEvidenceResponseB3Z-github.com/cosmos/cosmos-sdk/x/evidenc\
    e/types\xa8\xe2\x1e\x01J\x88\x07\n\x06\x12\x04\0\0\x1f\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\0\x20\n\x08\n\x01\x08\
    \x12\x03\x03\0O\n\t\n\x02\x08\x0b\x12\x03\x03\0O\n\x08\n\x01\x08\x12\x03\
    \x04\0$\n\x0b\n\x04\x08\xa5\xec\x03\x12\x03\x04\0$\n\t\n\x02\x03\0\x12\
    \x03\x06\0\x1e\n\t\n\x02\x03\x01\x12\x03\x07\0#\n\t\n\x02\x03\x02\x12\
    \x03\x08\0#\n3\n\x02\x06\0\x12\x04\x0b\0\x0f\x01\x1a'\x20Msg\x20defines\
    \x20the\x20evidence\x20Msg\x20service.\n\n\n\n\x03\x06\0\x01\x12\x03\x0b\
    \x08\x0b\n{\n\x04\x06\0\x02\0\x12\x03\x0e\x02L\x1an\x20SubmitEvidence\
    \x20submits\x20an\x20arbitrary\x20Evidence\x20of\x20misbehavior\x20such\
    \x20as\x20equivocation\x20or\n\x20counterfactual\x20signing.\n\n\x0c\n\
    \x05\x06\0\x02\0\x01\x12\x03\x0e\x06\x14\n\x0c\n\x05\x06\0\x02\0\x02\x12\
    \x03\x0e\x15&\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x0e1J\n\xa1\x01\n\x02\
    \x04\0\x12\x04\x13\0\x19\x01\x1a\x94\x01\x20MsgSubmitEvidence\x20represe\
    nts\x20a\x20message\x20that\x20supports\x20submitting\x20arbitrary\n\x20\
    Evidence\x20of\x20misbehavior\x20such\x20as\x20equivocation\x20or\x20cou\
    nterfactual\x20signing.\n\n\n\n\x03\x04\0\x01\x12\x03\x13\x08\x19\n\n\n\
    \x03\x04\0\x07\x12\x03\x14\x02-\n\r\n\x06\x04\0\x07\x8d\xf4\x03\x12\x03\
    \x14\x02-\n\n\n\x03\x04\0\x07\x12\x03\x15\x02-\n\r\n\x06\x04\0\x07\x81\
    \xf4\x03\x12\x03\x15\x02-\n\x0b\n\x04\x04\0\x02\0\x12\x03\x17\x02$\n\x0c\
    \n\x05\x04\0\x02\0\x05\x12\x03\x17\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03\x17\x16\x1f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x17\"#\n\x0b\n\
    \x04\x04\0\x02\x01\x12\x03\x18\x02T\n\x0c\n\x05\x04\0\x02\x01\x06\x12\
    \x03\x18\x02\x15\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x18\x16\x1e\n\x0c\
    \n\x05\x04\0\x02\x01\x03\x12\x03\x18\"#\n\x0c\n\x05\x04\0\x02\x01\x08\
    \x12\x03\x18$S\n\x0f\n\x08\x04\0\x02\x01\x08\xc9\xd6\x05\x12\x03\x18%R\n\
    U\n\x02\x04\x01\x12\x04\x1c\0\x1f\x01\x1aI\x20MsgSubmitEvidenceResponse\
    \x20defines\x20the\x20Msg/SubmitEvidence\x20response\x20type.\n\n\n\n\
    \x03\x04\x01\x01\x12\x03\x1c\x08!\n5\n\x04\x04\x01\x02\0\x12\x03\x1e\x02\
    \x11\x1a(\x20hash\x20defines\x20the\x20hash\x20of\x20the\x20evidence.\n\
    \n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x1e\x02\x07\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03\x1e\x08\x0c\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x1e\
    \x0f\x10b\x06proto3\
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
            deps.push(::protobuf::well_known_types::any::file_descriptor().clone());
            deps.push(super::cosmos::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(MsgSubmitEvidence::generated_message_descriptor_data());
            messages.push(MsgSubmitEvidenceResponse::generated_message_descriptor_data());
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

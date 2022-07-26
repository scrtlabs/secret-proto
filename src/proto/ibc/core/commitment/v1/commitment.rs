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

//! Generated file from `ibc/core/commitment/v1/commitment.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

///  MerkleRoot defines a merkle root hash.
///  In the Cosmos SDK, the AppHash of a block header becomes the root.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:ibc.core.commitment.v1.MerkleRoot)
pub struct MerkleRoot {
    // message fields
    // @@protoc_insertion_point(field:ibc.core.commitment.v1.MerkleRoot.hash)
    pub hash: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:ibc.core.commitment.v1.MerkleRoot.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MerkleRoot {
    fn default() -> &'a MerkleRoot {
        <MerkleRoot as ::protobuf::Message>::default_instance()
    }
}

impl MerkleRoot {
    pub fn new() -> MerkleRoot {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hash",
            |m: &MerkleRoot| { &m.hash },
            |m: &mut MerkleRoot| { &mut m.hash },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MerkleRoot>(
            "MerkleRoot",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MerkleRoot {
    const NAME: &'static str = "MerkleRoot";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
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
            my_size += ::protobuf::rt::bytes_size(1, &self.hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.hash.is_empty() {
            os.write_bytes(1, &self.hash)?;
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

    fn new() -> MerkleRoot {
        MerkleRoot::new()
    }

    fn clear(&mut self) {
        self.hash.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MerkleRoot {
        static instance: MerkleRoot = MerkleRoot {
            hash: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MerkleRoot {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MerkleRoot").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MerkleRoot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MerkleRoot {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  MerklePrefix is merkle path prefixed to the key.
///  The constructed key from the Path and the key will be append(Path.KeyPath,
///  append(Path.KeyPrefix, key...))
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:ibc.core.commitment.v1.MerklePrefix)
pub struct MerklePrefix {
    // message fields
    // @@protoc_insertion_point(field:ibc.core.commitment.v1.MerklePrefix.key_prefix)
    pub key_prefix: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:ibc.core.commitment.v1.MerklePrefix.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MerklePrefix {
    fn default() -> &'a MerklePrefix {
        <MerklePrefix as ::protobuf::Message>::default_instance()
    }
}

impl MerklePrefix {
    pub fn new() -> MerklePrefix {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "key_prefix",
            |m: &MerklePrefix| { &m.key_prefix },
            |m: &mut MerklePrefix| { &mut m.key_prefix },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MerklePrefix>(
            "MerklePrefix",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MerklePrefix {
    const NAME: &'static str = "MerklePrefix";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.key_prefix = is.read_bytes()?;
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
        if !self.key_prefix.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key_prefix);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.key_prefix.is_empty() {
            os.write_bytes(1, &self.key_prefix)?;
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

    fn new() -> MerklePrefix {
        MerklePrefix::new()
    }

    fn clear(&mut self) {
        self.key_prefix.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MerklePrefix {
        static instance: MerklePrefix = MerklePrefix {
            key_prefix: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MerklePrefix {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MerklePrefix").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MerklePrefix {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MerklePrefix {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  MerklePath is the path used to verify commitment proofs, which can be an
///  arbitrary structured object (defined by a commitment type).
///  MerklePath is represented from root-to-leaf
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:ibc.core.commitment.v1.MerklePath)
pub struct MerklePath {
    // message fields
    // @@protoc_insertion_point(field:ibc.core.commitment.v1.MerklePath.key_path)
    pub key_path: ::std::vec::Vec<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:ibc.core.commitment.v1.MerklePath.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MerklePath {
    fn default() -> &'a MerklePath {
        <MerklePath as ::protobuf::Message>::default_instance()
    }
}

impl MerklePath {
    pub fn new() -> MerklePath {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "key_path",
            |m: &MerklePath| { &m.key_path },
            |m: &mut MerklePath| { &mut m.key_path },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MerklePath>(
            "MerklePath",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MerklePath {
    const NAME: &'static str = "MerklePath";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.key_path.push(is.read_string()?);
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
        for value in &self.key_path {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.key_path {
            os.write_string(1, &v)?;
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

    fn new() -> MerklePath {
        MerklePath::new()
    }

    fn clear(&mut self) {
        self.key_path.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MerklePath {
        static instance: MerklePath = MerklePath {
            key_path: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MerklePath {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MerklePath").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MerklePath {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MerklePath {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  MerkleProof is a wrapper type over a chain of CommitmentProofs.
///  It demonstrates membership or non-membership for an element or set of
///  elements, verifiable in conjunction with a known commitment root. Proofs
///  should be succinct.
///  MerkleProofs are ordered from leaf-to-root
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:ibc.core.commitment.v1.MerkleProof)
pub struct MerkleProof {
    // message fields
    // @@protoc_insertion_point(field:ibc.core.commitment.v1.MerkleProof.proofs)
    pub proofs: ::std::vec::Vec<super::proofs::CommitmentProof>,
    // special fields
    // @@protoc_insertion_point(special_field:ibc.core.commitment.v1.MerkleProof.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MerkleProof {
    fn default() -> &'a MerkleProof {
        <MerkleProof as ::protobuf::Message>::default_instance()
    }
}

impl MerkleProof {
    pub fn new() -> MerkleProof {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "proofs",
            |m: &MerkleProof| { &m.proofs },
            |m: &mut MerkleProof| { &mut m.proofs },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MerkleProof>(
            "MerkleProof",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MerkleProof {
    const NAME: &'static str = "MerkleProof";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.proofs.push(is.read_message()?);
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
        for value in &self.proofs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.proofs {
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

    fn new() -> MerkleProof {
        MerkleProof::new()
    }

    fn clear(&mut self) {
        self.proofs.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MerkleProof {
        static instance: MerkleProof = MerkleProof {
            proofs: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MerkleProof {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MerkleProof").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MerkleProof {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MerkleProof {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'ibc/core/commitment/v1/commitment.proto\x12\x16ibc.core.commitment.v1\
    \x1a\x14gogoproto/gogo.proto\x1a\x13confio/proofs.proto\"&\n\nMerkleRoot\
    \x12\x12\n\x04hash\x18\x01\x20\x01(\x0cR\x04hash:\x04\x88\xa0\x1f\0\"D\n\
    \x0cMerklePrefix\x124\n\nkey_prefix\x18\x01\x20\x01(\x0cR\tkeyPrefixB\
    \x15\xf2\xde\x1f\x11yaml:\"key_prefix\"\"B\n\nMerklePath\x12.\n\x08key_p\
    ath\x18\x01\x20\x03(\tR\x07keyPathB\x13\xf2\xde\x1f\x0fyaml:\"key_path\"\
    :\x04\x98\xa0\x1f\0\"=\n\x0bMerkleProof\x12.\n\x06proofs\x18\x01\x20\x03\
    (\x0b2\x16.ics23.CommitmentProofR\x06proofsB>Z<github.com/cosmos/ibc-go/\
    v3/modules/core/23-commitment/typesJ\xf3\t\n\x06\x12\x04\0\0(\x01\n\x08\
    \n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\0\x1f\n\x08\n\x01\
    \x08\x12\x03\x04\0S\n\t\n\x02\x08\x0b\x12\x03\x04\0S\n\t\n\x02\x03\0\x12\
    \x03\x06\0\x1e\n\t\n\x02\x03\x01\x12\x03\x07\0\x1d\nx\n\x02\x04\0\x12\
    \x04\x0b\0\x0f\x01\x1al\x20MerkleRoot\x20defines\x20a\x20merkle\x20root\
    \x20hash.\n\x20In\x20the\x20Cosmos\x20SDK,\x20the\x20AppHash\x20of\x20a\
    \x20block\x20header\x20becomes\x20the\x20root.\n\n\n\n\x03\x04\0\x01\x12\
    \x03\x0b\x08\x12\n\n\n\x03\x04\0\x07\x12\x03\x0c\x02-\n\r\n\x06\x04\0\
    \x07\x81\xf4\x03\x12\x03\x0c\x02-\n\x0b\n\x04\x04\0\x02\0\x12\x03\x0e\
    \x02\x11\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x0e\x02\x07\n\x0c\n\x05\x04\
    \0\x02\0\x01\x12\x03\x0e\x08\x0c\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0e\
    \x0f\x10\n\xac\x01\n\x02\x04\x01\x12\x04\x14\0\x16\x01\x1a\x9f\x01\x20Me\
    rklePrefix\x20is\x20merkle\x20path\x20prefixed\x20to\x20the\x20key.\n\
    \x20The\x20constructed\x20key\x20from\x20the\x20Path\x20and\x20the\x20ke\
    y\x20will\x20be\x20append(Path.KeyPath,\n\x20append(Path.KeyPrefix,\x20k\
    ey...))\n\n\n\n\x03\x04\x01\x01\x12\x03\x14\x08\x14\n\x0b\n\x04\x04\x01\
    \x02\0\x12\x03\x15\x02F\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x15\x02\
    \x07\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x15\x08\x12\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\x15\x15\x16\n\x0c\n\x05\x04\x01\x02\0\x08\x12\x03\
    \x15\x17E\n\x0f\n\x08\x04\x01\x02\0\x08\xee\xfb\x03\x12\x03\x15\x18D\n\
    \xc1\x01\n\x02\x04\x02\x12\x04\x1b\0\x1f\x01\x1a\xb4\x01\x20MerklePath\
    \x20is\x20the\x20path\x20used\x20to\x20verify\x20commitment\x20proofs,\
    \x20which\x20can\x20be\x20an\n\x20arbitrary\x20structured\x20object\x20(\
    defined\x20by\x20a\x20commitment\x20type).\n\x20MerklePath\x20is\x20repr\
    esented\x20from\x20root-to-leaf\n\n\n\n\x03\x04\x02\x01\x12\x03\x1b\x08\
    \x12\n\n\n\x03\x04\x02\x07\x12\x03\x1c\x02.\n\r\n\x06\x04\x02\x07\x83\
    \xf4\x03\x12\x03\x1c\x02.\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x1e\x02L\n\
    \x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x1e\x02\n\n\x0c\n\x05\x04\x02\x02\0\
    \x05\x12\x03\x1e\x0b\x11\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x1e\x12\
    \x1a\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x1e\x1d\x1e\n\x0c\n\x05\x04\
    \x02\x02\0\x08\x12\x03\x1e\x1fK\n\x0f\n\x08\x04\x02\x02\0\x08\xee\xfb\
    \x03\x12\x03\x1e\x20J\n\xa0\x02\n\x02\x04\x03\x12\x04&\0(\x01\x1a\x93\
    \x02\x20MerkleProof\x20is\x20a\x20wrapper\x20type\x20over\x20a\x20chain\
    \x20of\x20CommitmentProofs.\n\x20It\x20demonstrates\x20membership\x20or\
    \x20non-membership\x20for\x20an\x20element\x20or\x20set\x20of\n\x20eleme\
    nts,\x20verifiable\x20in\x20conjunction\x20with\x20a\x20known\x20commitm\
    ent\x20root.\x20Proofs\n\x20should\x20be\x20succinct.\n\x20MerkleProofs\
    \x20are\x20ordered\x20from\x20leaf-to-root\n\n\n\n\x03\x04\x03\x01\x12\
    \x03&\x08\x13\n\x0b\n\x04\x04\x03\x02\0\x12\x03'\x02,\n\x0c\n\x05\x04\
    \x03\x02\0\x04\x12\x03'\x02\n\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03'\x0b\
    \x20\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03'!'\n\x0c\n\x05\x04\x03\x02\0\
    \x03\x12\x03'*+b\x06proto3\
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
            deps.push(super::proofs::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(MerkleRoot::generated_message_descriptor_data());
            messages.push(MerklePrefix::generated_message_descriptor_data());
            messages.push(MerklePath::generated_message_descriptor_data());
            messages.push(MerkleProof::generated_message_descriptor_data());
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

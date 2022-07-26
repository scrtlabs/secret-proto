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

//! Generated file from `tendermint/types/validator.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:tendermint.types.ValidatorSet)
pub struct ValidatorSet {
    // message fields
    // @@protoc_insertion_point(field:tendermint.types.ValidatorSet.validators)
    pub validators: ::std::vec::Vec<Validator>,
    // @@protoc_insertion_point(field:tendermint.types.ValidatorSet.proposer)
    pub proposer: ::protobuf::MessageField<Validator>,
    // @@protoc_insertion_point(field:tendermint.types.ValidatorSet.total_voting_power)
    pub total_voting_power: i64,
    // special fields
    // @@protoc_insertion_point(special_field:tendermint.types.ValidatorSet.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ValidatorSet {
    fn default() -> &'a ValidatorSet {
        <ValidatorSet as ::protobuf::Message>::default_instance()
    }
}

impl ValidatorSet {
    pub fn new() -> ValidatorSet {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "validators",
            |m: &ValidatorSet| { &m.validators },
            |m: &mut ValidatorSet| { &mut m.validators },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Validator>(
            "proposer",
            |m: &ValidatorSet| { &m.proposer },
            |m: &mut ValidatorSet| { &mut m.proposer },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "total_voting_power",
            |m: &ValidatorSet| { &m.total_voting_power },
            |m: &mut ValidatorSet| { &mut m.total_voting_power },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ValidatorSet>(
            "ValidatorSet",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ValidatorSet {
    const NAME: &'static str = "ValidatorSet";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.validators.push(is.read_message()?);
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.proposer)?;
                },
                24 => {
                    self.total_voting_power = is.read_int64()?;
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
        for value in &self.validators {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.proposer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.total_voting_power != 0 {
            my_size += ::protobuf::rt::int64_size(3, self.total_voting_power);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.validators {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if let Some(v) = self.proposer.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.total_voting_power != 0 {
            os.write_int64(3, self.total_voting_power)?;
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

    fn new() -> ValidatorSet {
        ValidatorSet::new()
    }

    fn clear(&mut self) {
        self.validators.clear();
        self.proposer.clear();
        self.total_voting_power = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ValidatorSet {
        static instance: ValidatorSet = ValidatorSet {
            validators: ::std::vec::Vec::new(),
            proposer: ::protobuf::MessageField::none(),
            total_voting_power: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ValidatorSet {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ValidatorSet").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ValidatorSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ValidatorSet {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:tendermint.types.Validator)
pub struct Validator {
    // message fields
    // @@protoc_insertion_point(field:tendermint.types.Validator.address)
    pub address: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:tendermint.types.Validator.pub_key)
    pub pub_key: ::protobuf::MessageField<super::keys::PublicKey>,
    // @@protoc_insertion_point(field:tendermint.types.Validator.voting_power)
    pub voting_power: i64,
    // @@protoc_insertion_point(field:tendermint.types.Validator.proposer_priority)
    pub proposer_priority: i64,
    // special fields
    // @@protoc_insertion_point(special_field:tendermint.types.Validator.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Validator {
    fn default() -> &'a Validator {
        <Validator as ::protobuf::Message>::default_instance()
    }
}

impl Validator {
    pub fn new() -> Validator {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "address",
            |m: &Validator| { &m.address },
            |m: &mut Validator| { &mut m.address },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::keys::PublicKey>(
            "pub_key",
            |m: &Validator| { &m.pub_key },
            |m: &mut Validator| { &mut m.pub_key },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "voting_power",
            |m: &Validator| { &m.voting_power },
            |m: &mut Validator| { &mut m.voting_power },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "proposer_priority",
            |m: &Validator| { &m.proposer_priority },
            |m: &mut Validator| { &mut m.proposer_priority },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Validator>(
            "Validator",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Validator {
    const NAME: &'static str = "Validator";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.address = is.read_bytes()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pub_key)?;
                },
                24 => {
                    self.voting_power = is.read_int64()?;
                },
                32 => {
                    self.proposer_priority = is.read_int64()?;
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
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.address);
        }
        if let Some(v) = self.pub_key.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.voting_power != 0 {
            my_size += ::protobuf::rt::int64_size(3, self.voting_power);
        }
        if self.proposer_priority != 0 {
            my_size += ::protobuf::rt::int64_size(4, self.proposer_priority);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.address.is_empty() {
            os.write_bytes(1, &self.address)?;
        }
        if let Some(v) = self.pub_key.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.voting_power != 0 {
            os.write_int64(3, self.voting_power)?;
        }
        if self.proposer_priority != 0 {
            os.write_int64(4, self.proposer_priority)?;
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

    fn new() -> Validator {
        Validator::new()
    }

    fn clear(&mut self) {
        self.address.clear();
        self.pub_key.clear();
        self.voting_power = 0;
        self.proposer_priority = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Validator {
        static instance: Validator = Validator {
            address: ::std::vec::Vec::new(),
            pub_key: ::protobuf::MessageField::none(),
            voting_power: 0,
            proposer_priority: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Validator {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Validator").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Validator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Validator {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:tendermint.types.SimpleValidator)
pub struct SimpleValidator {
    // message fields
    // @@protoc_insertion_point(field:tendermint.types.SimpleValidator.pub_key)
    pub pub_key: ::protobuf::MessageField<super::keys::PublicKey>,
    // @@protoc_insertion_point(field:tendermint.types.SimpleValidator.voting_power)
    pub voting_power: i64,
    // special fields
    // @@protoc_insertion_point(special_field:tendermint.types.SimpleValidator.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SimpleValidator {
    fn default() -> &'a SimpleValidator {
        <SimpleValidator as ::protobuf::Message>::default_instance()
    }
}

impl SimpleValidator {
    pub fn new() -> SimpleValidator {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::keys::PublicKey>(
            "pub_key",
            |m: &SimpleValidator| { &m.pub_key },
            |m: &mut SimpleValidator| { &mut m.pub_key },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "voting_power",
            |m: &SimpleValidator| { &m.voting_power },
            |m: &mut SimpleValidator| { &mut m.voting_power },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SimpleValidator>(
            "SimpleValidator",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SimpleValidator {
    const NAME: &'static str = "SimpleValidator";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pub_key)?;
                },
                16 => {
                    self.voting_power = is.read_int64()?;
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
        if let Some(v) = self.pub_key.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.voting_power != 0 {
            my_size += ::protobuf::rt::int64_size(2, self.voting_power);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.pub_key.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.voting_power != 0 {
            os.write_int64(2, self.voting_power)?;
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

    fn new() -> SimpleValidator {
        SimpleValidator::new()
    }

    fn clear(&mut self) {
        self.pub_key.clear();
        self.voting_power = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SimpleValidator {
        static instance: SimpleValidator = SimpleValidator {
            pub_key: ::protobuf::MessageField::none(),
            voting_power: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SimpleValidator {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SimpleValidator").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SimpleValidator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SimpleValidator {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20tendermint/types/validator.proto\x12\x10tendermint.types\x1a\x14go\
    goproto/gogo.proto\x1a\x1ctendermint/crypto/keys.proto\"\xb2\x01\n\x0cVa\
    lidatorSet\x12;\n\nvalidators\x18\x01\x20\x03(\x0b2\x1b.tendermint.types\
    .ValidatorR\nvalidators\x127\n\x08proposer\x18\x02\x20\x01(\x0b2\x1b.ten\
    dermint.types.ValidatorR\x08proposer\x12,\n\x12total_voting_power\x18\
    \x03\x20\x01(\x03R\x10totalVotingPower\"\xb2\x01\n\tValidator\x12\x18\n\
    \x07address\x18\x01\x20\x01(\x0cR\x07address\x12;\n\x07pub_key\x18\x02\
    \x20\x01(\x0b2\x1c.tendermint.crypto.PublicKeyR\x06pubKeyB\x04\xc8\xde\
    \x1f\0\x12!\n\x0cvoting_power\x18\x03\x20\x01(\x03R\x0bvotingPower\x12+\
    \n\x11proposer_priority\x18\x04\x20\x01(\x03R\x10proposerPriority\"k\n\
    \x0fSimpleValidator\x125\n\x07pub_key\x18\x01\x20\x01(\x0b2\x1c.tendermi\
    nt.crypto.PublicKeyR\x06pubKey\x12!\n\x0cvoting_power\x18\x02\x20\x01(\
    \x03R\x0bvotingPowerB9Z7github.com/tendermint/tendermint/proto/tendermin\
    t/typesJ\xab\x05\n\x06\x12\x04\0\0\x18\x01\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\x08\n\x01\x02\x12\x03\x01\0\x19\n\x08\n\x01\x08\x12\x03\x03\0N\n\
    \t\n\x02\x08\x0b\x12\x03\x03\0N\n\t\n\x02\x03\0\x12\x03\x05\0\x1e\n\t\n\
    \x02\x03\x01\x12\x03\x06\0&\n\n\n\x02\x04\0\x12\x04\x08\0\x0c\x01\n\n\n\
    \x03\x04\0\x01\x12\x03\x08\x08\x14\n\x0b\n\x04\x04\0\x02\0\x12\x03\t\x02\
    ,\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\t\x02\n\n\x0c\n\x05\x04\0\x02\0\
    \x06\x12\x03\t\x0b\x14\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\t\x15\x1f\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\t*+\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \n\x02,\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\n\x02\x0b\n\x0c\n\x05\x04\
    \0\x02\x01\x01\x12\x03\n\x15\x1d\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\n\
    *+\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x0b\x02,\n\x0c\n\x05\x04\0\x02\x02\
    \x05\x12\x03\x0b\x02\x07\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0b\x15'\
    \n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0b*+\n\n\n\x02\x04\x01\x12\x04\
    \x0e\0\x13\x01\n\n\n\x03\x04\x01\x01\x12\x03\x0e\x08\x11\n\x0b\n\x04\x04\
    \x01\x02\0\x12\x03\x0f\x024\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x0f\
    \x02\x07\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0f\x1e%\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\x0f23\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x10\x02\
    S\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\x10\x02\x1d\n\x0c\n\x05\x04\
    \x01\x02\x01\x01\x12\x03\x10\x1e%\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\
    \x03\x1023\n\x0c\n\x05\x04\x01\x02\x01\x08\x12\x03\x104R\n\x0f\n\x08\x04\
    \x01\x02\x01\x08\xe9\xfb\x03\x12\x03\x105Q\n\x0b\n\x04\x04\x01\x02\x02\
    \x12\x03\x11\x024\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\x11\x02\x07\n\
    \x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x11\x1e*\n\x0c\n\x05\x04\x01\x02\
    \x02\x03\x12\x03\x1123\n\x0b\n\x04\x04\x01\x02\x03\x12\x03\x12\x024\n\
    \x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\x12\x02\x07\n\x0c\n\x05\x04\x01\
    \x02\x03\x01\x12\x03\x12\x1e/\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03\
    \x1223\n\n\n\x02\x04\x02\x12\x04\x15\0\x18\x01\n\n\n\x03\x04\x02\x01\x12\
    \x03\x15\x08\x17\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x16\x02/\n\x0c\n\x05\
    \x04\x02\x02\0\x06\x12\x03\x16\x02\x1d\n\x0c\n\x05\x04\x02\x02\0\x01\x12\
    \x03\x16\x1e%\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x16-.\n\x0b\n\x04\
    \x04\x02\x02\x01\x12\x03\x17\x02/\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\
    \x03\x17\x02\x07\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x17\x1e*\n\x0c\
    \n\x05\x04\x02\x02\x01\x03\x12\x03\x17-.b\x06proto3\
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
            deps.push(super::keys::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(ValidatorSet::generated_message_descriptor_data());
            messages.push(Validator::generated_message_descriptor_data());
            messages.push(SimpleValidator::generated_message_descriptor_data());
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

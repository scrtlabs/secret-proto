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

//! Generated file from `cosmos/authz/v1beta1/tx.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

///  MsgGrant is a request type for Grant method. It declares authorization to the grantee
///  on behalf of the granter with the provided expiration time.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.authz.v1beta1.MsgGrant)
pub struct MsgGrant {
    // message fields
    // @@protoc_insertion_point(field:cosmos.authz.v1beta1.MsgGrant.granter)
    pub granter: ::std::string::String,
    // @@protoc_insertion_point(field:cosmos.authz.v1beta1.MsgGrant.grantee)
    pub grantee: ::std::string::String,
    // @@protoc_insertion_point(field:cosmos.authz.v1beta1.MsgGrant.grant)
    pub grant: ::protobuf::MessageField<super::authz::Grant>,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.authz.v1beta1.MsgGrant.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgGrant {
    fn default() -> &'a MsgGrant {
        <MsgGrant as ::protobuf::Message>::default_instance()
    }
}

impl MsgGrant {
    pub fn new() -> MsgGrant {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "granter",
            |m: &MsgGrant| { &m.granter },
            |m: &mut MsgGrant| { &mut m.granter },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "grantee",
            |m: &MsgGrant| { &m.grantee },
            |m: &mut MsgGrant| { &mut m.grantee },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::authz::Grant>(
            "grant",
            |m: &MsgGrant| { &m.grant },
            |m: &mut MsgGrant| { &mut m.grant },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgGrant>(
            "MsgGrant",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgGrant {
    const NAME: &'static str = "MsgGrant";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.granter = is.read_string()?;
                },
                18 => {
                    self.grantee = is.read_string()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.grant)?;
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
        if !self.granter.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.granter);
        }
        if !self.grantee.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.grantee);
        }
        if let Some(v) = self.grant.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.granter.is_empty() {
            os.write_string(1, &self.granter)?;
        }
        if !self.grantee.is_empty() {
            os.write_string(2, &self.grantee)?;
        }
        if let Some(v) = self.grant.as_ref() {
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

    fn new() -> MsgGrant {
        MsgGrant::new()
    }

    fn clear(&mut self) {
        self.granter.clear();
        self.grantee.clear();
        self.grant.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgGrant {
        static instance: MsgGrant = MsgGrant {
            granter: ::std::string::String::new(),
            grantee: ::std::string::String::new(),
            grant: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgGrant {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgGrant").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgGrant {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgGrant {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  MsgExecResponse defines the Msg/MsgExecResponse response type.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.authz.v1beta1.MsgExecResponse)
pub struct MsgExecResponse {
    // message fields
    // @@protoc_insertion_point(field:cosmos.authz.v1beta1.MsgExecResponse.results)
    pub results: ::std::vec::Vec<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.authz.v1beta1.MsgExecResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgExecResponse {
    fn default() -> &'a MsgExecResponse {
        <MsgExecResponse as ::protobuf::Message>::default_instance()
    }
}

impl MsgExecResponse {
    pub fn new() -> MsgExecResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "results",
            |m: &MsgExecResponse| { &m.results },
            |m: &mut MsgExecResponse| { &mut m.results },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgExecResponse>(
            "MsgExecResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgExecResponse {
    const NAME: &'static str = "MsgExecResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.results.push(is.read_bytes()?);
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
        for value in &self.results {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.results {
            os.write_bytes(1, &v)?;
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

    fn new() -> MsgExecResponse {
        MsgExecResponse::new()
    }

    fn clear(&mut self) {
        self.results.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgExecResponse {
        static instance: MsgExecResponse = MsgExecResponse {
            results: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgExecResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgExecResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgExecResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgExecResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  MsgExec attempts to execute the provided messages using
///  authorizations granted to the grantee. Each message should have only
///  one signer corresponding to the granter of the authorization.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.authz.v1beta1.MsgExec)
pub struct MsgExec {
    // message fields
    // @@protoc_insertion_point(field:cosmos.authz.v1beta1.MsgExec.grantee)
    pub grantee: ::std::string::String,
    ///  Authorization Msg requests to execute. Each msg must implement Authorization interface
    ///  The x/authz will try to find a grant matching (msg.signers[0], grantee, MsgTypeURL(msg))
    ///  triple and validate it.
    // @@protoc_insertion_point(field:cosmos.authz.v1beta1.MsgExec.msgs)
    pub msgs: ::std::vec::Vec<::protobuf::well_known_types::any::Any>,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.authz.v1beta1.MsgExec.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgExec {
    fn default() -> &'a MsgExec {
        <MsgExec as ::protobuf::Message>::default_instance()
    }
}

impl MsgExec {
    pub fn new() -> MsgExec {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "grantee",
            |m: &MsgExec| { &m.grantee },
            |m: &mut MsgExec| { &mut m.grantee },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "msgs",
            |m: &MsgExec| { &m.msgs },
            |m: &mut MsgExec| { &mut m.msgs },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgExec>(
            "MsgExec",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgExec {
    const NAME: &'static str = "MsgExec";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.grantee = is.read_string()?;
                },
                18 => {
                    self.msgs.push(is.read_message()?);
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
        if !self.grantee.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.grantee);
        }
        for value in &self.msgs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.grantee.is_empty() {
            os.write_string(1, &self.grantee)?;
        }
        for v in &self.msgs {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> MsgExec {
        MsgExec::new()
    }

    fn clear(&mut self) {
        self.grantee.clear();
        self.msgs.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgExec {
        static instance: MsgExec = MsgExec {
            grantee: ::std::string::String::new(),
            msgs: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgExec {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgExec").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgExec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgExec {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  MsgGrantResponse defines the Msg/MsgGrant response type.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.authz.v1beta1.MsgGrantResponse)
pub struct MsgGrantResponse {
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.authz.v1beta1.MsgGrantResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgGrantResponse {
    fn default() -> &'a MsgGrantResponse {
        <MsgGrantResponse as ::protobuf::Message>::default_instance()
    }
}

impl MsgGrantResponse {
    pub fn new() -> MsgGrantResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(0);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgGrantResponse>(
            "MsgGrantResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgGrantResponse {
    const NAME: &'static str = "MsgGrantResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MsgGrantResponse {
        MsgGrantResponse::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgGrantResponse {
        static instance: MsgGrantResponse = MsgGrantResponse {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgGrantResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgGrantResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgGrantResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgGrantResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  MsgRevoke revokes any authorization with the provided sdk.Msg type on the
///  granter's account with that has been granted to the grantee.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.authz.v1beta1.MsgRevoke)
pub struct MsgRevoke {
    // message fields
    // @@protoc_insertion_point(field:cosmos.authz.v1beta1.MsgRevoke.granter)
    pub granter: ::std::string::String,
    // @@protoc_insertion_point(field:cosmos.authz.v1beta1.MsgRevoke.grantee)
    pub grantee: ::std::string::String,
    // @@protoc_insertion_point(field:cosmos.authz.v1beta1.MsgRevoke.msg_type_url)
    pub msg_type_url: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.authz.v1beta1.MsgRevoke.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgRevoke {
    fn default() -> &'a MsgRevoke {
        <MsgRevoke as ::protobuf::Message>::default_instance()
    }
}

impl MsgRevoke {
    pub fn new() -> MsgRevoke {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "granter",
            |m: &MsgRevoke| { &m.granter },
            |m: &mut MsgRevoke| { &mut m.granter },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "grantee",
            |m: &MsgRevoke| { &m.grantee },
            |m: &mut MsgRevoke| { &mut m.grantee },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "msg_type_url",
            |m: &MsgRevoke| { &m.msg_type_url },
            |m: &mut MsgRevoke| { &mut m.msg_type_url },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgRevoke>(
            "MsgRevoke",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgRevoke {
    const NAME: &'static str = "MsgRevoke";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.granter = is.read_string()?;
                },
                18 => {
                    self.grantee = is.read_string()?;
                },
                26 => {
                    self.msg_type_url = is.read_string()?;
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
        if !self.granter.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.granter);
        }
        if !self.grantee.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.grantee);
        }
        if !self.msg_type_url.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.msg_type_url);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.granter.is_empty() {
            os.write_string(1, &self.granter)?;
        }
        if !self.grantee.is_empty() {
            os.write_string(2, &self.grantee)?;
        }
        if !self.msg_type_url.is_empty() {
            os.write_string(3, &self.msg_type_url)?;
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

    fn new() -> MsgRevoke {
        MsgRevoke::new()
    }

    fn clear(&mut self) {
        self.granter.clear();
        self.grantee.clear();
        self.msg_type_url.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgRevoke {
        static instance: MsgRevoke = MsgRevoke {
            granter: ::std::string::String::new(),
            grantee: ::std::string::String::new(),
            msg_type_url: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgRevoke {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgRevoke").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgRevoke {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgRevoke {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  MsgRevokeResponse defines the Msg/MsgRevokeResponse response type.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.authz.v1beta1.MsgRevokeResponse)
pub struct MsgRevokeResponse {
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.authz.v1beta1.MsgRevokeResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgRevokeResponse {
    fn default() -> &'a MsgRevokeResponse {
        <MsgRevokeResponse as ::protobuf::Message>::default_instance()
    }
}

impl MsgRevokeResponse {
    pub fn new() -> MsgRevokeResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(0);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgRevokeResponse>(
            "MsgRevokeResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgRevokeResponse {
    const NAME: &'static str = "MsgRevokeResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MsgRevokeResponse {
        MsgRevokeResponse::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgRevokeResponse {
        static instance: MsgRevokeResponse = MsgRevokeResponse {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgRevokeResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgRevokeResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgRevokeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgRevokeResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dcosmos/authz/v1beta1/tx.proto\x12\x14cosmos.authz.v1beta1\x1a\x19c\
    osmos_proto/cosmos.proto\x1a\x14gogoproto/gogo.proto\x1a\x1fgoogle/proto\
    buf/timestamp.proto\x1a\x19google/protobuf/any.proto\x1a#cosmos/base/abc\
    i/v1beta1/abci.proto\x1a\x20cosmos/authz/v1beta1/authz.proto\"w\n\x08Msg\
    Grant\x12\x18\n\x07granter\x18\x01\x20\x01(\tR\x07granter\x12\x18\n\x07g\
    rantee\x18\x02\x20\x01(\tR\x07grantee\x127\n\x05grant\x18\x03\x20\x01(\
    \x0b2\x1b.cosmos.authz.v1beta1.GrantR\x05grantB\x04\xc8\xde\x1f\0\"+\n\
    \x0fMsgExecResponse\x12\x18\n\x07results\x18\x01\x20\x03(\x0cR\x07result\
    s\"o\n\x07MsgExec\x12\x18\n\x07grantee\x18\x01\x20\x01(\tR\x07grantee\
    \x12J\n\x04msgs\x18\x02\x20\x03(\x0b2\x14.google.protobuf.AnyR\x04msgsB\
    \x20\xca\xb4-\x1csdk.Msg,\x20authz.Authorization\"\x12\n\x10MsgGrantResp\
    onse\"a\n\tMsgRevoke\x12\x18\n\x07granter\x18\x01\x20\x01(\tR\x07granter\
    \x12\x18\n\x07grantee\x18\x02\x20\x01(\tR\x07grantee\x12\x20\n\x0cmsg_ty\
    pe_url\x18\x03\x20\x01(\tR\nmsgTypeUrl\"\x13\n\x11MsgRevokeResponse2\xf8\
    \x01\n\x03Msg\x12O\n\x05Grant\x12\x1e.cosmos.authz.v1beta1.MsgGrant\x1a&\
    .cosmos.authz.v1beta1.MsgGrantResponse\x12L\n\x04Exec\x12\x1d.cosmos.aut\
    hz.v1beta1.MsgExec\x1a%.cosmos.authz.v1beta1.MsgExecResponse\x12R\n\x06R\
    evoke\x12\x1f.cosmos.authz.v1beta1.MsgRevoke\x1a'.cosmos.authz.v1beta1.M\
    sgRevokeResponseB*Z$github.com/cosmos/cosmos-sdk/x/authz\xc8\xe1\x1e\0J\
    \x9a\x14\n\x06\x12\x04\x01\0E\x1c\n\"\n\x01\x0c\x12\x03\x01\0\x12\x1a\
    \x18\x20Since:\x20cosmos-sdk\x200.43\n\n\x08\n\x01\x02\x12\x03\x02\0\x1d\
    \n\t\n\x02\x03\0\x12\x03\x04\0#\n\t\n\x02\x03\x01\x12\x03\x05\0\x1e\n\t\
    \n\x02\x03\x02\x12\x03\x06\0)\n\t\n\x02\x03\x03\x12\x03\x07\0#\n\t\n\x02\
    \x03\x04\x12\x03\x08\0-\n\t\n\x02\x03\x05\x12\x03\t\0*\n\x08\n\x01\x08\
    \x12\x03\x0b\0P\n\t\n\x02\x08\x0b\x12\x03\x0b\0P\n\x08\n\x01\x08\x12\x03\
    \x0c\0/\n\x0b\n\x04\x08\x99\xec\x03\x12\x03\x0c\0/\n0\n\x02\x06\0\x12\
    \x04\x0f\0\x1e\x01\x1a$\x20Msg\x20defines\x20the\x20authz\x20Msg\x20serv\
    ice.\n\n\n\n\x03\x06\0\x01\x12\x03\x0f\x08\x0b\n\xfd\x01\n\x04\x06\0\x02\
    \0\x12\x03\x14\x021\x1a\xef\x01\x20Grant\x20grants\x20the\x20provided\
    \x20authorization\x20to\x20the\x20grantee\x20on\x20the\x20granter's\n\
    \x20account\x20with\x20the\x20provided\x20expiration\x20time.\x20If\x20t\
    here\x20is\x20already\x20a\x20grant\n\x20for\x20the\x20given\x20(granter\
    ,\x20grantee,\x20Authorization)\x20triple,\x20then\x20the\x20grant\n\x20\
    will\x20be\x20overwritten.\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x14\x06\
    \x0b\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x14\x0c\x14\n\x0c\n\x05\x06\0\
    \x02\0\x03\x12\x03\x14\x1f/\n\xc9\x01\n\x04\x06\0\x02\x01\x12\x03\x19\
    \x02.\x1a\xbb\x01\x20Exec\x20attempts\x20to\x20execute\x20the\x20provide\
    d\x20messages\x20using\n\x20authorizations\x20granted\x20to\x20the\x20gr\
    antee.\x20Each\x20message\x20should\x20have\x20only\n\x20one\x20signer\
    \x20corresponding\x20to\x20the\x20granter\x20of\x20the\x20authorization.\
    \n\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x19\x06\n\n\x0c\n\x05\x06\0\x02\
    \x01\x02\x12\x03\x19\x0b\x12\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\x19\
    \x1d,\n\x9a\x01\n\x04\x06\0\x02\x02\x12\x03\x1d\x024\x1a\x8c\x01\x20Revo\
    ke\x20revokes\x20any\x20authorization\x20corresponding\x20to\x20the\x20p\
    rovided\x20method\x20name\x20on\x20the\n\x20granter's\x20account\x20that\
    \x20has\x20been\x20granted\x20to\x20the\x20grantee.\n\n\x0c\n\x05\x06\0\
    \x02\x02\x01\x12\x03\x1d\x06\x0c\n\x0c\n\x05\x06\0\x02\x02\x02\x12\x03\
    \x1d\r\x16\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03\x1d!2\n\xa1\x01\n\x02\
    \x04\0\x12\x04\"\0'\x01\x1a\x94\x01\x20MsgGrant\x20is\x20a\x20request\
    \x20type\x20for\x20Grant\x20method.\x20It\x20declares\x20authorization\
    \x20to\x20the\x20grantee\n\x20on\x20behalf\x20of\x20the\x20granter\x20wi\
    th\x20the\x20provided\x20expiration\x20time.\n\n\n\n\x03\x04\0\x01\x12\
    \x03\"\x08\x10\n\x0b\n\x04\x04\0\x02\0\x12\x03#\x02\x15\n\x0c\n\x05\x04\
    \0\x02\0\x05\x12\x03#\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03#\t\x10\
    \n\x0c\n\x05\x04\0\x02\0\x03\x12\x03#\x13\x14\n\x0b\n\x04\x04\0\x02\x01\
    \x12\x03$\x02\x15\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03$\x02\x08\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x03$\t\x10\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03$\x13\x14\n\x0b\n\x04\x04\0\x02\x02\x12\x03&\x02F\n\x0c\n\x05\x04\0\
    \x02\x02\x06\x12\x03&\x02\x1c\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03&\x1d\
    \"\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03&%&\n\x0c\n\x05\x04\0\x02\x02\
    \x08\x12\x03&'E\n\x0f\n\x08\x04\0\x02\x02\x08\xe9\xfb\x03\x12\x03&(D\nL\
    \n\x02\x04\x01\x12\x04*\0,\x01\x1a@\x20MsgExecResponse\x20defines\x20the\
    \x20Msg/MsgExecResponse\x20response\x20type.\n\n\n\n\x03\x04\x01\x01\x12\
    \x03*\x08\x17\n\x0b\n\x04\x04\x01\x02\0\x12\x03+\x02\x1d\n\x0c\n\x05\x04\
    \x01\x02\0\x04\x12\x03+\x02\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03+\x0b\
    \x10\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03+\x11\x18\n\x0c\n\x05\x04\x01\
    \x02\0\x03\x12\x03+\x1b\x1c\n\xcb\x01\n\x02\x04\x02\x12\x041\07\x01\x1a\
    \xbe\x01\x20MsgExec\x20attempts\x20to\x20execute\x20the\x20provided\x20m\
    essages\x20using\n\x20authorizations\x20granted\x20to\x20the\x20grantee.\
    \x20Each\x20message\x20should\x20have\x20only\n\x20one\x20signer\x20corr\
    esponding\x20to\x20the\x20granter\x20of\x20the\x20authorization.\n\n\n\n\
    \x03\x04\x02\x01\x12\x031\x08\x0f\n\x0b\n\x04\x04\x02\x02\0\x12\x032\x02\
    \x15\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x032\x02\x08\n\x0c\n\x05\x04\x02\
    \x02\0\x01\x12\x032\t\x10\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x032\x13\x14\
    \n\xd9\x01\n\x04\x04\x02\x02\x01\x12\x036\x02l\x1a\xcb\x01\x20Authorizat\
    ion\x20Msg\x20requests\x20to\x20execute.\x20Each\x20msg\x20must\x20imple\
    ment\x20Authorization\x20interface\n\x20The\x20x/authz\x20will\x20try\
    \x20to\x20find\x20a\x20grant\x20matching\x20(msg.signers[0],\x20grantee,\
    \x20MsgTypeURL(msg))\n\x20triple\x20and\x20validate\x20it.\n\n\x0c\n\x05\
    \x04\x02\x02\x01\x04\x12\x036\x02\n\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\
    \x036\x0b\x1e\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x036\x1f#\n\x0c\n\x05\
    \x04\x02\x02\x01\x03\x12\x036&'\n\x0c\n\x05\x04\x02\x02\x01\x08\x12\x036\
    (k\n\x0f\n\x08\x04\x02\x02\x01\x08\xc9\xd6\x05\x12\x036)j\nE\n\x02\x04\
    \x03\x12\x03:\0\x1b\x1a:\x20MsgGrantResponse\x20defines\x20the\x20Msg/Ms\
    gGrant\x20response\x20type.\n\n\n\n\x03\x04\x03\x01\x12\x03:\x08\x18\n\
    \x96\x01\n\x02\x04\x04\x12\x04>\0B\x01\x1a\x89\x01\x20MsgRevoke\x20revok\
    es\x20any\x20authorization\x20with\x20the\x20provided\x20sdk.Msg\x20type\
    \x20on\x20the\n\x20granter's\x20account\x20with\x20that\x20has\x20been\
    \x20granted\x20to\x20the\x20grantee.\n\n\n\n\x03\x04\x04\x01\x12\x03>\
    \x08\x11\n\x0b\n\x04\x04\x04\x02\0\x12\x03?\x02\x1a\n\x0c\n\x05\x04\x04\
    \x02\0\x05\x12\x03?\x02\x08\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03?\t\x10\
    \n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03?\x18\x19\n\x0b\n\x04\x04\x04\x02\
    \x01\x12\x03@\x02\x1a\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03@\x02\x08\n\
    \x0c\n\x05\x04\x04\x02\x01\x01\x12\x03@\t\x10\n\x0c\n\x05\x04\x04\x02\
    \x01\x03\x12\x03@\x18\x19\n\x0b\n\x04\x04\x04\x02\x02\x12\x03A\x02\x1a\n\
    \x0c\n\x05\x04\x04\x02\x02\x05\x12\x03A\x02\x08\n\x0c\n\x05\x04\x04\x02\
    \x02\x01\x12\x03A\t\x15\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03A\x18\x19\
    \nO\n\x02\x04\x05\x12\x03E\0\x1c\x1aD\x20MsgRevokeResponse\x20defines\
    \x20the\x20Msg/MsgRevokeResponse\x20response\x20type.\n\n\n\n\x03\x04\
    \x05\x01\x12\x03E\x08\x19b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(6);
            deps.push(super::cosmos::file_descriptor().clone());
            deps.push(super::gogo::file_descriptor().clone());
            deps.push(::protobuf::well_known_types::timestamp::file_descriptor().clone());
            deps.push(::protobuf::well_known_types::any::file_descriptor().clone());
            deps.push(super::abci::file_descriptor().clone());
            deps.push(super::authz::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(6);
            messages.push(MsgGrant::generated_message_descriptor_data());
            messages.push(MsgExecResponse::generated_message_descriptor_data());
            messages.push(MsgExec::generated_message_descriptor_data());
            messages.push(MsgGrantResponse::generated_message_descriptor_data());
            messages.push(MsgRevoke::generated_message_descriptor_data());
            messages.push(MsgRevokeResponse::generated_message_descriptor_data());
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

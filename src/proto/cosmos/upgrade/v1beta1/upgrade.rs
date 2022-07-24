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

//! Generated file from `cosmos/upgrade/v1beta1/upgrade.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

///  Plan specifies information about a planned upgrade and when it should occur.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.upgrade.v1beta1.Plan)
pub struct Plan {
    // message fields
    ///  Sets the name for the upgrade. This name will be used by the upgraded
    ///  version of the software to apply any special "on-upgrade" commands during
    ///  the first BeginBlock method after the upgrade is applied. It is also used
    ///  to detect whether a software version can handle a given upgrade. If no
    ///  upgrade handler with this name has been set in the software, it will be
    ///  assumed that the software is out-of-date when the upgrade Time or Height is
    ///  reached and the software will exit.
    // @@protoc_insertion_point(field:cosmos.upgrade.v1beta1.Plan.name)
    pub name: ::std::string::String,
    ///  Deprecated: Time based upgrades have been deprecated. Time based upgrade logic
    ///  has been removed from the SDK.
    ///  If this field is not empty, an error will be thrown.
    // @@protoc_insertion_point(field:cosmos.upgrade.v1beta1.Plan.time)
    pub time: ::protobuf::MessageField<::protobuf::well_known_types::timestamp::Timestamp>,
    ///  The height at which the upgrade must be performed.
    ///  Only used if Time is not set.
    // @@protoc_insertion_point(field:cosmos.upgrade.v1beta1.Plan.height)
    pub height: i64,
    ///  Any application specific upgrade info to be included on-chain
    ///  such as a git commit that validators could automatically upgrade to
    // @@protoc_insertion_point(field:cosmos.upgrade.v1beta1.Plan.info)
    pub info: ::std::string::String,
    ///  Deprecated: UpgradedClientState field has been deprecated. IBC upgrade logic has been
    ///  moved to the IBC module in the sub module 02-client.
    ///  If this field is not empty, an error will be thrown.
    // @@protoc_insertion_point(field:cosmos.upgrade.v1beta1.Plan.upgraded_client_state)
    pub upgraded_client_state: ::protobuf::MessageField<::protobuf::well_known_types::any::Any>,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.upgrade.v1beta1.Plan.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Plan {
    fn default() -> &'a Plan {
        <Plan as ::protobuf::Message>::default_instance()
    }
}

impl Plan {
    pub fn new() -> Plan {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &Plan| { &m.name },
            |m: &mut Plan| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::timestamp::Timestamp>(
            "time",
            |m: &Plan| { &m.time },
            |m: &mut Plan| { &mut m.time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "height",
            |m: &Plan| { &m.height },
            |m: &mut Plan| { &mut m.height },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "info",
            |m: &Plan| { &m.info },
            |m: &mut Plan| { &mut m.info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::any::Any>(
            "upgraded_client_state",
            |m: &Plan| { &m.upgraded_client_state },
            |m: &mut Plan| { &mut m.upgraded_client_state },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Plan>(
            "Plan",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Plan {
    const NAME: &'static str = "Plan";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.name = is.read_string()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.time)?;
                },
                24 => {
                    self.height = is.read_int64()?;
                },
                34 => {
                    self.info = is.read_string()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.upgraded_client_state)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if let Some(v) = self.time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::int64_size(3, self.height);
        }
        if !self.info.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.info);
        }
        if let Some(v) = self.upgraded_client_state.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(v) = self.time.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.height != 0 {
            os.write_int64(3, self.height)?;
        }
        if !self.info.is_empty() {
            os.write_string(4, &self.info)?;
        }
        if let Some(v) = self.upgraded_client_state.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
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

    fn new() -> Plan {
        Plan::new()
    }

    fn clear(&mut self) {
        self.name.clear();
        self.time.clear();
        self.height = 0;
        self.info.clear();
        self.upgraded_client_state.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Plan {
        static instance: Plan = Plan {
            name: ::std::string::String::new(),
            time: ::protobuf::MessageField::none(),
            height: 0,
            info: ::std::string::String::new(),
            upgraded_client_state: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Plan {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Plan").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Plan {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Plan {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  SoftwareUpgradeProposal is a gov Content type for initiating a software
///  upgrade.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.upgrade.v1beta1.SoftwareUpgradeProposal)
pub struct SoftwareUpgradeProposal {
    // message fields
    // @@protoc_insertion_point(field:cosmos.upgrade.v1beta1.SoftwareUpgradeProposal.title)
    pub title: ::std::string::String,
    // @@protoc_insertion_point(field:cosmos.upgrade.v1beta1.SoftwareUpgradeProposal.description)
    pub description: ::std::string::String,
    // @@protoc_insertion_point(field:cosmos.upgrade.v1beta1.SoftwareUpgradeProposal.plan)
    pub plan: ::protobuf::MessageField<Plan>,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.upgrade.v1beta1.SoftwareUpgradeProposal.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SoftwareUpgradeProposal {
    fn default() -> &'a SoftwareUpgradeProposal {
        <SoftwareUpgradeProposal as ::protobuf::Message>::default_instance()
    }
}

impl SoftwareUpgradeProposal {
    pub fn new() -> SoftwareUpgradeProposal {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "title",
            |m: &SoftwareUpgradeProposal| { &m.title },
            |m: &mut SoftwareUpgradeProposal| { &mut m.title },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "description",
            |m: &SoftwareUpgradeProposal| { &m.description },
            |m: &mut SoftwareUpgradeProposal| { &mut m.description },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Plan>(
            "plan",
            |m: &SoftwareUpgradeProposal| { &m.plan },
            |m: &mut SoftwareUpgradeProposal| { &mut m.plan },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SoftwareUpgradeProposal>(
            "SoftwareUpgradeProposal",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SoftwareUpgradeProposal {
    const NAME: &'static str = "SoftwareUpgradeProposal";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.title = is.read_string()?;
                },
                18 => {
                    self.description = is.read_string()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.plan)?;
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
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.title);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.description);
        }
        if let Some(v) = self.plan.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.title.is_empty() {
            os.write_string(1, &self.title)?;
        }
        if !self.description.is_empty() {
            os.write_string(2, &self.description)?;
        }
        if let Some(v) = self.plan.as_ref() {
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

    fn new() -> SoftwareUpgradeProposal {
        SoftwareUpgradeProposal::new()
    }

    fn clear(&mut self) {
        self.title.clear();
        self.description.clear();
        self.plan.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SoftwareUpgradeProposal {
        static instance: SoftwareUpgradeProposal = SoftwareUpgradeProposal {
            title: ::std::string::String::new(),
            description: ::std::string::String::new(),
            plan: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SoftwareUpgradeProposal {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SoftwareUpgradeProposal").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SoftwareUpgradeProposal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SoftwareUpgradeProposal {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  CancelSoftwareUpgradeProposal is a gov Content type for cancelling a software
///  upgrade.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.upgrade.v1beta1.CancelSoftwareUpgradeProposal)
pub struct CancelSoftwareUpgradeProposal {
    // message fields
    // @@protoc_insertion_point(field:cosmos.upgrade.v1beta1.CancelSoftwareUpgradeProposal.title)
    pub title: ::std::string::String,
    // @@protoc_insertion_point(field:cosmos.upgrade.v1beta1.CancelSoftwareUpgradeProposal.description)
    pub description: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.upgrade.v1beta1.CancelSoftwareUpgradeProposal.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CancelSoftwareUpgradeProposal {
    fn default() -> &'a CancelSoftwareUpgradeProposal {
        <CancelSoftwareUpgradeProposal as ::protobuf::Message>::default_instance()
    }
}

impl CancelSoftwareUpgradeProposal {
    pub fn new() -> CancelSoftwareUpgradeProposal {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "title",
            |m: &CancelSoftwareUpgradeProposal| { &m.title },
            |m: &mut CancelSoftwareUpgradeProposal| { &mut m.title },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "description",
            |m: &CancelSoftwareUpgradeProposal| { &m.description },
            |m: &mut CancelSoftwareUpgradeProposal| { &mut m.description },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CancelSoftwareUpgradeProposal>(
            "CancelSoftwareUpgradeProposal",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CancelSoftwareUpgradeProposal {
    const NAME: &'static str = "CancelSoftwareUpgradeProposal";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.title = is.read_string()?;
                },
                18 => {
                    self.description = is.read_string()?;
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
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.title);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.description);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.title.is_empty() {
            os.write_string(1, &self.title)?;
        }
        if !self.description.is_empty() {
            os.write_string(2, &self.description)?;
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

    fn new() -> CancelSoftwareUpgradeProposal {
        CancelSoftwareUpgradeProposal::new()
    }

    fn clear(&mut self) {
        self.title.clear();
        self.description.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CancelSoftwareUpgradeProposal {
        static instance: CancelSoftwareUpgradeProposal = CancelSoftwareUpgradeProposal {
            title: ::std::string::String::new(),
            description: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CancelSoftwareUpgradeProposal {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CancelSoftwareUpgradeProposal").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CancelSoftwareUpgradeProposal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CancelSoftwareUpgradeProposal {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  ModuleVersion specifies a module and its consensus version.
///
///  Since: cosmos-sdk 0.43
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.upgrade.v1beta1.ModuleVersion)
pub struct ModuleVersion {
    // message fields
    ///  name of the app module
    // @@protoc_insertion_point(field:cosmos.upgrade.v1beta1.ModuleVersion.name)
    pub name: ::std::string::String,
    ///  consensus version of the app module
    // @@protoc_insertion_point(field:cosmos.upgrade.v1beta1.ModuleVersion.version)
    pub version: u64,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.upgrade.v1beta1.ModuleVersion.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ModuleVersion {
    fn default() -> &'a ModuleVersion {
        <ModuleVersion as ::protobuf::Message>::default_instance()
    }
}

impl ModuleVersion {
    pub fn new() -> ModuleVersion {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &ModuleVersion| { &m.name },
            |m: &mut ModuleVersion| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "version",
            |m: &ModuleVersion| { &m.version },
            |m: &mut ModuleVersion| { &mut m.version },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ModuleVersion>(
            "ModuleVersion",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ModuleVersion {
    const NAME: &'static str = "ModuleVersion";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.name = is.read_string()?;
                },
                16 => {
                    self.version = is.read_uint64()?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if self.version != 0 {
            my_size += ::protobuf::rt::uint64_size(2, self.version);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.version != 0 {
            os.write_uint64(2, self.version)?;
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

    fn new() -> ModuleVersion {
        ModuleVersion::new()
    }

    fn clear(&mut self) {
        self.name.clear();
        self.version = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ModuleVersion {
        static instance: ModuleVersion = ModuleVersion {
            name: ::std::string::String::new(),
            version: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ModuleVersion {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ModuleVersion").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ModuleVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ModuleVersion {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$cosmos/upgrade/v1beta1/upgrade.proto\x12\x16cosmos.upgrade.v1beta1\
    \x1a\x19google/protobuf/any.proto\x1a\x14gogoproto/gogo.proto\x1a\x1fgoo\
    gle/protobuf/timestamp.proto\"\xfa\x01\n\x04Plan\x12\x12\n\x04name\x18\
    \x01\x20\x01(\tR\x04name\x12:\n\x04time\x18\x02\x20\x01(\x0b2\x1a.google\
    .protobuf.TimestampR\x04timeB\n\x18\x01\x90\xdf\x1f\x01\xc8\xde\x1f\0\
    \x12\x16\n\x06height\x18\x03\x20\x01(\x03R\x06height\x12\x12\n\x04info\
    \x18\x04\x20\x01(\tR\x04info\x12l\n\x15upgraded_client_state\x18\x05\x20\
    \x01(\x0b2\x14.google.protobuf.AnyR\x13upgradedClientStateB\"\x18\x01\
    \xf2\xde\x1f\x1cyaml:\"upgraded_client_state\":\x08\xe8\xa0\x1f\x01\x98\
    \xa0\x1f\0\"\x93\x01\n\x17SoftwareUpgradeProposal\x12\x14\n\x05title\x18\
    \x01\x20\x01(\tR\x05title\x12\x20\n\x0bdescription\x18\x02\x20\x01(\tR\
    \x0bdescription\x126\n\x04plan\x18\x03\x20\x01(\x0b2\x1c.cosmos.upgrade.\
    v1beta1.PlanR\x04planB\x04\xc8\xde\x1f\0:\x08\xe8\xa0\x1f\x01\x98\xa0\
    \x1f\0\"a\n\x1dCancelSoftwareUpgradeProposal\x12\x14\n\x05title\x18\x01\
    \x20\x01(\tR\x05title\x12\x20\n\x0bdescription\x18\x02\x20\x01(\tR\x0bde\
    scription:\x08\xe8\xa0\x1f\x01\x98\xa0\x1f\0\"G\n\rModuleVersion\x12\x12\
    \n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x18\n\x07version\x18\x02\x20\
    \x01(\x04R\x07version:\x08\xe8\xa0\x1f\x01\x98\xa0\x1f\x01B2Z,github.com\
    /cosmos/cosmos-sdk/x/upgrade/types\xc8\xe1\x1e\0J\x8d\x15\n\x06\x12\x04\
    \0\0M\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\0\
    \x1f\n\t\n\x02\x03\0\x12\x03\x03\0#\n\t\n\x02\x03\x01\x12\x03\x04\0\x1e\
    \n\t\n\x02\x03\x02\x12\x03\x05\0)\n\x08\n\x01\x08\x12\x03\x07\0X\n\t\n\
    \x02\x08\x0b\x12\x03\x07\0X\n\x08\n\x01\x08\x12\x03\x08\0/\n\x0b\n\x04\
    \x08\x99\xec\x03\x12\x03\x08\0/\nZ\n\x02\x04\0\x12\x04\x0b\0*\x01\x1aN\
    \x20Plan\x20specifies\x20information\x20about\x20a\x20planned\x20upgrade\
    \x20and\x20when\x20it\x20should\x20occur.\n\n\n\n\x03\x04\0\x01\x12\x03\
    \x0b\x08\x0c\n\n\n\x03\x04\0\x07\x12\x03\x0c\x02-\n\r\n\x06\x04\0\x07\
    \x8d\xf4\x03\x12\x03\x0c\x02-\n\n\n\x03\x04\0\x07\x12\x03\r\x02.\n\r\n\
    \x06\x04\0\x07\x83\xf4\x03\x12\x03\r\x02.\n\xee\x03\n\x04\x04\0\x02\0\
    \x12\x03\x16\x02\x12\x1a\xe0\x03\x20Sets\x20the\x20name\x20for\x20the\
    \x20upgrade.\x20This\x20name\x20will\x20be\x20used\x20by\x20the\x20upgra\
    ded\n\x20version\x20of\x20the\x20software\x20to\x20apply\x20any\x20speci\
    al\x20\"on-upgrade\"\x20commands\x20during\n\x20the\x20first\x20BeginBlo\
    ck\x20method\x20after\x20the\x20upgrade\x20is\x20applied.\x20It\x20is\
    \x20also\x20used\n\x20to\x20detect\x20whether\x20a\x20software\x20versio\
    n\x20can\x20handle\x20a\x20given\x20upgrade.\x20If\x20no\n\x20upgrade\
    \x20handler\x20with\x20this\x20name\x20has\x20been\x20set\x20in\x20the\
    \x20software,\x20it\x20will\x20be\n\x20assumed\x20that\x20the\x20softwar\
    e\x20is\x20out-of-date\x20when\x20the\x20upgrade\x20Time\x20or\x20Height\
    \x20is\n\x20reached\x20and\x20the\x20software\x20will\x20exit.\n\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03\x16\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\x16\t\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x16\x10\x11\n\xb4\x01\n\
    \x04\x04\0\x02\x01\x12\x03\x1b\x02s\x1a\xa6\x01\x20Deprecated:\x20Time\
    \x20based\x20upgrades\x20have\x20been\x20deprecated.\x20Time\x20based\
    \x20upgrade\x20logic\n\x20has\x20been\x20removed\x20from\x20the\x20SDK.\
    \n\x20If\x20this\x20field\x20is\x20not\x20empty,\x20an\x20error\x20will\
    \x20be\x20thrown.\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x1b\x02\x1b\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x1b\x1c\x20\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\x1b#$\n\x0c\n\x05\x04\0\x02\x01\x08\x12\x03\x1b%r\n\r\n\
    \x06\x04\0\x02\x01\x08\x03\x12\x03\x1b&7\n\x0f\n\x08\x04\0\x02\x01\x08\
    \xf2\xfb\x03\x12\x03\x1b9S\n\x0f\n\x08\x04\0\x02\x01\x08\xe9\xfb\x03\x12\
    \x03\x1bUq\n`\n\x04\x04\0\x02\x02\x12\x03\x1f\x02\x13\x1aS\x20The\x20hei\
    ght\x20at\x20which\x20the\x20upgrade\x20must\x20be\x20performed.\n\x20On\
    ly\x20used\x20if\x20Time\x20is\x20not\x20set.\n\n\x0c\n\x05\x04\0\x02\
    \x02\x05\x12\x03\x1f\x02\x07\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x1f\
    \x08\x0e\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x1f\x11\x12\n\x92\x01\n\
    \x04\x04\0\x02\x03\x12\x03#\x02\x12\x1a\x84\x01\x20Any\x20application\
    \x20specific\x20upgrade\x20info\x20to\x20be\x20included\x20on-chain\n\
    \x20such\x20as\x20a\x20git\x20commit\x20that\x20validators\x20could\x20a\
    utomatically\x20upgrade\x20to\n\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03#\
    \x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03#\t\r\n\x0c\n\x05\x04\0\
    \x02\x03\x03\x12\x03#\x10\x11\n\xd2\x01\n\x04\x04\0\x02\x04\x12\x04(\x02\
    )S\x1a\xc3\x01\x20Deprecated:\x20UpgradedClientState\x20field\x20has\x20\
    been\x20deprecated.\x20IBC\x20upgrade\x20logic\x20has\x20been\n\x20moved\
    \x20to\x20the\x20IBC\x20module\x20in\x20the\x20sub\x20module\x2002-clien\
    t.\n\x20If\x20this\x20field\x20is\x20not\x20empty,\x20an\x20error\x20wil\
    l\x20be\x20thrown.\n\n\x0c\n\x05\x04\0\x02\x04\x06\x12\x03(\x02\x15\n\
    \x0c\n\x05\x04\0\x02\x04\x01\x12\x03(\x16+\n\x0c\n\x05\x04\0\x02\x04\x03\
    \x12\x03(./\n\x0c\n\x05\x04\0\x02\x04\x08\x12\x03)\x06R\n\r\n\x06\x04\0\
    \x02\x04\x08\x03\x12\x03)\x07\x18\n\x0f\n\x08\x04\0\x02\x04\x08\xee\xfb\
    \x03\x12\x03)\x1aQ\n_\n\x02\x04\x01\x12\x04.\05\x01\x1aS\x20SoftwareUpgr\
    adeProposal\x20is\x20a\x20gov\x20Content\x20type\x20for\x20initiating\
    \x20a\x20software\n\x20upgrade.\n\n\n\n\x03\x04\x01\x01\x12\x03.\x08\x1f\
    \n\n\n\x03\x04\x01\x07\x12\x03/\x02-\n\r\n\x06\x04\x01\x07\x8d\xf4\x03\
    \x12\x03/\x02-\n\n\n\x03\x04\x01\x07\x12\x030\x02.\n\r\n\x06\x04\x01\x07\
    \x83\xf4\x03\x12\x030\x02.\n\x0b\n\x04\x04\x01\x02\0\x12\x032\x02\x19\n\
    \x0c\n\x05\x04\x01\x02\0\x05\x12\x032\x02\x08\n\x0c\n\x05\x04\x01\x02\0\
    \x01\x12\x032\t\x0e\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x032\x17\x18\n\x0b\
    \n\x04\x04\x01\x02\x01\x12\x033\x02\x19\n\x0c\n\x05\x04\x01\x02\x01\x05\
    \x12\x033\x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x033\t\x14\n\x0c\n\
    \x05\x04\x01\x02\x01\x03\x12\x033\x17\x18\n\x0b\n\x04\x04\x01\x02\x02\
    \x12\x034\x028\n\x0c\n\x05\x04\x01\x02\x02\x06\x12\x034\x02\x06\n\x0c\n\
    \x05\x04\x01\x02\x02\x01\x12\x034\t\r\n\x0c\n\x05\x04\x01\x02\x02\x03\
    \x12\x034\x17\x18\n\x0c\n\x05\x04\x01\x02\x02\x08\x12\x034\x197\n\x0f\n\
    \x08\x04\x01\x02\x02\x08\xe9\xfb\x03\x12\x034\x1a6\ne\n\x02\x04\x02\x12\
    \x049\0?\x01\x1aY\x20CancelSoftwareUpgradeProposal\x20is\x20a\x20gov\x20\
    Content\x20type\x20for\x20cancelling\x20a\x20software\n\x20upgrade.\n\n\
    \n\n\x03\x04\x02\x01\x12\x039\x08%\n\n\n\x03\x04\x02\x07\x12\x03:\x02-\n\
    \r\n\x06\x04\x02\x07\x8d\xf4\x03\x12\x03:\x02-\n\n\n\x03\x04\x02\x07\x12\
    \x03;\x02.\n\r\n\x06\x04\x02\x07\x83\xf4\x03\x12\x03;\x02.\n\x0b\n\x04\
    \x04\x02\x02\0\x12\x03=\x02\x19\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03=\
    \x02\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03=\t\x0e\n\x0c\n\x05\x04\
    \x02\x02\0\x03\x12\x03=\x17\x18\n\x0b\n\x04\x04\x02\x02\x01\x12\x03>\x02\
    \x19\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03>\x02\x08\n\x0c\n\x05\x04\
    \x02\x02\x01\x01\x12\x03>\t\x14\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03>\
    \x17\x18\nb\n\x02\x04\x03\x12\x04D\0M\x01\x1aV\x20ModuleVersion\x20speci\
    fies\x20a\x20module\x20and\x20its\x20consensus\x20version.\n\n\x20Since:\
    \x20cosmos-sdk\x200.43\n\n\n\n\x03\x04\x03\x01\x12\x03D\x08\x15\n\n\n\
    \x03\x04\x03\x07\x12\x03E\x02-\n\r\n\x06\x04\x03\x07\x8d\xf4\x03\x12\x03\
    E\x02-\n\n\n\x03\x04\x03\x07\x12\x03F\x02-\n\r\n\x06\x04\x03\x07\x83\xf4\
    \x03\x12\x03F\x02-\n%\n\x04\x04\x03\x02\0\x12\x03I\x02\x12\x1a\x18\x20na\
    me\x20of\x20the\x20app\x20module\n\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\
    I\x02\x08\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03I\t\r\n\x0c\n\x05\x04\x03\
    \x02\0\x03\x12\x03I\x10\x11\n2\n\x04\x04\x03\x02\x01\x12\x03L\x02\x15\
    \x1a%\x20consensus\x20version\x20of\x20the\x20app\x20module\n\n\x0c\n\
    \x05\x04\x03\x02\x01\x05\x12\x03L\x02\x08\n\x0c\n\x05\x04\x03\x02\x01\
    \x01\x12\x03L\t\x10\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03L\x13\x14b\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(::protobuf::well_known_types::any::file_descriptor().clone());
            deps.push(super::gogo::file_descriptor().clone());
            deps.push(::protobuf::well_known_types::timestamp::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(Plan::generated_message_descriptor_data());
            messages.push(SoftwareUpgradeProposal::generated_message_descriptor_data());
            messages.push(CancelSoftwareUpgradeProposal::generated_message_descriptor_data());
            messages.push(ModuleVersion::generated_message_descriptor_data());
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

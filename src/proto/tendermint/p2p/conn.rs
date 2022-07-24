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

//! Generated file from `tendermint/p2p/conn.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:tendermint.p2p.PacketPing)
pub struct PacketPing {
    // special fields
    // @@protoc_insertion_point(special_field:tendermint.p2p.PacketPing.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PacketPing {
    fn default() -> &'a PacketPing {
        <PacketPing as ::protobuf::Message>::default_instance()
    }
}

impl PacketPing {
    pub fn new() -> PacketPing {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(0);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PacketPing>(
            "PacketPing",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PacketPing {
    const NAME: &'static str = "PacketPing";

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

    fn new() -> PacketPing {
        PacketPing::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PacketPing {
        static instance: PacketPing = PacketPing {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PacketPing {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PacketPing").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PacketPing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PacketPing {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:tendermint.p2p.PacketPong)
pub struct PacketPong {
    // special fields
    // @@protoc_insertion_point(special_field:tendermint.p2p.PacketPong.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PacketPong {
    fn default() -> &'a PacketPong {
        <PacketPong as ::protobuf::Message>::default_instance()
    }
}

impl PacketPong {
    pub fn new() -> PacketPong {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(0);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PacketPong>(
            "PacketPong",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PacketPong {
    const NAME: &'static str = "PacketPong";

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

    fn new() -> PacketPong {
        PacketPong::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PacketPong {
        static instance: PacketPong = PacketPong {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PacketPong {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PacketPong").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PacketPong {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PacketPong {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:tendermint.p2p.PacketMsg)
pub struct PacketMsg {
    // message fields
    // @@protoc_insertion_point(field:tendermint.p2p.PacketMsg.channel_id)
    pub channel_id: i32,
    // @@protoc_insertion_point(field:tendermint.p2p.PacketMsg.eof)
    pub eof: bool,
    // @@protoc_insertion_point(field:tendermint.p2p.PacketMsg.data)
    pub data: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:tendermint.p2p.PacketMsg.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PacketMsg {
    fn default() -> &'a PacketMsg {
        <PacketMsg as ::protobuf::Message>::default_instance()
    }
}

impl PacketMsg {
    pub fn new() -> PacketMsg {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "channel_id",
            |m: &PacketMsg| { &m.channel_id },
            |m: &mut PacketMsg| { &mut m.channel_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "eof",
            |m: &PacketMsg| { &m.eof },
            |m: &mut PacketMsg| { &mut m.eof },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data",
            |m: &PacketMsg| { &m.data },
            |m: &mut PacketMsg| { &mut m.data },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PacketMsg>(
            "PacketMsg",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PacketMsg {
    const NAME: &'static str = "PacketMsg";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.channel_id = is.read_int32()?;
                },
                16 => {
                    self.eof = is.read_bool()?;
                },
                26 => {
                    self.data = is.read_bytes()?;
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
        if self.channel_id != 0 {
            my_size += ::protobuf::rt::int32_size(1, self.channel_id);
        }
        if self.eof != false {
            my_size += 1 + 1;
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.channel_id != 0 {
            os.write_int32(1, self.channel_id)?;
        }
        if self.eof != false {
            os.write_bool(2, self.eof)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(3, &self.data)?;
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

    fn new() -> PacketMsg {
        PacketMsg::new()
    }

    fn clear(&mut self) {
        self.channel_id = 0;
        self.eof = false;
        self.data.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PacketMsg {
        static instance: PacketMsg = PacketMsg {
            channel_id: 0,
            eof: false,
            data: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PacketMsg {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PacketMsg").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PacketMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PacketMsg {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:tendermint.p2p.Packet)
pub struct Packet {
    // message oneof groups
    pub sum: ::std::option::Option<packet::Sum>,
    // special fields
    // @@protoc_insertion_point(special_field:tendermint.p2p.Packet.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Packet {
    fn default() -> &'a Packet {
        <Packet as ::protobuf::Message>::default_instance()
    }
}

impl Packet {
    pub fn new() -> Packet {
        ::std::default::Default::default()
    }

    // .tendermint.p2p.PacketPing packet_ping = 1;

    pub fn packet_ping(&self) -> &PacketPing {
        match self.sum {
            ::std::option::Option::Some(packet::Sum::PacketPing(ref v)) => v,
            _ => <PacketPing as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_packet_ping(&mut self) {
        self.sum = ::std::option::Option::None;
    }

    pub fn has_packet_ping(&self) -> bool {
        match self.sum {
            ::std::option::Option::Some(packet::Sum::PacketPing(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_packet_ping(&mut self, v: PacketPing) {
        self.sum = ::std::option::Option::Some(packet::Sum::PacketPing(v))
    }

    // Mutable pointer to the field.
    pub fn mut_packet_ping(&mut self) -> &mut PacketPing {
        if let ::std::option::Option::Some(packet::Sum::PacketPing(_)) = self.sum {
        } else {
            self.sum = ::std::option::Option::Some(packet::Sum::PacketPing(PacketPing::new()));
        }
        match self.sum {
            ::std::option::Option::Some(packet::Sum::PacketPing(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_packet_ping(&mut self) -> PacketPing {
        if self.has_packet_ping() {
            match self.sum.take() {
                ::std::option::Option::Some(packet::Sum::PacketPing(v)) => v,
                _ => panic!(),
            }
        } else {
            PacketPing::new()
        }
    }

    // .tendermint.p2p.PacketPong packet_pong = 2;

    pub fn packet_pong(&self) -> &PacketPong {
        match self.sum {
            ::std::option::Option::Some(packet::Sum::PacketPong(ref v)) => v,
            _ => <PacketPong as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_packet_pong(&mut self) {
        self.sum = ::std::option::Option::None;
    }

    pub fn has_packet_pong(&self) -> bool {
        match self.sum {
            ::std::option::Option::Some(packet::Sum::PacketPong(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_packet_pong(&mut self, v: PacketPong) {
        self.sum = ::std::option::Option::Some(packet::Sum::PacketPong(v))
    }

    // Mutable pointer to the field.
    pub fn mut_packet_pong(&mut self) -> &mut PacketPong {
        if let ::std::option::Option::Some(packet::Sum::PacketPong(_)) = self.sum {
        } else {
            self.sum = ::std::option::Option::Some(packet::Sum::PacketPong(PacketPong::new()));
        }
        match self.sum {
            ::std::option::Option::Some(packet::Sum::PacketPong(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_packet_pong(&mut self) -> PacketPong {
        if self.has_packet_pong() {
            match self.sum.take() {
                ::std::option::Option::Some(packet::Sum::PacketPong(v)) => v,
                _ => panic!(),
            }
        } else {
            PacketPong::new()
        }
    }

    // .tendermint.p2p.PacketMsg packet_msg = 3;

    pub fn packet_msg(&self) -> &PacketMsg {
        match self.sum {
            ::std::option::Option::Some(packet::Sum::PacketMsg(ref v)) => v,
            _ => <PacketMsg as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_packet_msg(&mut self) {
        self.sum = ::std::option::Option::None;
    }

    pub fn has_packet_msg(&self) -> bool {
        match self.sum {
            ::std::option::Option::Some(packet::Sum::PacketMsg(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_packet_msg(&mut self, v: PacketMsg) {
        self.sum = ::std::option::Option::Some(packet::Sum::PacketMsg(v))
    }

    // Mutable pointer to the field.
    pub fn mut_packet_msg(&mut self) -> &mut PacketMsg {
        if let ::std::option::Option::Some(packet::Sum::PacketMsg(_)) = self.sum {
        } else {
            self.sum = ::std::option::Option::Some(packet::Sum::PacketMsg(PacketMsg::new()));
        }
        match self.sum {
            ::std::option::Option::Some(packet::Sum::PacketMsg(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_packet_msg(&mut self) -> PacketMsg {
        if self.has_packet_msg() {
            match self.sum.take() {
                ::std::option::Option::Some(packet::Sum::PacketMsg(v)) => v,
                _ => panic!(),
            }
        } else {
            PacketMsg::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, PacketPing>(
            "packet_ping",
            Packet::has_packet_ping,
            Packet::packet_ping,
            Packet::mut_packet_ping,
            Packet::set_packet_ping,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, PacketPong>(
            "packet_pong",
            Packet::has_packet_pong,
            Packet::packet_pong,
            Packet::mut_packet_pong,
            Packet::set_packet_pong,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, PacketMsg>(
            "packet_msg",
            Packet::has_packet_msg,
            Packet::packet_msg,
            Packet::mut_packet_msg,
            Packet::set_packet_msg,
        ));
        oneofs.push(packet::Sum::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Packet>(
            "Packet",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Packet {
    const NAME: &'static str = "Packet";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.sum = ::std::option::Option::Some(packet::Sum::PacketPing(is.read_message()?));
                },
                18 => {
                    self.sum = ::std::option::Option::Some(packet::Sum::PacketPong(is.read_message()?));
                },
                26 => {
                    self.sum = ::std::option::Option::Some(packet::Sum::PacketMsg(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.sum {
            match v {
                &packet::Sum::PacketPing(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &packet::Sum::PacketPong(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &packet::Sum::PacketMsg(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.sum {
            match v {
                &packet::Sum::PacketPing(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
                &packet::Sum::PacketPong(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
                &packet::Sum::PacketMsg(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
                },
            };
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

    fn new() -> Packet {
        Packet::new()
    }

    fn clear(&mut self) {
        self.sum = ::std::option::Option::None;
        self.sum = ::std::option::Option::None;
        self.sum = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Packet {
        static instance: Packet = Packet {
            sum: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Packet {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Packet").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Packet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Packet {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `Packet`
pub mod packet {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:tendermint.p2p.Packet.sum)
    pub enum Sum {
        // @@protoc_insertion_point(oneof_field:tendermint.p2p.Packet.packet_ping)
        PacketPing(super::PacketPing),
        // @@protoc_insertion_point(oneof_field:tendermint.p2p.Packet.packet_pong)
        PacketPong(super::PacketPong),
        // @@protoc_insertion_point(oneof_field:tendermint.p2p.Packet.packet_msg)
        PacketMsg(super::PacketMsg),
    }

    impl ::protobuf::Oneof for Sum {
    }

    impl ::protobuf::OneofFull for Sum {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::Packet as ::protobuf::MessageFull>::descriptor().oneof_by_name("sum").unwrap()).clone()
        }
    }

    impl Sum {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Sum>("sum")
        }
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:tendermint.p2p.AuthSigMessage)
pub struct AuthSigMessage {
    // message fields
    // @@protoc_insertion_point(field:tendermint.p2p.AuthSigMessage.pub_key)
    pub pub_key: ::protobuf::MessageField<super::keys::PublicKey>,
    // @@protoc_insertion_point(field:tendermint.p2p.AuthSigMessage.sig)
    pub sig: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:tendermint.p2p.AuthSigMessage.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AuthSigMessage {
    fn default() -> &'a AuthSigMessage {
        <AuthSigMessage as ::protobuf::Message>::default_instance()
    }
}

impl AuthSigMessage {
    pub fn new() -> AuthSigMessage {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::keys::PublicKey>(
            "pub_key",
            |m: &AuthSigMessage| { &m.pub_key },
            |m: &mut AuthSigMessage| { &mut m.pub_key },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sig",
            |m: &AuthSigMessage| { &m.sig },
            |m: &mut AuthSigMessage| { &mut m.sig },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AuthSigMessage>(
            "AuthSigMessage",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AuthSigMessage {
    const NAME: &'static str = "AuthSigMessage";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pub_key)?;
                },
                18 => {
                    self.sig = is.read_bytes()?;
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
        if !self.sig.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.sig);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.pub_key.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.sig.is_empty() {
            os.write_bytes(2, &self.sig)?;
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

    fn new() -> AuthSigMessage {
        AuthSigMessage::new()
    }

    fn clear(&mut self) {
        self.pub_key.clear();
        self.sig.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AuthSigMessage {
        static instance: AuthSigMessage = AuthSigMessage {
            pub_key: ::protobuf::MessageField::none(),
            sig: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AuthSigMessage {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AuthSigMessage").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AuthSigMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthSigMessage {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19tendermint/p2p/conn.proto\x12\x0etendermint.p2p\x1a\x14gogoproto/g\
    ogo.proto\x1a\x1ctendermint/crypto/keys.proto\"\x0c\n\nPacketPing\"\x0c\
    \n\nPacketPong\"h\n\tPacketMsg\x12,\n\nchannel_id\x18\x01\x20\x01(\x05R\
    \tchannelIdB\r\xe2\xde\x1f\tChannelID\x12\x19\n\x03eof\x18\x02\x20\x01(\
    \x08R\x03eofB\x07\xe2\xde\x1f\x03EOF\x12\x12\n\x04data\x18\x03\x20\x01(\
    \x0cR\x04data\"\xc9\x01\n\x06Packet\x12=\n\x0bpacket_ping\x18\x01\x20\
    \x01(\x0b2\x1a.tendermint.p2p.PacketPingH\0R\npacketPing\x12=\n\x0bpacke\
    t_pong\x18\x02\x20\x01(\x0b2\x1a.tendermint.p2p.PacketPongH\0R\npacketPo\
    ng\x12:\n\npacket_msg\x18\x03\x20\x01(\x0b2\x19.tendermint.p2p.PacketMsg\
    H\0R\tpacketMsgB\x05\n\x03sum\"_\n\x0eAuthSigMessage\x12;\n\x07pub_key\
    \x18\x01\x20\x01(\x0b2\x1c.tendermint.crypto.PublicKeyR\x06pubKeyB\x04\
    \xc8\xde\x1f\0\x12\x10\n\x03sig\x18\x02\x20\x01(\x0cR\x03sigB7Z5github.c\
    om/tendermint/tendermint/proto/tendermint/p2pJ\xee\x05\n\x06\x12\x04\0\0\
    \x1d\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\0\
    \x17\n\x08\n\x01\x08\x12\x03\x03\0L\n\t\n\x02\x08\x0b\x12\x03\x03\0L\n\t\
    \n\x02\x03\0\x12\x03\x05\0\x1e\n\t\n\x02\x03\x01\x12\x03\x06\0&\n\t\n\
    \x02\x04\0\x12\x03\x08\0\x15\n\n\n\x03\x04\0\x01\x12\x03\x08\x08\x12\n\t\
    \n\x02\x04\x01\x12\x03\n\0\x15\n\n\n\x03\x04\x01\x01\x12\x03\n\x08\x12\n\
    \n\n\x02\x04\x02\x12\x04\x0c\0\x10\x01\n\n\n\x03\x04\x02\x01\x12\x03\x0c\
    \x08\x11\n\x0b\n\x04\x04\x02\x02\0\x12\x03\r\x02>\n\x0c\n\x05\x04\x02\
    \x02\0\x05\x12\x03\r\x02\x07\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\r\x08\
    \x12\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\r\x15\x16\n\x0c\n\x05\x04\x02\
    \x02\0\x08\x12\x03\r\x17=\n\x0f\n\x08\x04\x02\x02\0\x08\xec\xfb\x03\x12\
    \x03\r\x18<\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x0e\x028\n\x0c\n\x05\x04\
    \x02\x02\x01\x05\x12\x03\x0e\x02\x06\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\
    \x03\x0e\x08\x0b\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x0e\x15\x16\n\
    \x0c\n\x05\x04\x02\x02\x01\x08\x12\x03\x0e\x177\n\x0f\n\x08\x04\x02\x02\
    \x01\x08\xec\xfb\x03\x12\x03\x0e\x186\n\x0b\n\x04\x04\x02\x02\x02\x12\
    \x03\x0f\x02\x17\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03\x0f\x02\x07\n\
    \x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x0f\x08\x0c\n\x0c\n\x05\x04\x02\
    \x02\x02\x03\x12\x03\x0f\x15\x16\n\n\n\x02\x04\x03\x12\x04\x12\0\x18\x01\
    \n\n\n\x03\x04\x03\x01\x12\x03\x12\x08\x0e\n\x0c\n\x04\x04\x03\x08\0\x12\
    \x04\x13\x02\x17\x03\n\x0c\n\x05\x04\x03\x08\0\x01\x12\x03\x13\x08\x0b\n\
    \x0b\n\x04\x04\x03\x02\0\x12\x03\x14\x04\x1f\n\x0c\n\x05\x04\x03\x02\0\
    \x06\x12\x03\x14\x04\x0e\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x14\x0f\
    \x1a\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x14\x1d\x1e\n\x0b\n\x04\x04\
    \x03\x02\x01\x12\x03\x15\x04\x1f\n\x0c\n\x05\x04\x03\x02\x01\x06\x12\x03\
    \x15\x04\x0e\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03\x15\x0f\x1a\n\x0c\n\
    \x05\x04\x03\x02\x01\x03\x12\x03\x15\x1d\x1e\n\x0b\n\x04\x04\x03\x02\x02\
    \x12\x03\x16\x04\x1f\n\x0c\n\x05\x04\x03\x02\x02\x06\x12\x03\x16\x04\r\n\
    \x0c\n\x05\x04\x03\x02\x02\x01\x12\x03\x16\x0f\x19\n\x0c\n\x05\x04\x03\
    \x02\x02\x03\x12\x03\x16\x1d\x1e\n\n\n\x02\x04\x04\x12\x04\x1a\0\x1d\x01\
    \n\n\n\x03\x04\x04\x01\x12\x03\x1a\x08\x16\n\x0b\n\x04\x04\x04\x02\0\x12\
    \x03\x1b\x02I\n\x0c\n\x05\x04\x04\x02\0\x06\x12\x03\x1b\x02\x1d\n\x0c\n\
    \x05\x04\x04\x02\0\x01\x12\x03\x1b\x1e%\n\x0c\n\x05\x04\x04\x02\0\x03\
    \x12\x03\x1b()\n\x0c\n\x05\x04\x04\x02\0\x08\x12\x03\x1b*H\n\x0f\n\x08\
    \x04\x04\x02\0\x08\xe9\xfb\x03\x12\x03\x1b+G\n\x0b\n\x04\x04\x04\x02\x01\
    \x12\x03\x1c\x02*\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03\x1c\x02\x07\n\
    \x0c\n\x05\x04\x04\x02\x01\x01\x12\x03\x1c\x1e!\n\x0c\n\x05\x04\x04\x02\
    \x01\x03\x12\x03\x1c()b\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(5);
            messages.push(PacketPing::generated_message_descriptor_data());
            messages.push(PacketPong::generated_message_descriptor_data());
            messages.push(PacketMsg::generated_message_descriptor_data());
            messages.push(Packet::generated_message_descriptor_data());
            messages.push(AuthSigMessage::generated_message_descriptor_data());
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

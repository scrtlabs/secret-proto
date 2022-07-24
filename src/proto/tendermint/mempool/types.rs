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

//! Generated file from `tendermint/mempool/types.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:tendermint.mempool.Txs)
pub struct Txs {
    // message fields
    // @@protoc_insertion_point(field:tendermint.mempool.Txs.txs)
    pub txs: ::std::vec::Vec<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:tendermint.mempool.Txs.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Txs {
    fn default() -> &'a Txs {
        <Txs as ::protobuf::Message>::default_instance()
    }
}

impl Txs {
    pub fn new() -> Txs {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "txs",
            |m: &Txs| { &m.txs },
            |m: &mut Txs| { &mut m.txs },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Txs>(
            "Txs",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Txs {
    const NAME: &'static str = "Txs";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.txs.push(is.read_bytes()?);
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
        for value in &self.txs {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.txs {
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

    fn new() -> Txs {
        Txs::new()
    }

    fn clear(&mut self) {
        self.txs.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Txs {
        static instance: Txs = Txs {
            txs: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Txs {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Txs").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Txs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Txs {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:tendermint.mempool.Message)
pub struct Message {
    // message oneof groups
    pub sum: ::std::option::Option<message::Sum>,
    // special fields
    // @@protoc_insertion_point(special_field:tendermint.mempool.Message.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Message {
    fn default() -> &'a Message {
        <Message as ::protobuf::Message>::default_instance()
    }
}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    // .tendermint.mempool.Txs txs = 1;

    pub fn txs(&self) -> &Txs {
        match self.sum {
            ::std::option::Option::Some(message::Sum::Txs(ref v)) => v,
            _ => <Txs as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_txs(&mut self) {
        self.sum = ::std::option::Option::None;
    }

    pub fn has_txs(&self) -> bool {
        match self.sum {
            ::std::option::Option::Some(message::Sum::Txs(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_txs(&mut self, v: Txs) {
        self.sum = ::std::option::Option::Some(message::Sum::Txs(v))
    }

    // Mutable pointer to the field.
    pub fn mut_txs(&mut self) -> &mut Txs {
        if let ::std::option::Option::Some(message::Sum::Txs(_)) = self.sum {
        } else {
            self.sum = ::std::option::Option::Some(message::Sum::Txs(Txs::new()));
        }
        match self.sum {
            ::std::option::Option::Some(message::Sum::Txs(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_txs(&mut self) -> Txs {
        if self.has_txs() {
            match self.sum.take() {
                ::std::option::Option::Some(message::Sum::Txs(v)) => v,
                _ => panic!(),
            }
        } else {
            Txs::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, Txs>(
            "txs",
            Message::has_txs,
            Message::txs,
            Message::mut_txs,
            Message::set_txs,
        ));
        oneofs.push(message::Sum::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Message>(
            "Message",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Message {
    const NAME: &'static str = "Message";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.sum = ::std::option::Option::Some(message::Sum::Txs(is.read_message()?));
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
                &message::Sum::Txs(ref v) => {
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
                &message::Sum::Txs(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> Message {
        Message::new()
    }

    fn clear(&mut self) {
        self.sum = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Message {
        static instance: Message = Message {
            sum: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Message {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Message").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `Message`
pub mod message {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:tendermint.mempool.Message.sum)
    pub enum Sum {
        // @@protoc_insertion_point(oneof_field:tendermint.mempool.Message.txs)
        Txs(super::Txs),
    }

    impl ::protobuf::Oneof for Sum {
    }

    impl ::protobuf::OneofFull for Sum {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::Message as ::protobuf::MessageFull>::descriptor().oneof_by_name("sum").unwrap()).clone()
        }
    }

    impl Sum {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Sum>("sum")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1etendermint/mempool/types.proto\x12\x12tendermint.mempool\"\x17\n\
    \x03Txs\x12\x10\n\x03txs\x18\x01\x20\x03(\x0cR\x03txs\"=\n\x07Message\
    \x12+\n\x03txs\x18\x01\x20\x01(\x0b2\x17.tendermint.mempool.TxsH\0R\x03t\
    xsB\x05\n\x03sumB;Z9github.com/tendermint/tendermint/proto/tendermint/me\
    mpoolJ\xf9\x01\n\x06\x12\x04\0\0\r\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\
    \x08\n\x01\x02\x12\x03\x01\0\x1b\n\x08\n\x01\x08\x12\x03\x03\0P\n\t\n\
    \x02\x08\x0b\x12\x03\x03\0P\n\n\n\x02\x04\0\x12\x04\x05\0\x07\x01\n\n\n\
    \x03\x04\0\x01\x12\x03\x05\x08\x0b\n\x0b\n\x04\x04\0\x02\0\x12\x03\x06\
    \x02\x19\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x06\x02\n\n\x0c\n\x05\x04\0\
    \x02\0\x05\x12\x03\x06\x0b\x10\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x06\
    \x11\x14\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x06\x17\x18\n\n\n\x02\x04\
    \x01\x12\x04\t\0\r\x01\n\n\n\x03\x04\x01\x01\x12\x03\t\x08\x0f\n\x0c\n\
    \x04\x04\x01\x08\0\x12\x04\n\x02\x0c\x03\n\x0c\n\x05\x04\x01\x08\0\x01\
    \x12\x03\n\x08\x0b\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0b\x04\x10\n\x0c\n\
    \x05\x04\x01\x02\0\x06\x12\x03\x0b\x04\x07\n\x0c\n\x05\x04\x01\x02\0\x01\
    \x12\x03\x0b\x08\x0b\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0b\x0e\x0fb\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(Txs::generated_message_descriptor_data());
            messages.push(Message::generated_message_descriptor_data());
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
// This file is generated by rust-protobuf 2.0.5. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

pub mod exts {
    use protobuf::Message as Message_imported_for_functions;

    pub const expose_oneof_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17001, phantom: ::std::marker::PhantomData };

    pub const expose_fields_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const serde_derive_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17030, phantom: ::std::marker::PhantomData };

    pub const serde_derive_cfg_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 17031, phantom: ::std::marker::PhantomData };

    pub const expose_oneof: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17001, phantom: ::std::marker::PhantomData };

    pub const expose_fields: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const serde_derive: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17030, phantom: ::std::marker::PhantomData };

    pub const serde_derive_cfg: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 17031, phantom: ::std::marker::PhantomData };

    pub const expose_fields_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0frustproto.proto\x12\trustproto\x1a\x20google/protobuf/descriptor.p\
    roto:H\n\x10expose_oneof_all\x18\xe9\x84\x01\x20\x01(\x08\x12\x1c.google\
    .protobuf.FileOptionsR\x0eexposeOneofAll:J\n\x11expose_fields_all\x18\
    \xeb\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x0fexpose\
    FieldsAll:T\n\x16generate_accessors_all\x18\xec\x84\x01\x20\x01(\x08\x12\
    \x1c.google.protobuf.FileOptionsR\x14generateAccessorsAll:b\n\x1ecarller\
    che_bytes_for_bytes_all\x18\xf3\x84\x01\x20\x01(\x08\x12\x1c.google.prot\
    obuf.FileOptionsR\x1acarllercheBytesForBytesAll:d\n\x1fcarllerche_bytes_\
    for_string_all\x18\xf4\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.File\
    OptionsR\x1bcarllercheBytesForStringAll:H\n\x10serde_derive_all\x18\x86\
    \x85\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x0eserdeDeriv\
    eAll:O\n\x14serde_derive_cfg_all\x18\x87\x85\x01\x20\x01(\t\x12\x1c.goog\
    le.protobuf.FileOptionsR\x11serdeDeriveCfgAll:D\n\x0cexpose_oneof\x18\
    \xe9\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x0bexp\
    oseOneof:F\n\rexpose_fields\x18\xeb\x84\x01\x20\x01(\x08\x12\x1f.google.\
    protobuf.MessageOptionsR\x0cexposeFields:P\n\x12generate_accessors\x18\
    \xec\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x11gen\
    erateAccessors:^\n\x1acarllerche_bytes_for_bytes\x18\xf3\x84\x01\x20\x01\
    (\x08\x12\x1f.google.protobuf.MessageOptionsR\x17carllercheBytesForBytes\
    :`\n\x1bcarllerche_bytes_for_string\x18\xf4\x84\x01\x20\x01(\x08\x12\x1f\
    .google.protobuf.MessageOptionsR\x18carllercheBytesForString:D\n\x0cserd\
    e_derive\x18\x86\x85\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOpt\
    ionsR\x0bserdeDerive:K\n\x10serde_derive_cfg\x18\x87\x85\x01\x20\x01(\t\
    \x12\x1f.google.protobuf.MessageOptionsR\x0eserdeDeriveCfg:O\n\x13expose\
    _fields_field\x18\xeb\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.Field\
    OptionsR\x11exposeFieldsField:Y\n\x18generate_accessors_field\x18\xec\
    \x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x16generateA\
    ccessorsField:g\n\x20carllerche_bytes_for_bytes_field\x18\xf3\x84\x01\
    \x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x1ccarllercheBytesFo\
    rBytesField:i\n!carllerche_bytes_for_string_field\x18\xf4\x84\x01\x20\
    \x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x1dcarllercheBytesForStr\
    ingFieldJ\xe6\x12\n\x06\x12\x04\0\04\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\
    \n\t\n\x02\x03\0\x12\x03\x02\x07)\nh\n\x01\x02\x12\x03\x07\x08\x112^\x20\
    see\x20https://github.com/gogo/protobuf/blob/master/gogoproto/gogo.proto\
    \n\x20for\x20the\x20original\x20idea\n\n\t\n\x01\x07\x12\x04\t\0\x18\x01\
    \n7\n\x02\x07\0\x12\x03\x0b\x04+\x1a,\x20When\x20true,\x20oneof\x20field\
    \x20is\x20generated\x20public\n\n\n\n\x03\x07\0\x02\x12\x03\t\x07\"\n\n\
    \n\x03\x07\0\x04\x12\x03\x0b\x04\x0c\n\n\n\x03\x07\0\x05\x12\x03\x0b\r\
    \x11\n\n\n\x03\x07\0\x01\x12\x03\x0b\x12\"\n\n\n\x03\x07\0\x03\x12\x03\
    \x0b%*\nI\n\x02\x07\x01\x12\x03\r\x04,\x1a>\x20When\x20true\x20all\x20fi\
    elds\x20are\x20public,\x20and\x20not\x20accessors\x20generated\n\n\n\n\
    \x03\x07\x01\x02\x12\x03\t\x07\"\n\n\n\x03\x07\x01\x04\x12\x03\r\x04\x0c\
    \n\n\n\x03\x07\x01\x05\x12\x03\r\r\x11\n\n\n\x03\x07\x01\x01\x12\x03\r\
    \x12#\n\n\n\x03\x07\x01\x03\x12\x03\r&+\nP\n\x02\x07\x02\x12\x03\x0f\x04\
    1\x1aE\x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20acce\
    ssors\x20are\x20not\x20generated\n\n\n\n\x03\x07\x02\x02\x12\x03\t\x07\"\
    \n\n\n\x03\x07\x02\x04\x12\x03\x0f\x04\x0c\n\n\n\x03\x07\x02\x05\x12\x03\
    \x0f\r\x11\n\n\n\x03\x07\x02\x01\x12\x03\x0f\x12(\n\n\n\x03\x07\x02\x03\
    \x12\x03\x0f+0\n2\n\x02\x07\x03\x12\x03\x11\x049\x1a'\x20Use\x20`bytes::\
    Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\x03\x07\x03\x02\x12\x03\t\x07\
    \"\n\n\n\x03\x07\x03\x04\x12\x03\x11\x04\x0c\n\n\n\x03\x07\x03\x05\x12\
    \x03\x11\r\x11\n\n\n\x03\x07\x03\x01\x12\x03\x11\x120\n\n\n\x03\x07\x03\
    \x03\x12\x03\x1138\n3\n\x02\x07\x04\x12\x03\x13\x04:\x1a(\x20Use\x20`byt\
    es::Bytes`\x20for\x20`string`\x20fields\n\n\n\n\x03\x07\x04\x02\x12\x03\
    \t\x07\"\n\n\n\x03\x07\x04\x04\x12\x03\x13\x04\x0c\n\n\n\x03\x07\x04\x05\
    \x12\x03\x13\r\x11\n\n\n\x03\x07\x04\x01\x12\x03\x13\x121\n\n\n\x03\x07\
    \x04\x03\x12\x03\x1349\nJ\n\x02\x07\x05\x12\x03\x15\x04+\x1a?\x20Use\x20\
    `serde_derive`\x20to\x20implement\x20`Serialize`\x20and\x20`Deserialize`\
    \n\n\n\n\x03\x07\x05\x02\x12\x03\t\x07\"\n\n\n\x03\x07\x05\x04\x12\x03\
    \x15\x04\x0c\n\n\n\x03\x07\x05\x05\x12\x03\x15\r\x11\n\n\n\x03\x07\x05\
    \x01\x12\x03\x15\x12\"\n\n\n\x03\x07\x05\x03\x12\x03\x15%*\n3\n\x02\x07\
    \x06\x12\x03\x17\x041\x1a(\x20Guard\x20serde\x20annotations\x20with\x20c\
    fg\x20attr.\n\n\n\n\x03\x07\x06\x02\x12\x03\t\x07\"\n\n\n\x03\x07\x06\
    \x04\x12\x03\x17\x04\x0c\n\n\n\x03\x07\x06\x05\x12\x03\x17\r\x13\n\n\n\
    \x03\x07\x06\x01\x12\x03\x17\x14(\n\n\n\x03\x07\x06\x03\x12\x03\x17+0\n\
    \t\n\x01\x07\x12\x04\x1a\0)\x01\n7\n\x02\x07\x07\x12\x03\x1c\x04'\x1a,\
    \x20When\x20true,\x20oneof\x20field\x20is\x20generated\x20public\n\n\n\n\
    \x03\x07\x07\x02\x12\x03\x1a\x07%\n\n\n\x03\x07\x07\x04\x12\x03\x1c\x04\
    \x0c\n\n\n\x03\x07\x07\x05\x12\x03\x1c\r\x11\n\n\n\x03\x07\x07\x01\x12\
    \x03\x1c\x12\x1e\n\n\n\x03\x07\x07\x03\x12\x03\x1c!&\nI\n\x02\x07\x08\
    \x12\x03\x1e\x04(\x1a>\x20When\x20true\x20all\x20fields\x20are\x20public\
    ,\x20and\x20not\x20accessors\x20generated\n\n\n\n\x03\x07\x08\x02\x12\
    \x03\x1a\x07%\n\n\n\x03\x07\x08\x04\x12\x03\x1e\x04\x0c\n\n\n\x03\x07\
    \x08\x05\x12\x03\x1e\r\x11\n\n\n\x03\x07\x08\x01\x12\x03\x1e\x12\x1f\n\n\
    \n\x03\x07\x08\x03\x12\x03\x1e\"'\nP\n\x02\x07\t\x12\x03\x20\x04-\x1aE\
    \x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20accessors\
    \x20are\x20not\x20generated\n\n\n\n\x03\x07\t\x02\x12\x03\x1a\x07%\n\n\n\
    \x03\x07\t\x04\x12\x03\x20\x04\x0c\n\n\n\x03\x07\t\x05\x12\x03\x20\r\x11\
    \n\n\n\x03\x07\t\x01\x12\x03\x20\x12$\n\n\n\x03\x07\t\x03\x12\x03\x20',\
    \n2\n\x02\x07\n\x12\x03\"\x045\x1a'\x20Use\x20`bytes::Bytes`\x20for\x20`\
    bytes`\x20fields\n\n\n\n\x03\x07\n\x02\x12\x03\x1a\x07%\n\n\n\x03\x07\n\
    \x04\x12\x03\"\x04\x0c\n\n\n\x03\x07\n\x05\x12\x03\"\r\x11\n\n\n\x03\x07\
    \n\x01\x12\x03\"\x12,\n\n\n\x03\x07\n\x03\x12\x03\"/4\n3\n\x02\x07\x0b\
    \x12\x03$\x046\x1a(\x20Use\x20`bytes::Bytes`\x20for\x20`string`\x20field\
    s\n\n\n\n\x03\x07\x0b\x02\x12\x03\x1a\x07%\n\n\n\x03\x07\x0b\x04\x12\x03\
    $\x04\x0c\n\n\n\x03\x07\x0b\x05\x12\x03$\r\x11\n\n\n\x03\x07\x0b\x01\x12\
    \x03$\x12-\n\n\n\x03\x07\x0b\x03\x12\x03$05\nJ\n\x02\x07\x0c\x12\x03&\
    \x04'\x1a?\x20Use\x20`serde_derive`\x20to\x20implement\x20`Serialize`\
    \x20and\x20`Deserialize`\n\n\n\n\x03\x07\x0c\x02\x12\x03\x1a\x07%\n\n\n\
    \x03\x07\x0c\x04\x12\x03&\x04\x0c\n\n\n\x03\x07\x0c\x05\x12\x03&\r\x11\n\
    \n\n\x03\x07\x0c\x01\x12\x03&\x12\x1e\n\n\n\x03\x07\x0c\x03\x12\x03&!&\n\
    3\n\x02\x07\r\x12\x03(\x04-\x1a(\x20Guard\x20serde\x20annotations\x20wit\
    h\x20cfg\x20attr.\n\n\n\n\x03\x07\r\x02\x12\x03\x1a\x07%\n\n\n\x03\x07\r\
    \x04\x12\x03(\x04\x0c\n\n\n\x03\x07\r\x05\x12\x03(\r\x13\n\n\n\x03\x07\r\
    \x01\x12\x03(\x14$\n\n\n\x03\x07\r\x03\x12\x03(',\n\t\n\x01\x07\x12\x04+\
    \04\x01\nI\n\x02\x07\x0e\x12\x03-\x04.\x1a>\x20When\x20true\x20all\x20fi\
    elds\x20are\x20public,\x20and\x20not\x20accessors\x20generated\n\n\n\n\
    \x03\x07\x0e\x02\x12\x03+\x07#\n\n\n\x03\x07\x0e\x04\x12\x03-\x04\x0c\n\
    \n\n\x03\x07\x0e\x05\x12\x03-\r\x11\n\n\n\x03\x07\x0e\x01\x12\x03-\x12%\
    \n\n\n\x03\x07\x0e\x03\x12\x03-(-\nP\n\x02\x07\x0f\x12\x03/\x043\x1aE\
    \x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20accessors\
    \x20are\x20not\x20generated\n\n\n\n\x03\x07\x0f\x02\x12\x03+\x07#\n\n\n\
    \x03\x07\x0f\x04\x12\x03/\x04\x0c\n\n\n\x03\x07\x0f\x05\x12\x03/\r\x11\n\
    \n\n\x03\x07\x0f\x01\x12\x03/\x12*\n\n\n\x03\x07\x0f\x03\x12\x03/-2\n2\n\
    \x02\x07\x10\x12\x031\x04;\x1a'\x20Use\x20`bytes::Bytes`\x20for\x20`byte\
    s`\x20fields\n\n\n\n\x03\x07\x10\x02\x12\x03+\x07#\n\n\n\x03\x07\x10\x04\
    \x12\x031\x04\x0c\n\n\n\x03\x07\x10\x05\x12\x031\r\x11\n\n\n\x03\x07\x10\
    \x01\x12\x031\x122\n\n\n\x03\x07\x10\x03\x12\x0315:\n3\n\x02\x07\x11\x12\
    \x033\x04<\x1a(\x20Use\x20`bytes::Bytes`\x20for\x20`string`\x20fields\n\
    \n\n\n\x03\x07\x11\x02\x12\x03+\x07#\n\n\n\x03\x07\x11\x04\x12\x033\x04\
    \x0c\n\n\n\x03\x07\x11\x05\x12\x033\r\x11\n\n\n\x03\x07\x11\x01\x12\x033\
    \x123\n\n\n\x03\x07\x11\x03\x12\x0336;\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}

use descriptor::*;
use misc::*;
use core::*;
use rt;

fn rust_name(field_type: FieldDescriptorProto_Type) -> &'static str {
    match field_type {
        TYPE_DOUBLE   => "f64",
        TYPE_FLOAT    => "f32",
        TYPE_INT32    => "i32",
        TYPE_INT64    => "i64",
        TYPE_UINT32   => "u32",
        TYPE_UINT64   => "u64",
        TYPE_SINT32   => "i32",
        TYPE_SINT64   => "i64",
        TYPE_FIXED32  => "u32",
        TYPE_FIXED64  => "u64",
        TYPE_SFIXED32 => "s32",
        TYPE_SFIXED64 => "s64",
        TYPE_BOOL     => "bool",
        TYPE_STRING   => "~str",
        TYPE_BYTES    => "~[u8]",
        TYPE_ENUM | TYPE_GROUP | TYPE_MESSAGE => fail!()
    }
}

fn protobuf_name(field_type: FieldDescriptorProto_Type) -> &'static str {
    match field_type {
        TYPE_DOUBLE   => "double",
        TYPE_FLOAT    => "float",
        TYPE_INT32    => "int32",
        TYPE_INT64    => "int64",
        TYPE_UINT32   => "uint32",
        TYPE_UINT64   => "uint64",
        TYPE_SINT32   => "sint32",
        TYPE_SINT64   => "sint64",
        TYPE_FIXED32  => "fixed32",
        TYPE_FIXED64  => "fixed64",
        TYPE_SFIXED32 => "sfixed32",
        TYPE_SFIXED64 => "sfixed64",
        TYPE_BOOL     => "bool",
        TYPE_STRING   => "string",
        TYPE_BYTES    => "bytes",
        TYPE_ENUM | TYPE_GROUP | TYPE_MESSAGE => fail!()
    }
}

fn field_type_wire_type(field_type: FieldDescriptorProto_Type) -> wire_format::WireType {
    use core::wire_format::*;
    match field_type {
        TYPE_INT32    => WireTypeVarint,
        TYPE_INT64    => WireTypeVarint,
        TYPE_UINT32   => WireTypeVarint,
        TYPE_UINT64   => WireTypeVarint,
        TYPE_SINT32   => WireTypeVarint,
        TYPE_SINT64   => WireTypeVarint,
        TYPE_BOOL     => WireTypeVarint,
        TYPE_ENUM     => WireTypeVarint,
        TYPE_FIXED32  => WireTypeFixed32,
        TYPE_FIXED64  => WireTypeFixed64,
        TYPE_SFIXED32 => WireTypeFixed32,
        TYPE_SFIXED64 => WireTypeFixed64,
        TYPE_FLOAT    => WireTypeFixed32,
        TYPE_DOUBLE   => WireTypeFixed64,
        TYPE_STRING   => WireTypeLengthDelimited,
        TYPE_BYTES    => WireTypeLengthDelimited,
        TYPE_MESSAGE  => WireTypeLengthDelimited,
        TYPE_GROUP    => fail!()
    }
}

fn field_type_size(field_type: FieldDescriptorProto_Type) -> Option<u32> {
    match field_type {
        TYPE_BOOL => Some(1),
        t if field_type_wire_type(t) == wire_format::WireTypeFixed32 => Some(4),
        t if field_type_wire_type(t) == wire_format::WireTypeFixed64 => Some(8),
        _ => None
    }
}

fn field_type_name(field: &FieldDescriptorProto, pkg: &str) -> ~str {
    match field.type_name {
        Some(ref type_name) => {
            let current_pkg_prefix = "." + pkg + ".";
            if (*type_name).starts_with(current_pkg_prefix) {
                remove_prefix(*type_name, current_pkg_prefix).to_owned()
            } else {
                remove_to(*type_name, '.').to_owned()
            }
        },
        None =>
            rust_name(field.field_type.get()).to_owned()
    }
}

#[deriving(Clone)]
enum RepeatMode {
    Single,
    RepeatRegular,
    RepeatPacked,
}

#[deriving(Clone)]
struct Field {
    proto_field: FieldDescriptorProto,
    name: ~str,
    field_type: FieldDescriptorProto_Type,
    wire_type: wire_format::WireType,
    type_name: ~str,
    number: u32,
    repeated: bool,
    packed: bool,
    repeat_mode: RepeatMode,
}

impl Field {
    fn parse(field: &FieldDescriptorProto, pkg: &str) -> Option<Field> {
        let type_name = field_type_name(field, pkg).replace(".", "_");
        let repeated = match field.label.get() {
            LABEL_REPEATED => true,
            LABEL_OPTIONAL | LABEL_REQUIRED => false,
        };
        let name = match field.name.get_ref().to_owned() {
            ~"type" => ~"field_type",
            x => x,
        };
        let packed = match field.options {
            Some(ref options) => options.packed.get_or_default(false),
            None => false
        };
        let repeat_mode =
            if repeated {
                if packed { RepeatPacked } else { RepeatRegular }
            } else {
                Single
            };
        Some(Field {
            proto_field: field.clone(),
            name: name,
            field_type: field.field_type.get(),
            wire_type: field_type_wire_type(field.field_type.get()),
            type_name: type_name,
            number: field.number.get() as u32,
            repeated: repeated,
            packed: packed,
            repeat_mode: repeat_mode,
        })
    }
}

#[deriving(Clone)]
struct Message {
    proto_message: DescriptorProto,
    pkg: ~str,
    prefix: ~str,
    type_name: ~str,
    fields: ~[Field],
}

impl<'self> Message {
    fn parse(proto_message: &DescriptorProto, pkg: &str, prefix: &str) -> Message {
        Message {
            proto_message: proto_message.clone(),
            pkg: pkg.to_owned(),
            prefix: prefix.to_owned(),
            type_name: prefix + proto_message.name.get_ref().to_owned(),
            fields: do proto_message.field.flat_map |field| {
                match Field::parse(field, pkg) {
                    Some(field) => ~[field],
                    None => ~[]
                }
            },
        }
    }

    fn has_any_message_field(&self) -> bool {
        for self.fields.iter().advance |field| {
            if field.field_type == TYPE_MESSAGE {
                return true;
            }
        }
        false
    }

    fn required_fields(&'self self) -> ~[&'self Field] {
        let mut r = ~[];
        for self.fields.iter().advance |field| {
            if field.proto_field.label.get() == LABEL_REQUIRED {
                r.push(field);
            }
        }
        r
    }
}


struct IndentWriter {
    writer: @Writer,
    indent: ~str,
    // TODO: refs
    msg: Option<Message>,
    field: Option<Field>,
}

impl IndentWriter {
    fn new(writer: @Writer) -> IndentWriter {
        IndentWriter {
            writer: writer,
            indent: ~"",
            msg: None,
            field: None,
        }
    }

    fn bind_message(&self, msg: &Message, cb: &fn(&IndentWriter)) {
        cb(&IndentWriter {
            writer: self.writer,
            indent: self.indent.to_owned(),
            msg: Some(msg.clone()),
            field: None,
        });
    }

    fn bind_field(&self, field: &Field, cb: &fn(&IndentWriter)) {
        //assert!(self.msg.is_some());
        cb(&IndentWriter {
            writer: self.writer,
            indent: self.indent.to_owned(),
            msg: self.msg.clone(),
            field: Some(field.clone()),
        });
    }

    fn fields(&self, cb: &fn(&IndentWriter)) {
        let fields = self.msg.get_ref().fields.to_owned();
        let mut iter = fields.iter();
        for iter.advance |field| {
            self.bind_field(field, |w| cb(w));
        }
    }

    fn write_line(&self, line: &str) {
        if line.is_empty() {
            self.writer.write_line("")
        } else {
            self.writer.write_line(self.indent + line);
        }
    }

    fn write_lines(&self, lines: &[~str]) {
        for lines.iter().advance |line| {
            self.write_line(*line);
        }
    }

    fn indent(&self) -> IndentWriter {
        IndentWriter {
            writer: self.writer,
            indent: self.indent + "    ",
            msg: self.msg.clone(),
            field: self.field.clone(),
        }
    }

    fn indented(&self, cb: &fn(&IndentWriter)) {
        let next = self.indent();
        cb(&next);
    }

    fn commented(&self, cb: &fn(&IndentWriter)) {
        cb(&IndentWriter {
            writer: self.writer,
            indent: "// " + self.indent,
            msg: self.msg.clone(),
            field: self.field.clone(),
        });
    }

    fn block(&self, first_line: &str, last_line: &str, cb: &fn(&IndentWriter)) {
        self.write_line(first_line);
        self.indented(cb);
        self.write_line(last_line);
    }

    fn expr_block(&self, prefix: &str, cb: &fn(&IndentWriter)) {
        self.block(prefix + " {", "}", cb);
    }

    fn stmt_block(&self, prefix: &str, cb: &fn(&IndentWriter)) {
        self.block(prefix + " {", "};", cb);
    }

    fn impl_block(&self, name: &str, cb: &fn(&IndentWriter)) {
        self.expr_block(fmt!("impl %s", name), cb);
    }

    fn impl_for_block(&self, tr: &str, ty: &str, cb: &fn(&IndentWriter)) {
        self.expr_block(fmt!("impl %s for %s", tr, ty), cb);
    }

    fn pub_struct(&self, name: &str, cb: &fn(&IndentWriter)) {
        self.expr_block("pub struct " + name, cb);
    }

    fn def_struct(&self, name: &str, cb: &fn(&IndentWriter)) {
        self.expr_block("struct " + name, cb);
    }

    fn def_mod(&self, name: &str, cb: &fn(&IndentWriter)) {
        self.expr_block("mod " + name, cb);
    }

    fn field(&self, name: &str, value: &str) {
        self.write_line(fmt!("%s: %s,", name, value));
    }

    fn fail(&self) {
        self.write_line("fail!()");
    }

    fn todo(&self) {
        self.write_line("fail!(\"TODO\");");
    }

    fn comment(&self, comment: &str) {
        if comment.is_empty() {
            self.write_line("//");
        } else {
            self.write_line("// " + comment);
        }
    }

    fn pub_fn(&self, sig: &str, cb: &fn(&IndentWriter)) {
        self.expr_block(fmt!("pub fn %s", sig), cb);
    }

    fn def_fn(&self, sig: &str, cb: &fn(&IndentWriter)) {
        self.expr_block(fmt!("fn %s", sig), cb);
    }

    fn while_block(&self, cond: &str, cb: &fn(&IndentWriter)) {
        self.expr_block(fmt!("while %s", cond), cb);
    }

    fn if_stmt(&self, cond: &str, cb: &fn(&IndentWriter)) {
        self.stmt_block(fmt!("if %s", cond), cb);
    }

    fn for_stmt(&self, over: &str, varn: &str, cb: &fn(&IndentWriter)) {
        self.stmt_block(fmt!("for %s |%s|", over, varn), cb);
    }

    fn match_block(&self, value: &str, cb: &fn(&IndentWriter)) {
        self.stmt_block(fmt!("match %s", value), cb);
    }

    fn match_expr(&self, value: &str, cb: &fn(&IndentWriter)) {
        self.expr_block(fmt!("match %s", value), cb);
    }

    fn case_block(&self, cond: &str, cb: &fn(&IndentWriter)) {
        self.block(fmt!("%s => {", cond), "},", cb);
    }

    fn case_expr(&self, cond: &str, body: &str) {
        self.write_line(fmt!("%s => %s,", cond, body));
    }

    fn clear_field_func(&self) -> ~str {
        "clear_" + self.field.get_ref().name
    }

    fn clear_field(&self) {
        if self.field.get_ref().repeated {
            self.write_line(fmt!("self.%s.clear();", self.field.get_ref().name));
        } else {
            self.write_line(fmt!("self.%s = None;", self.field.get_ref().name));
        }
    }

}

fn write_merge_from_field(field: &Field, w: &IndentWriter) {
    let wire_type = field_type_wire_type(field.field_type);
    let repeat_mode =
        if field.repeated {
            if wire_type == wire_format::WireTypeLengthDelimited {
                RepeatRegular
            } else {
                RepeatPacked // may be both regular or packed
            }
        } else {
            Single
        };

    let read_proc = match field.field_type {
        TYPE_MESSAGE => None,
        TYPE_ENUM => Some(fmt!("%s::new(is.read_int32())", field.type_name)),
        t => Some(fmt!("is.read_%s()", protobuf_name(t))),
    };

    match repeat_mode {
        Single | RepeatRegular => {
            w.write_line(fmt!("assert_eq!(wire_format::%?, wire_type);", wire_type));
            match field.field_type {
                TYPE_MESSAGE => {
                    w.write_line(fmt!("let mut tmp = %s::new();", field.type_name));
                    w.write_line(fmt!("is.merge_message(&mut tmp);"));
                },
                _ => {
                    w.write_line(fmt!("let tmp = %s;", *read_proc.get_ref()));
                },
            };
            match repeat_mode {
                Single => w.write_line(fmt!("self.%s = Some(tmp);", field.name)),
                RepeatRegular => w.write_line(fmt!("self.%s.push(tmp);", field.name)),
                _ => fail!()
            }
        },
        RepeatPacked => {
            w.write_line(fmt!("if wire_type == wire_format::%? {", wire_format::WireTypeLengthDelimited));
            do w.indented |w| {
                w.write_line("let len = is.read_raw_varint32();");
                w.write_line("let old_limit = is.push_limit(len);");
                do w.while_block("!is.eof()") |w| {
                    w.write_line(fmt!("self.%s.push(%s);", field.name, *read_proc.get_ref()));
                };
                w.write_line("is.pop_limit(old_limit);");
            }
            w.write_line("} else {");
            do w.indented |w| {
                w.write_line(fmt!("assert_eq!(wire_format::%?, wire_type);", wire_type));
                w.write_line(fmt!("self.%s.push(%s);", field.name, *read_proc.get_ref()));
            }
            w.write_line("}");
        },
    };
}

fn write_message(msg: &Message, w: &IndentWriter) {
    let pkg = msg.pkg.as_slice();
    let message_type = &msg.proto_message;

    do w.bind_message(msg) |w| {
        w.write_line(fmt!("#[deriving(Clone,Eq)]"));
        do w.pub_struct(msg.type_name) |w| {
            for msg.fields.iter().advance |field| {
                if field.type_name.contains_char('.') {
                    loop;
                }
                let full_type = match field.repeated {
                    true  => fmt!("~[%s]", field.type_name),
                    false => fmt!("Option<%s>", field.type_name),
                };
                w.field(field.name, full_type);
            }
            if msg.fields.is_empty() {
                w.field("dummy", "bool");
            }
        }

        w.write_line("");

        do w.impl_block(msg.type_name) |w| {
            do w.pub_fn(fmt!("new() -> %s", msg.type_name)) |w| {
                do w.expr_block(msg.type_name) |w| {
                    for msg.fields.iter().advance |field| {
                        let init = match field.repeated {
                            true  => ~"~[]",
                            false => ~"None",
                        };
                        w.field(field.name, init);
                    }
                    if msg.fields.is_empty() {
                        w.field("dummy", "false");
                    }
                }
            }
            w.write_line("");
            if !msg.has_any_message_field() {
                // `sizes` and `sizes_pos` are unused
                w.write_line("#[allow(unused_variable)]");
            }
            do w.pub_fn("write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint)") |w| {
                for msg.fields.iter().advance |field| {
                    let field_type = field.field_type;
                    let write_method_suffix = match field_type {
                        TYPE_MESSAGE => "message",
                        TYPE_ENUM => "enum",
                        t => protobuf_name(t),
                    };
                    let field_number = field.proto_field.number.get();
                    let vv = match field.field_type {
                        TYPE_MESSAGE => "v", // TODO: as &Message
                        TYPE_ENUM => "*v as i32",
                        _ => "*v",
                    };
                    let write_value_lines = match field.field_type {
                        TYPE_MESSAGE => ~[
                            fmt!("os.write_tag(%d, wire_format::%?);",
                                    field_number as int, wire_format::WireTypeLengthDelimited),
                            fmt!("os.write_raw_varint32(sizes[*sizes_pos]);"),
                            fmt!("*sizes_pos += 1;"),
                            fmt!("v.write_to_with_computed_sizes(os, sizes, sizes_pos);"),
                        ],
                        _ => ~[
                            fmt!("os.write_%s(%d, %s);", write_method_suffix, field_number as int, vv),
                        ],
                    };
                    match field.repeat_mode {
                        Single => {
                            do w.match_block(fmt!("self.%s", field.name)) |w| {
                                do w.case_block("Some(ref v)") |w| {
                                    w.write_lines(write_value_lines);
                                };
                                w.case_expr("None", "{}");
                            }
                        },
                        RepeatPacked => {
                            do w.if_stmt(fmt!("!self.%s.is_empty()", field.name)) |w| {
                                w.write_line(fmt!("os.write_tag(%d, wire_format::%?);", field_number as int, wire_format::WireTypeLengthDelimited));
                                w.write_line(fmt!("os.write_raw_varint32(rt::vec_packed_data_size(self.%s, wire_format::%?));", field.name, field_type_wire_type(field.field_type)));
                                do w.for_stmt(fmt!("self.%s.iter().advance", field.name), "v") |w| {
                                    w.write_line(fmt!("os.write_%s_no_tag(%s);", write_method_suffix, vv));
                                }
                            }
                        },
                        RepeatRegular => {
                            do w.for_stmt(fmt!("self.%s.iter().advance", field.name), "v") |w| {
                                w.write_lines(write_value_lines);
                            }
                        },
                    };
                }
            }
            do w.fields |w| {
                w.write_line("");
                do w.pub_fn(fmt!("%s(&mut self)", w.clear_field_func())) |w| {
                    w.clear_field();
                }

                // TODO: set_
                // TODO: has_
                // TODO: mut_
                // TODO: add_
            }
        }

        w.write_line("");

        do w.impl_for_block("Message", msg.type_name) |w| {
            do w.def_fn(fmt!("new() -> %s", msg.type_name)) |w| {
                w.write_line(fmt!("%s::new()", msg.type_name));
            }
            w.write_line("");
            do w.def_fn("clear(&mut self)") |w| {
                do w.fields |w| {
                    w.write_line(fmt!("self.%s();", w.clear_field_func()));
                }
            }
            w.write_line("");
            do w.def_fn(fmt!("is_initialized(&self) -> bool")) |w| {
                let required_fields = msg.required_fields();
                for required_fields.iter().advance |field| {
                    do w.if_stmt(fmt!("self.%s.is_none()", field.name)) |w| {
                        w.write_line("return false;");
                    }
                }
                w.write_line("true");
            }
            w.write_line("");
            do w.def_fn(fmt!("merge_from(&mut self, is: &mut CodedInputStream)")) |w| {
                do w.while_block("!is.eof()") |w| {
                    w.write_line(fmt!("let (field_number, wire_type) = is.read_tag_unpack();"));
                    do w.match_block("field_number") |w| {
                        for msg.fields.iter().advance |field| {
                            do w.case_block(field.number.to_str()) |w| {
                                write_merge_from_field(field, w);
                            }
                        }
                        do w.case_block("_") |w| {
                            w.write_line(fmt!("// TODO: store in unknown fields"));
                            w.write_line(fmt!("is.skip_field(wire_type);"));
                        }
                    }
                }
            }
            w.write_line("");
            // Append sizes of messages in the tree to the specified vector.
            // First appended element is size of self, and then nested message sizes.
            // in serialization order are appended recursively.");
            w.comment("Compute sizes of nested messages");
            do w.def_fn("compute_sizes(&self, sizes: &mut ~[u32]) -> u32") |w| {
                w.write_line("let pos = sizes.len();");
                w.write_line("sizes.push(0);");
                w.write_line("let mut my_size = 0;");
                for msg.fields.iter().advance |field| {
                    match field.repeat_mode {
                        Single | RepeatRegular => {
                            match field_type_size(field.field_type) {
                                Some(s) => {
                                    if field.repeated {
                                        w.write_line(fmt!(
                                                "my_size += %d * self.%s.len();",
                                                (s + rt::tag_size(field.number)) as int,
                                                field.name));
                                    } else {
                                        do w.if_stmt(fmt!("self.%s.is_some()", field.name)) |w| {
                                            w.write_line(fmt!(
                                                    "my_size += %d;",
                                                    (s + rt::tag_size(field.number)) as int));
                                        }
                                    }
                                },
                                None => {
                                    do w.for_stmt(fmt!("self.%s.iter().advance", field.name), "value") |w| {
                                        match field.field_type {
                                            TYPE_MESSAGE => {
                                                w.write_line("let len = value.compute_sizes(sizes);");
                                                w.write_line(fmt!(
                                                        "my_size += %u + rt::compute_raw_varint32_size(len) + len;",
                                                        rt::tag_size(field.number) as uint));
                                            },
                                            TYPE_BYTES | TYPE_STRING => {
                                                let pn = protobuf_name(field.field_type);
                                                w.write_line(fmt!(
                                                        "my_size += rt::%s_size(%d, *value);",
                                                        pn,
                                                        field.number as int));
                                            },
                                            TYPE_ENUM => {
                                                w.write_line(fmt!(
                                                        "my_size += rt::enum_size(%d, *value);",
                                                        field.number as int));
                                            },
                                            _ => {
                                                w.write_line(fmt!(
                                                        "my_size += rt::value_size(%d, *value, wire_format::%?);",
                                                        field.number as int, field.wire_type));
                                            },
                                        }
                                    }
                                },
                            };
                        },
                        RepeatPacked => {
                            w.write_line(fmt!(
                                    "my_size += rt::vec_packed_size(%d, self.%s, wire_format::%?);",
                                    field.number as int, field.name, field.wire_type));
                        },
                    };
                }
                w.write_line("sizes[pos] = my_size;");
                w.comment("value is returned for convenience");
                w.write_line("my_size");
            }
            w.write_line("");
            do w.pub_fn("write_to(&self, os: &mut CodedOutputStream)") |w| {
                w.write_line("self.check_initialized();");
                w.write_line("let mut sizes: ~[u32] = ~[];");
                w.write_line("self.compute_sizes(&mut sizes);");
                w.write_line("let mut sizes_pos = 1; // first element is self");
                w.write_line("self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);");
                w.write_line("assert_eq!(sizes_pos, sizes.len());");
            }
        }

        for message_type.nested_type.iter().advance |nested_type| {
            w.write_line("");
            write_message(&Message::parse(nested_type, pkg, msg.type_name + "_"), w);
        }

        for message_type.enum_type.iter().advance |enum_type| {
            w.write_line("");
            write_enum(msg.type_name + "_", w, enum_type);
        }
    }
}

fn write_enum(prefix: &str, w: &IndentWriter, enum_type: &EnumDescriptorProto) {
    let enum_type_name = prefix + enum_type.name.get_ref().to_owned();
    w.write_line(fmt!("#[deriving(Clone,Eq)]"));
    w.write_line(fmt!("pub enum %s {", enum_type_name));
    for enum_type.value.iter().advance |value| {
        w.write_line(fmt!("    %s = %d,", value.name.get_ref().to_owned(), value.number.get() as int));
    }
    w.write_line(fmt!("}"));
    w.write_line("");
    do w.impl_block(enum_type_name) |w| {
        do w.pub_fn(fmt!("new(value: i32) -> %s", enum_type_name)) |w| {
            do w.match_expr("value") |w| {
                for enum_type.value.iter().advance |value| {
                    let value_number = value.number.get();
                    let value_name = value.name.get_ref().to_owned();
                    w.write_line(fmt!("%d => %s,", value_number as int, value_name));
                }
                w.write_line(fmt!("_ => fail!()"));
            }
        }
    }
    w.write_line("");
    do w.impl_for_block("ProtobufEnum", enum_type_name) |w| {
        do w.pub_fn("value(&self) -> i32") |w| {
            w.write_line("*self as i32")
        }
    }
}

fn remove_to<'s>(s: &'s str, c: char) -> &'s str {
    match s.rfind(c) {
        Some(pos) => s.slice_from(pos + 1),
        None => s
    }
}

fn remove_from(s: &str, c: char) -> ~str {
    match s.find(c) {
        Some(pos) => s.slice_to(pos).to_owned(),
        None => s.to_owned()
    }
}

fn remove_suffix<'s>(s: &'s str, suffix: &str) -> &'s str {
    if !s.ends_with(suffix) {
        fail!();
    }
    s.slice_to(s.len() - suffix.len())
}

fn remove_prefix<'s>(s: &'s str, prefix: &str) -> &'s str {
    if !s.starts_with(prefix) {
        fail!();
    }
    s.slice_from(prefix.len())
}

fn remove_prefix_if_present<'s>(s: &'s str, prefix: &str) -> &'s str {
    if s.starts_with(prefix) {
        remove_prefix(s, prefix)
    } else {
        s
    }
}


fn last_part_of_package<'s>(pkg: &'s str) -> &'s str {
    remove_to(pkg, '.')
}

fn proto_path_to_rust_base<'s>(path: &'s str) -> &'s str {
    remove_suffix(remove_to(path, '/'), ".proto")
}

struct GenResult {
    name: ~str,
    content: ~[u8],
}

struct GenOptions {
    dummy: bool,
}

pub fn gen(files: &[FileDescriptorProto], _: &GenOptions) -> ~[GenResult] {
    let mut results: ~[GenResult] = ~[];
    for files.iter().advance |file| {
        let base = proto_path_to_rust_base(*file.name.get_ref());

        let os0 = VecWriter::new();
        let os = os0 as @Writer;

        let w = IndentWriter::new(os);

        w.write_line("// This file is generated. Do not edit");
        w.write_line("");

        w.write_line("use protobuf::*;");
        w.write_line("use protobuf::rt;");
        for file.dependency.iter().advance |dep| {
            w.write_line(fmt!("use %s::*;", proto_path_to_rust_base(*dep)));
        }

        for file.message_type.iter().advance |message_type| {
            w.write_line("");
            write_message(&Message::parse(message_type, *file.package.get_ref(), ""), &w);
        }
        for file.enum_type.iter().advance |enum_type| {
            w.write_line("");
            write_enum("", &w, enum_type);
        }

        results.push(GenResult {
            name: base + ".rs",
            content: os0.vec.to_owned(),
        });
    }
    results
}

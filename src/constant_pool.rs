pub const CP_UTF8: u8 = 1;
pub const CP_INTEGER: u8 = 3;
pub const CP_FLOAT: u8 = 4;
pub const CP_LONG: u8 = 5;
pub const CP_DOUBLE: u8 = 6;
pub const CP_CLASS: u8 = 7;
pub const CP_STRING: u8 = 8;
pub const CP_FIELDREF: u8 = 9;
pub const CP_METHODREF: u8 = 10;
pub const CP_INTERFACE_METHODREF: u8 = 11;
pub const CP_NAME_AND_TYPE: u8 = 12;

#[derive(Debug)]
pub enum CpEntry {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    // stores a valid index into the constant pool containing an UTF8 entry
    Class {
        name_index: u16,
    },
    // stores a valid index into the constant pool containing an UTF8 entry
    String {
        string_index: u16,
    },
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
}

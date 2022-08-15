use crate::{
    class::Class,
    constant_pool::{
        CpEntry, CP_CLASS, CP_DOUBLE, CP_FIELDREF, CP_FLOAT, CP_INTERFACE_METHODREF, CP_LONG,
        CP_METHODREF, CP_NAME_AND_TYPE, CP_UTF8,
    },
    constant_pool::{CP_INTEGER, CP_STRING},
};
use byteorder::{BigEndian, ByteOrder};

pub struct ClassfileParser {
    filename: String,
    file_bytes: Vec<u8>,
    current_idx: usize,

    minor_v: u16,
    major_v: u16,
    cp_count: u16,
}

impl ClassfileParser {
    pub fn of(filename: String, classfile_bytes: Vec<u8>) -> ClassfileParser {
        ClassfileParser {
            filename,
            file_bytes: classfile_bytes,
            current_idx: 0,
            minor_v: 0,
            major_v: 0,
            cp_count: 0,
        }
    }

    pub fn parse(&mut self) -> Class {
        self.parse_header();
        let cp = self.parse_constant_pool();

        println!("Minor v: {}", self.minor_v);
        println!("Major v: {}", self.major_v);
        println!("CP count: {}", self.cp_count);
        println!("CP: \n{:?}", cp);

        Class::of()
    }

    fn parse_header(&mut self) -> () {
        // check the magic number
        if self.file_bytes[0] != 0xca
            || self.file_bytes[1] != 0xfe
            || self.file_bytes[2] != 0xba
            || self.file_bytes[3] != 0xbe
        {
            panic!(
                "The input file {} isn't a valid Java class file. It doesn't contain a correct magic number!",
                self.filename
            )
        }

        self.current_idx = 3;

        self.minor_v = self.read_and_merge_next_two_bytes();
        self.major_v = self.read_and_merge_next_two_bytes();
        self.cp_count = self.read_and_merge_next_two_bytes();
    }

    fn parse_constant_pool(&mut self) -> Vec<CpEntry> {
        let mut cp_entries: Vec<CpEntry> = Vec::with_capacity(self.cp_count as usize);

        let mut cp_idx: u16 = 0;
        while cp_idx < self.cp_count {
            self.current_idx += 1;
            let tag = self.file_bytes[self.current_idx];

            // If current cp entry is a double width value, we need to jump to cp_idx + 2
            let mut is_cp_entry_double_width = false;

            let cp_entry = match tag {
                CP_UTF8 => {
                    let string_len = self.read_and_merge_next_two_bytes() as usize;
                    let mut string_bytes = Vec::with_capacity(string_len);

                    for _ in 0..string_len {
                        self.current_idx += 1;
                        string_bytes.push(self.file_bytes[self.current_idx]);
                    }

                    let parsed_string = String::from_utf8(string_bytes).expect(&format!(
                        "Error while parsing UTF-8 string from the constnat pool at index {}",
                        cp_idx
                    ));

                    CpEntry::Utf8(parsed_string)
                }
                CP_INTEGER => {
                    let bytes = self.read_next_four_bytes();
                    CpEntry::Integer(BigEndian::read_i32(&bytes))
                }
                CP_FLOAT => {
                    let bytes = self.read_next_four_bytes();
                    CpEntry::Float(BigEndian::read_f32(&bytes))
                }
                CP_LONG => {
                    let high_bytes = self.read_next_four_bytes();
                    let low_bytes = self.read_next_four_bytes();

                    let bytes = [high_bytes, low_bytes].concat().as_slice().to_owned();

                    is_cp_entry_double_width = true;

                    CpEntry::Long(BigEndian::read_i64(&bytes))
                }
                CP_DOUBLE => {
                    let high_bytes = self.read_next_four_bytes();
                    let low_bytes = self.read_next_four_bytes();

                    let bytes = [high_bytes, low_bytes].concat().as_slice().to_owned();

                    is_cp_entry_double_width = true;

                    CpEntry::Double(BigEndian::read_f64(&bytes))
                }
                CP_CLASS => {
                    let index_to_name = self.read_and_merge_next_two_bytes();
                    CpEntry::Class {
                        name_index: index_to_name,
                    }
                }
                CP_STRING => {
                    let utf8_ref = self.read_and_merge_next_two_bytes();
                    CpEntry::String {
                        string_index: utf8_ref,
                    }
                }
                CP_FIELDREF => {
                    let class_index = self.read_and_merge_next_two_bytes();
                    let name_and_type_index = self.read_and_merge_next_two_bytes();

                    CpEntry::FieldRef {
                        class_index,
                        name_and_type_index,
                    }
                }
                CP_METHODREF => {
                    let class_index = self.read_and_merge_next_two_bytes();
                    let name_and_type_index = self.read_and_merge_next_two_bytes();

                    CpEntry::MethodRef {
                        class_index,
                        name_and_type_index,
                    }
                }
                CP_INTERFACE_METHODREF => {
                    let class_index = self.read_and_merge_next_two_bytes();
                    let name_and_type_index = self.read_and_merge_next_two_bytes();

                    CpEntry::InterfaceMethodRef {
                        class_index,
                        name_and_type_index,
                    }
                }
                CP_NAME_AND_TYPE => {
                    let name_index = self.read_and_merge_next_two_bytes();
                    let descriptor_index = self.read_and_merge_next_two_bytes();

                    CpEntry::NameAndType {
                        name_index,
                        descriptor_index,
                    }
                }
                _ => panic!(
                    "Unsupported constant pool type {} at CP position {}. Byte index {} of {}",
                    tag, cp_idx, self.current_idx, self.filename
                ),
            };

            cp_entries.push(cp_entry);

            if is_cp_entry_double_width {
                cp_idx += 2;
            } else {
                cp_idx += 1;
            }
        }

        return cp_entries;
    }

    // Reads the next 2 bytes of file_bytes, concats them as one u16 and move current_idx by 2 positions
    // leaves the index on the position of the last byte of these 2
    fn read_and_merge_next_two_bytes(&mut self) -> u16 {
        let result = ((self.file_bytes[self.current_idx + 1] as u16) << 8)
            + self.file_bytes[self.current_idx + 2] as u16;
        self.current_idx += 2;

        return result;
    }

    // Read the next 4 bytes of file_bytes and return them as an array of 4 eements
    // moves current_idx by 4 positions, leaves it on the position of the last byte of these 4
    fn read_next_four_bytes(&mut self) -> [u8; 4] {
        let mut bytes = [0; 4];

        for i in 0..4 {
            self.current_idx += 1;
            bytes[i] = self.file_bytes[self.current_idx];
        }
        return bytes;
    }
}

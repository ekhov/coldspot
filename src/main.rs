mod class;
mod classfile_parser;
mod constant_pool;

use classfile_parser::ClassfileParser;
use std::{env, fs::File, io, io::Read, path::Path};

fn main() {
    let classfile_name = get_class_name();
    let classfile_bytes =
        read_as_bytes(&classfile_name).expect(&format!("Cannot read {}", &classfile_name));

    let mut parser = ClassfileParser::of(classfile_name, classfile_bytes);

    parser.parse();
}

fn read_as_bytes(file_name: &String) -> Result<Vec<u8>, io::Error> {
    let path = Path::new(file_name);

    File::open(path).and_then(|mut file| {
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        Ok(bytes)
    })
}

fn get_class_name() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name_opt: Option<&String> = args.get(1);

    match file_name_opt {
        Some(file_name) => return file_name.to_owned(),
        None => panic!("Class name must be passed as a first argument!"),
    }
}

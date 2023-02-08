use std::{
    fs::File,
    io::{self, BufReader, Cursor, Read, Write},
    path::Path,
    str::from_utf8,
};

use crate::{
    deserialize::{
        json::{FromJson, JsonDeserializer},
        Deserialize,
    },
    error::{AnyError, IoResult},
    serialize::{
        json::{IntoJson, JsonSerializer, PrettyJsonFormatter},
        Serialize,
    },
};

pub trait Persistant: Sized + Serialize + Deserialize {
    fn save(&self, path: &str) -> Result<(), AnyError>;
    fn load(path: &str) -> Result<Self, AnyError>;
}

fn serialize_with_default<S: Serialize>(value: &S) -> Result<Vec<u8>, AnyError> {
    let mut buffer = Vec::with_capacity(128);
    let mut ser = DefaultSerializer::new(PrettyJsonFormatter::new("\t".to_string()), &mut buffer);
    value.serialize(&mut ser, &crate::position::Position::Closing)?;
    Ok(buffer)
}

fn deserialize_with_default<D: Deserialize>(value: &[u8]) -> Result<D, AnyError> {
    let mut cursor = Cursor::new(value);
    let mut des = DefaultDeserializer::new(&mut cursor);
    D::deserialize(&mut des)
}

fn serialize_factory<S: Serialize>(value: &S, path: &Path) -> Result<Vec<u8>, AnyError> {
    if let Some(ending) = path.extension() {
        if ending == "json" {
            return IntoJson::to_json(value).map(|json| json.into_bytes());
        }
    }
    serialize_with_default(value)
}

fn deserializer_factory<D: Deserialize>(value: &[u8], path: &Path) -> Result<D, AnyError> {
    if let Some(ending) = path.extension() {
        if ending == "json" {
            return FromJson::from_json(
                &from_utf8(value).expect("failed to convert content to utf8"),
            );
        }
    }
    deserialize_with_default(value)
}

type DefaultSerializer<'w> = JsonSerializer<'w, Vec<u8>, PrettyJsonFormatter>;
type DefaultDeserializer<'w> = JsonDeserializer<'w, Cursor<&'w [u8]>>;

impl<T: Sized + Serialize + Deserialize> Persistant for T {
    fn save(&self, path: &str) -> Result<(), AnyError> {
        let path = Path::new(path);
        let buffer = serialize_factory(self, path)?;
        write_bytes_file(buffer.as_slice(), path).map_err(|e| e.into())
    }

    fn load(path: &str) -> Result<Self, AnyError> {
        let path = Path::new(path);
        let content = read_bytes_file(path)?;
        deserializer_factory(content.as_slice(), path)
    }
}

fn write_bytes_file(bytes: &[u8], path: &Path) -> IoResult {
    let mut f = File::create(path)?;
    f.write_all(bytes)?;
    Ok(())
}

fn read_bytes_file(path: &Path) -> Result<Vec<u8>, io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}

#[cfg(test)]
mod test {
    use std::{
        fs::{self},
        path::Path,
    };

    use super::Persistant;

    struct FileLifetime {
        pub(crate) path: String,
    }

    impl Drop for FileLifetime {
        fn drop(&mut self) {
            let path = Path::new(&self.path);
            if path.exists() {
                if !path.is_dir() {
                    fs::remove_file(&path)
                        .expect(format!("failed to delete file: {}", self.path).as_str());
                } else {
                    fs::remove_dir_all(&path)
                        .expect(format!("failed to delete directory: {}", self.path).as_str());
                }
            }
        }
    }

    #[test]
    fn save_and_then_load_works() {
        let file_lifetime = FileLifetime {
            path: "save_i32.json".to_string(),
        };
        let data = 32i32;

        let saved = data.save(&file_lifetime.path);

        assert!(saved.is_ok());
        assert!(Path::new(&file_lifetime.path).exists());

        let loaded = i32::load(&file_lifetime.path);

        assert!(loaded.is_ok());
        assert_eq!(data, loaded.unwrap());
    }
}

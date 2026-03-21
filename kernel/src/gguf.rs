//! GGUF Model File Parser
//! 
//! Parses the GGUF format used by llama.cpp, Ollama, and most local LLMs.
//! Pure Rust, no_std compatible.
//! 
//! GGUF spec: https://github.com/ggerganov/ggml/blob/master/docs/gguf.md

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub const GGUF_MAGIC: u32 = 0x46475547; // "GGUF" in little-endian

#[derive(Debug)]
pub struct GgufFile {
    pub version: u32,
    pub tensor_count: u64,
    pub metadata: Vec<MetadataKV>,
    pub tensors: Vec<TensorInfo>,
    pub data_offset: usize,
}

#[derive(Debug, Clone)]
pub struct MetadataKV {
    pub key: String,
    pub value: MetadataValue,
}

#[derive(Debug, Clone)]
pub enum MetadataValue {
    Uint8(u8),
    Int8(i8),
    Uint16(u16),
    Int16(i16),
    Uint32(u32),
    Int32(i32),
    Float32(f32),
    Bool(bool),
    Str(String),
    Array(Vec<MetadataValue>),
    Uint64(u64),
    Int64(i64),
    Float64(f64),
}

#[derive(Debug, Clone)]
pub struct TensorInfo {
    pub name: String,
    pub n_dims: u32,
    pub dimensions: Vec<u64>,
    pub tensor_type: TensorType,
    pub offset: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum TensorType {
    F32 = 0,
    F16 = 1,
    Q4_0 = 2,
    Q4_1 = 3,
    Q5_0 = 6,
    Q5_1 = 7,
    Q8_0 = 8,
    Q8_1 = 9,
    Q2_K = 10,
    Q3_K = 11,
    Q4_K = 12,
    Q5_K = 13,
    Q6_K = 14,
    IQ2_XXS = 15,
    IQ2_XS = 16,
    IQ3_XXS = 17,
    IQ1_S = 18,
    IQ4_NL = 19,
    IQ3_S = 20,
    IQ2_S = 21,
    IQ4_XS = 22,
    Unknown = 255,
}

impl TensorType {
    fn from_u32(v: u32) -> Self {
        match v {
            0 => Self::F32, 1 => Self::F16, 2 => Self::Q4_0, 3 => Self::Q4_1,
            6 => Self::Q5_0, 7 => Self::Q5_1, 8 => Self::Q8_0, 9 => Self::Q8_1,
            10 => Self::Q2_K, 11 => Self::Q3_K, 12 => Self::Q4_K, 13 => Self::Q5_K,
            14 => Self::Q6_K, 15 => Self::IQ2_XXS, 16 => Self::IQ2_XS, 17 => Self::IQ3_XXS,
            18 => Self::IQ1_S, 19 => Self::IQ4_NL, 20 => Self::IQ3_S, 21 => Self::IQ2_S,
            22 => Self::IQ4_XS,
            _ => Self::Unknown,
        }
    }

    /// Bytes per element (for non-quantized types)
    pub fn element_size(&self) -> f32 {
        match self {
            Self::F32 => 4.0,
            Self::F16 => 2.0,
            Self::Q4_0 => 0.5 + 2.0/32.0,  // 4 bits + scale per block of 32
            Self::Q4_1 => 0.5 + 4.0/32.0,
            Self::Q8_0 => 1.0 + 2.0/32.0,
            Self::Q8_1 => 1.0 + 4.0/32.0,
            Self::Q2_K => 0.3125,
            Self::Q3_K => 0.4375,
            Self::Q4_K => 0.5625,
            Self::Q5_K => 0.6875,
            Self::Q6_K => 0.8125,
            _ => 1.0, // Default estimate
        }
    }
}

/// GGUF metadata value types
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
enum GgufValueType {
    Uint8 = 0,
    Int8 = 1,
    Uint16 = 2,
    Int16 = 3,
    Uint32 = 4,
    Int32 = 5,
    Float32 = 6,
    Bool = 7,
    Str = 8,
    Array = 9,
    Uint64 = 10,
    Int64 = 11,
    Float64 = 12,
}

/// Cursor for reading from a byte slice
struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Reader { data, pos: 0 }
    }

    fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    fn read_u8(&mut self) -> Option<u8> {
        if self.pos >= self.data.len() { return None; }
        let v = self.data[self.pos];
        self.pos += 1;
        Some(v)
    }

    fn read_u16(&mut self) -> Option<u16> {
        if self.pos + 2 > self.data.len() { return None; }
        let v = u16::from_le_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;
        Some(v)
    }

    fn read_u32(&mut self) -> Option<u32> {
        if self.pos + 4 > self.data.len() { return None; }
        let v = u32::from_le_bytes([
            self.data[self.pos], self.data[self.pos + 1],
            self.data[self.pos + 2], self.data[self.pos + 3],
        ]);
        self.pos += 4;
        Some(v)
    }

    fn read_u64(&mut self) -> Option<u64> {
        if self.pos + 8 > self.data.len() { return None; }
        let v = u64::from_le_bytes([
            self.data[self.pos], self.data[self.pos + 1],
            self.data[self.pos + 2], self.data[self.pos + 3],
            self.data[self.pos + 4], self.data[self.pos + 5],
            self.data[self.pos + 6], self.data[self.pos + 7],
        ]);
        self.pos += 8;
        Some(v)
    }

    fn read_i8(&mut self) -> Option<i8> { self.read_u8().map(|v| v as i8) }
    fn read_i16(&mut self) -> Option<i16> { self.read_u16().map(|v| v as i16) }
    fn read_i32(&mut self) -> Option<i32> { self.read_u32().map(|v| v as i32) }
    fn read_i64(&mut self) -> Option<i64> { self.read_u64().map(|v| v as i64) }

    fn read_f32(&mut self) -> Option<f32> {
        self.read_u32().map(|v| f32::from_bits(v))
    }

    fn read_f64(&mut self) -> Option<f64> {
        self.read_u64().map(|v| f64::from_bits(v))
    }

    fn read_string(&mut self) -> Option<String> {
        let len = self.read_u64()? as usize;
        if self.pos + len > self.data.len() { return None; }
        let s = core::str::from_utf8(&self.data[self.pos..self.pos + len]).ok()?;
        self.pos += len;
        Some(String::from(s))
    }

    fn read_value(&mut self) -> Option<MetadataValue> {
        let vtype = self.read_u32()?;
        match vtype {
            0 => self.read_u8().map(MetadataValue::Uint8),
            1 => self.read_i8().map(MetadataValue::Int8),
            2 => self.read_u16().map(MetadataValue::Uint16),
            3 => self.read_i16().map(MetadataValue::Int16),
            4 => self.read_u32().map(MetadataValue::Uint32),
            5 => self.read_i32().map(MetadataValue::Int32),
            6 => self.read_f32().map(MetadataValue::Float32),
            7 => self.read_u8().map(|v| MetadataValue::Bool(v != 0)),
            8 => self.read_string().map(MetadataValue::Str),
            9 => {
                // Array: element_type (u32) + count (u64) + elements
                let elem_type = self.read_u32()?;
                let count = self.read_u64()? as usize;
                let mut items = Vec::with_capacity(count.min(1024));
                for _ in 0..count {
                    // Read elements of the specified type
                    let val = match elem_type {
                        0 => self.read_u8().map(MetadataValue::Uint8),
                        1 => self.read_i8().map(MetadataValue::Int8),
                        2 => self.read_u16().map(MetadataValue::Uint16),
                        3 => self.read_i16().map(MetadataValue::Int16),
                        4 => self.read_u32().map(MetadataValue::Uint32),
                        5 => self.read_i32().map(MetadataValue::Int32),
                        6 => self.read_f32().map(MetadataValue::Float32),
                        7 => self.read_u8().map(|v| MetadataValue::Bool(v != 0)),
                        8 => self.read_string().map(MetadataValue::Str),
                        10 => self.read_u64().map(MetadataValue::Uint64),
                        11 => self.read_i64().map(MetadataValue::Int64),
                        12 => self.read_f64().map(MetadataValue::Float64),
                        _ => None,
                    }?;
                    items.push(val);
                }
                Some(MetadataValue::Array(items))
            }
            10 => self.read_u64().map(MetadataValue::Uint64),
            11 => self.read_i64().map(MetadataValue::Int64),
            12 => self.read_f64().map(MetadataValue::Float64),
            _ => None,
        }
    }
}

/// Parse a GGUF file from a byte slice
pub fn parse(data: &[u8]) -> Result<GgufFile, &'static str> {
    let mut reader = Reader::new(data);

    // Magic
    let magic = reader.read_u32().ok_or("Failed to read magic")?;
    if magic != GGUF_MAGIC {
        return Err("Invalid GGUF magic number");
    }

    // Version
    let version = reader.read_u32().ok_or("Failed to read version")?;
    if version < 2 || version > 3 {
        return Err("Unsupported GGUF version (need v2 or v3)");
    }

    // Counts
    let tensor_count = reader.read_u64().ok_or("Failed to read tensor count")?;
    let metadata_count = reader.read_u64().ok_or("Failed to read metadata count")?;

    // Metadata
    let mut metadata = Vec::with_capacity(metadata_count as usize);
    for _ in 0..metadata_count {
        let key = reader.read_string().ok_or("Failed to read metadata key")?;
        let value = reader.read_value().ok_or("Failed to read metadata value")?;
        metadata.push(MetadataKV { key, value });
    }

    // Tensor info
    let mut tensors = Vec::with_capacity(tensor_count as usize);
    for _ in 0..tensor_count {
        let name = reader.read_string().ok_or("Failed to read tensor name")?;
        let n_dims = reader.read_u32().ok_or("Failed to read tensor dims")?;
        let mut dimensions = Vec::with_capacity(n_dims as usize);
        for _ in 0..n_dims {
            dimensions.push(reader.read_u64().ok_or("Failed to read dimension")?);
        }
        let ttype = reader.read_u32().ok_or("Failed to read tensor type")?;
        let offset = reader.read_u64().ok_or("Failed to read tensor offset")?;

        tensors.push(TensorInfo {
            name,
            n_dims,
            dimensions,
            tensor_type: TensorType::from_u32(ttype),
            offset,
        });
    }

    // Alignment padding to data section
    let alignment = 32usize; // GGUF default alignment
    let data_offset = (reader.pos + alignment - 1) & !(alignment - 1);

    Ok(GgufFile {
        version,
        tensor_count,
        metadata,
        tensors,
        data_offset,
    })
}

/// Get a metadata value by key
impl GgufFile {
    pub fn get_metadata(&self, key: &str) -> Option<&MetadataValue> {
        self.metadata.iter().find(|kv| kv.key == key).map(|kv| &kv.value)
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        match self.get_metadata(key) {
            Some(MetadataValue::Str(s)) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn get_u32(&self, key: &str) -> Option<u32> {
        match self.get_metadata(key) {
            Some(MetadataValue::Uint32(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn get_u64(&self, key: &str) -> Option<u64> {
        match self.get_metadata(key) {
            Some(MetadataValue::Uint64(v)) => Some(*v),
            _ => None,
        }
    }

    /// Get model architecture (e.g., "llama", "phi", "qwen2")
    pub fn architecture(&self) -> Option<&str> {
        self.get_str("general.architecture")
    }

    /// Get model name
    pub fn name(&self) -> Option<&str> {
        self.get_str("general.name")
    }

    /// Get total parameter count estimate
    pub fn estimated_parameters(&self) -> u64 {
        self.tensors.iter()
            .map(|t| t.dimensions.iter().product::<u64>())
            .sum()
    }

    /// Get total model size in bytes (weights only)
    pub fn estimated_size_bytes(&self) -> u64 {
        self.tensors.iter()
            .map(|t| {
                let elements: u64 = t.dimensions.iter().product();
                (elements as f64 * t.tensor_type.element_size() as f64) as u64
            })
            .sum()
    }
}

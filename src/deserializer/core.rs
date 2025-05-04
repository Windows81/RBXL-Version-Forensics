use std::{
    io::{self, Read, Write},
    mem,
};

pub static FILE_MAGIC_HEADER: &[u8] = b"<roblox!";
pub static FILE_SIGNATURE: &[u8] = b"\x89\xff\x0d\x0a\x1a\x0a";
pub const FILE_VERSION: u16 = 0;

pub trait RbxReadExt: Read {
    fn read_le_u32(&mut self) -> io::Result<u32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;

        Ok(u32::from_le_bytes(buffer))
    }

    fn read_le_u16(&mut self) -> io::Result<u16> {
        let mut bytes = [0; 2];
        self.read_exact(&mut bytes)?;

        Ok(u16::from_le_bytes(bytes))
    }

    fn read_le_i16(&mut self) -> io::Result<i16> {
        let mut bytes = [0; 2];
        self.read_exact(&mut bytes)?;

        Ok(i16::from_le_bytes(bytes))
    }

    fn read_le_f32(&mut self) -> io::Result<f32> {
        let mut buffer = [0u8; 4];
        self.read_exact(&mut buffer)?;

        Ok(f32::from_le_bytes(buffer))
    }

    fn read_le_f64(&mut self) -> io::Result<f64> {
        let mut bytes = [0; 8];
        self.read_exact(&mut bytes)?;

        Ok(f64::from_le_bytes(bytes))
    }

    fn read_be_u32(&mut self) -> io::Result<u32> {
        let mut bytes = [0; 4];
        self.read_exact(&mut bytes)?;

        Ok(u32::from_be_bytes(bytes))
    }

    fn read_be_i64(&mut self) -> io::Result<i64> {
        let mut bytes = [0; 8];
        self.read_exact(&mut bytes)?;

        Ok(i64::from_be_bytes(bytes))
    }

    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buffer = [0u8];
        self.read_exact(&mut buffer)?;

        Ok(buffer[0])
    }

    /// Read a binary "string" in the format that Roblox's model files use.
    ///
    /// This function is safer than read_string because Roblox generally makes
    /// no guarantees about encoding of things it calls strings. rbx_binary
    /// makes a semantic differentiation between strings and binary buffers,
    /// which makes it more strict than Roblox but more likely to be correct.
    fn read_binary_string(&mut self) -> io::Result<Vec<u8>> {
        let length = self.read_le_u32()?;

        let mut value = Vec::with_capacity(length as usize);
        self.take(length as u64).read_to_end(&mut value)?;

        Ok(value)
    }

    /// Read a UTF-8 encoded string encoded how Roblox model files encode
    /// strings. This function isn't always appropriate because Roblox's formats
    /// generally aren't dilligent about data being valid Unicode.
    fn read_string(&mut self) -> io::Result<String> {
        let length = self.read_le_u32()?;
        let mut value = String::with_capacity(length as usize);
        self.take(length as u64).read_to_string(&mut value)?;

        Ok(value)
    }

    fn read_bool(&mut self) -> io::Result<bool> {
        Ok(self.read_u8()? != 0)
    }

    /// Fills `output` with blocks of `N` bytes from the buffer,
    /// deinterleaving them in the process.
    ///
    /// This function allocates `N * output.len()` bytes before reading.
    fn read_interleaved_bytes<const N: usize>(&mut self, output: &mut [[u8; N]]) -> io::Result<()> {
        let len = output.len();
        let mut buffer = vec![0; len * N];
        self.read_exact(&mut buffer)?;

        for (i, array) in output.iter_mut().enumerate() {
            for (j, byte) in array.iter_mut().enumerate() {
                *byte = buffer[i + len * j];
            }
        }

        Ok(())
    }

    /// Fills `output` with big-endian `i32` values read from the buffer.
    /// These values are untransformed while being read.
    fn read_interleaved_i32_array(&mut self, output: &mut [i32]) -> io::Result<()> {
        let mut read = vec![[0; mem::size_of::<i32>()]; output.len()];
        self.read_interleaved_bytes(&mut read)?;

        for (chunk, out) in read.into_iter().zip(output) {
            *out = untransform_i32(i32::from_be_bytes(chunk));
        }

        Ok(())
    }

    /// Fills `output` with big-endian `u32` values read from the buffer.
    fn read_interleaved_u32_array(&mut self, output: &mut [u32]) -> io::Result<()> {
        let mut read = vec![[0; mem::size_of::<u32>()]; output.len()];
        self.read_interleaved_bytes(&mut read)?;

        for (chunk, out) in read.into_iter().zip(output) {
            *out = u32::from_be_bytes(chunk);
        }

        Ok(())
    }

    /// Fills `output` with big-endian `f32` values read from the buffer.
    /// These values are properly unrotated while being read.
    fn read_interleaved_f32_array(&mut self, output: &mut [f32]) -> io::Result<()> {
        let mut read = vec![[0; mem::size_of::<u32>()]; output.len()];
        self.read_interleaved_bytes(&mut read)?;

        for (chunk, out) in read.into_iter().zip(output) {
            *out = f32::from_bits(u32::from_be_bytes(chunk).rotate_right(1));
        }

        Ok(())
    }

    /// Fills `output` with big-endian `i32` values read from the buffer.
    /// The values are properly untransformed and accumulated so as to properly
    /// read arrays of referent values.
    fn read_referent_array(&mut self, output: &mut [i32]) -> io::Result<()> {
        self.read_interleaved_i32_array(output)?;

        let mut last = 0;

        for referent in output.iter_mut() {
            *referent += last;
            last = *referent;
        }

        Ok(())
    }

    /// Fills `output` with big-endian `64` values read from the buffer.
    /// These values are untransformed while being read.
    fn read_interleaved_i64_array(&mut self, output: &mut [i64]) -> io::Result<()> {
        let mut read = vec![[0; mem::size_of::<i64>()]; output.len()];
        self.read_interleaved_bytes(&mut read)?;

        for (chunk, out) in read.into_iter().zip(output) {
            *out = untransform_i64(i64::from_be_bytes(chunk));
        }

        Ok(())
    }
}

impl<R> RbxReadExt for R where R: Read {}

pub trait RbxWriteExt: Write {
    fn write_le_u32(&mut self, value: u32) -> io::Result<()> {
        self.write_all(&value.to_le_bytes())?;

        Ok(())
    }

    fn write_le_u16(&mut self, value: u16) -> io::Result<()> {
        self.write_all(&value.to_le_bytes())?;

        Ok(())
    }

    fn write_le_i16(&mut self, value: i16) -> io::Result<()> {
        self.write_all(&value.to_le_bytes())?;

        Ok(())
    }

    fn write_le_f32(&mut self, value: f32) -> io::Result<()> {
        self.write_all(&value.to_le_bytes())?;

        Ok(())
    }

    fn write_le_f64(&mut self, value: f64) -> io::Result<()> {
        self.write_all(&value.to_le_bytes())?;

        Ok(())
    }

    fn write_u8(&mut self, value: u8) -> io::Result<()> {
        self.write_all(&[value])?;

        Ok(())
    }

    fn write_binary_string(&mut self, value: &[u8]) -> io::Result<()> {
        self.write_le_u32(value.len() as u32)?;
        self.write_all(value)?;

        Ok(())
    }

    fn write_string(&mut self, value: &str) -> io::Result<()> {
        self.write_binary_string(value.as_bytes())
    }

    fn write_bool(&mut self, value: bool) -> io::Result<()> {
        self.write_u8(value as u8)
    }

    /// Takes `values` and writes it as a blob of data with each value
    /// interleaved by `N` bytes.
    ///
    /// This function allocates `N * values.len()` bytes before writing.
    fn write_interleaved_bytes<const N: usize>(&mut self, values: &[[u8; N]]) -> io::Result<()> {
        let len = values.len();
        let mut blob = vec![0; len * N];
        for (i, bytes) in values.iter().enumerate() {
            for (j, byte) in bytes.iter().enumerate() {
                blob[i + len * j] = *byte;
            }
        }
        self.write_all(&blob)?;

        Ok(())
    }

    /// Writes all items from `values` into the buffer as a blob of interleaved
    /// bytes. Transformation is applied to the values as they're written.
    fn write_interleaved_i32_array<I>(&mut self, values: I) -> io::Result<()>
    where
        I: Iterator<Item = i32>,
    {
        let values: Vec<_> = values.map(|v| transform_i32(v).to_be_bytes()).collect();
        self.write_interleaved_bytes(&values)
    }

    /// Writes all items from `values` into the buffer as a blob of interleaved
    /// bytes.
    fn write_interleaved_u32_array(&mut self, values: &[u32]) -> io::Result<()> {
        let values: Vec<_> = values.iter().map(|v| v.to_be_bytes()).collect();
        self.write_interleaved_bytes(&values)
    }

    /// Writes all items from `values` into the buffer as a blob of interleaved
    /// bytes. Rotation is applied to the values as they're written.
    fn write_interleaved_f32_array<I>(&mut self, values: I) -> io::Result<()>
    where
        I: Iterator<Item = f32>,
    {
        let values: Vec<_> = values
            .map(|v| v.to_bits().rotate_left(1).to_be_bytes())
            .collect();
        self.write_interleaved_bytes(&values)
    }

    /// Writes all items from `values` into the buffer as a blob of interleaved
    /// bytes. The appropriate transformation and de-accumulation is done as
    /// values are written.
    fn write_referent_array<I>(&mut self, values: I) -> io::Result<()>
    where
        I: Iterator<Item = i32>,
    {
        let mut last_value = 0;
        let delta_encoded = values.map(|value| {
            let encoded = value - last_value;
            last_value = value;
            encoded
        });

        self.write_interleaved_i32_array(delta_encoded)
    }

    /// Writes all items from `values` into the buffer as a blob of interleaved
    /// bytes. Transformation is applied to the values as they're written.
    fn write_interleaved_i64_array<I>(&mut self, values: I) -> io::Result<()>
    where
        I: Iterator<Item = i64>,
    {
        let values: Vec<_> = values.map(|v| transform_i64(v).to_be_bytes()).collect();
        self.write_interleaved_bytes(&values)
    }
}

impl<W> RbxWriteExt for W where W: Write {}

/// Applies the 'zigzag' transformation done by Roblox to many `i32` values.
pub fn transform_i32(value: i32) -> i32 {
    (value << 1) ^ (value >> 31)
}

/// Inverses the 'zigzag' encoding transformation done by Roblox to many
/// `i32` values.
pub fn untransform_i32(value: i32) -> i32 {
    ((value as u32) >> 1) as i32 ^ -(value & 1)
}

/// Applies the 'zigzag' transformation done by Roblox to many `i64` values.
pub fn transform_i64(value: i64) -> i64 {
    (value << 1) ^ (value >> 63)
}

/// Inverses the 'zigzag' encoding transformation done by Roblox to many
/// `i64` values.
pub fn untransform_i64(value: i64) -> i64 {
    ((value as u64) >> 1) as i64 ^ -(value & 1)
}

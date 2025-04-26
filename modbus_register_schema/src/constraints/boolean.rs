use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BooleanConstraints {
    // max read: 2000 bits
    // max write: 1968 bits
    pub val: Vec<u8>,
    pub max_bits: u16,
}

impl BooleanConstraints {
    pub fn new(max_bits: u16) -> Self {
        let byte_size = (max_bits as usize + 7) / 8;
        Self {
            val: vec![0; byte_size],
            max_bits,
        }
    }

    pub fn set_bit(&mut self, index: u16, value: bool) {
        if index >= self.max_bits {
            panic!(
                "index out of bounds, index: {}, max_bits: {}",
                index, self.max_bits
            );
        }

        let byte_index = (index / 8) as usize;
        let bit_index = (index % 8) as usize;

        if value {
            self.val[byte_index] |= 1 << bit_index; // set
        } else {
            self.val[byte_index] &= !(1 << bit_index); // clear
        }
    }

    pub fn get_bit(&self, index: u16) -> bool {
        if index >= self.max_bits {
            panic!(
                "index out of bounds, index: {}, max_bits: {}",
                index, self.max_bits
            );
        }

        let byte_index = (index / 8) as usize;
        let bit_index = (index % 8) as usize;

        return (self.val[byte_index] & (1 << bit_index)) != 0;
    }

    pub fn set_bits(&mut self, start_index: u16, values: &[bool]) {
        let end_index = start_index + values.len() as u16;
        if end_index > self.max_bits {
            panic!(
                "index out of bounds, end_index: {}, max_bits: {}",
                end_index, self.max_bits
            );
        }

        for (i, &value) in values.iter().enumerate() {
            self.set_bit(start_index + i as u16, value);
        }
    }

    pub fn get_bits(&self, start_index: u16, length: usize) -> Vec<bool> {
        let end_index = start_index + length as u16;
        if end_index > self.max_bits {
            panic!(
                "index out of bounds, end_index: {}, max_bits: {}",
                end_index, self.max_bits
            );
        }

        return (0..length)
            .map(|i| self.get_bit(start_index + i as u16))
            .collect();
    }
}

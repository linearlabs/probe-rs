/// Implementation of the DAP_SWJ_SEQUENCE command
///
use super::super::{Category, CmsisDapError, Request, Response, SendError, Status};

#[derive(Clone, Copy, Debug)]
pub struct SequenceRequest {
    bit_count: u8,
    data: [u8; 32],
}

impl Request for SequenceRequest {
    const CATEGORY: Category = Category(0x12);

    fn to_bytes(&self, buffer: &mut [u8], offset: usize) -> Result<usize, SendError> {
        buffer[offset] = self.bit_count;

        // calculate transfer len in bytes
        // A bit_count of zero means that we want to transmit 256 bits
        let mut transfer_len_bytes: usize = if self.bit_count == 0 {
            256 / 8
        } else {
            usize::from(self.bit_count / 8)
        };

        if self.bit_count % 8 != 0 {
            transfer_len_bytes += 1;
        }

        buffer[(offset + 1)..(offset + 1 + transfer_len_bytes)]
            .copy_from_slice(&self.data[..transfer_len_bytes]);

        // bit_count + data
        Ok(1 + transfer_len_bytes)
    }
}

impl SequenceRequest {
    pub(crate) fn new(data: &[u8], bit_count: u8) -> Result<SequenceRequest, CmsisDapError> {
        if data.len() > 32 {
            return Err(CmsisDapError::TooMuchData);
        }

        if (bit_count as usize + 7) / 8 > data.len() {
            panic!("Data too short for given bit length. This is a bug, please report it.")
        }

        let mut owned_data = [0u8; 32];

        owned_data[..data.len()].copy_from_slice(data);

        Ok(SequenceRequest {
            bit_count,
            data: owned_data,
        })
    }
}

#[derive(Debug)]
pub struct SequenceResponse(pub(crate) Status);

impl Response for SequenceResponse {
    fn from_bytes(buffer: &[u8], offset: usize) -> Result<Self, SendError> {
        Ok(SequenceResponse(Status::from_byte(buffer[offset])?))
    }
}

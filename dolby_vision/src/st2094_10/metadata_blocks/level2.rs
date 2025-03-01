use bitvec_helpers::{bitvec_reader::BitVecReader, bitvec_writer::BitVecWriter};

#[cfg(feature = "serde_feature")]
use serde::Serialize;

use super::ExtMetadataBlock;

///  Creative intent trim passes per target display peak brightness
#[repr(C)]
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde_feature", derive(Serialize))]
pub struct ExtMetadataBlockLevel2 {
    pub target_max_pq: u16,
    pub trim_slope: u16,
    pub trim_offset: u16,
    pub trim_power: u16,
    pub trim_chroma_weight: u16,
    pub trim_saturation_gain: u16,
    pub ms_weight: i16,
}

impl ExtMetadataBlockLevel2 {
    pub fn parse(reader: &mut BitVecReader) -> ExtMetadataBlock {
        ExtMetadataBlock::Level2(Self {
            target_max_pq: reader.get_n(12),
            trim_slope: reader.get_n(12),
            trim_offset: reader.get_n(12),
            trim_power: reader.get_n(12),
            trim_chroma_weight: reader.get_n(12),
            trim_saturation_gain: reader.get_n(12),
            ms_weight: reader.get_n::<u16>(13) as i16,
        })
    }

    pub fn write(&self, writer: &mut BitVecWriter) {
        writer.write_n(&self.target_max_pq.to_be_bytes(), 12);
        writer.write_n(&self.trim_slope.to_be_bytes(), 12);
        writer.write_n(&self.trim_offset.to_be_bytes(), 12);
        writer.write_n(&self.trim_power.to_be_bytes(), 12);
        writer.write_n(&self.trim_chroma_weight.to_be_bytes(), 12);
        writer.write_n(&self.trim_saturation_gain.to_be_bytes(), 12);
        writer.write_n(&self.ms_weight.to_be_bytes(), 13);
    }
}

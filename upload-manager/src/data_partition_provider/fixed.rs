use super::{DataPartitionProvider, DataPartitionProviderFeedback, PartSize};
use std::num::NonZeroU64;

#[derive(Debug, Clone, Copy)]
pub struct FixedDataPartitionProvider(NonZeroU64);

impl FixedDataPartitionProvider {
    #[inline]
    pub fn new(part_size: u64) -> Option<Self> {
        NonZeroU64::new(part_size).map(Self)
    }

    #[inline]
    pub fn fixed_part_size(&self) -> NonZeroU64 {
        self.0
    }
}

impl DataPartitionProvider for FixedDataPartitionProvider {
    #[inline]
    fn part_size(&self) -> PartSize {
        self.0.into()
    }

    #[inline]
    fn feedback(&self, _feedback: DataPartitionProviderFeedback<'_>) {}
}

impl From<NonZeroU64> for FixedDataPartitionProvider {
    #[inline]
    fn from(part_size: NonZeroU64) -> Self {
        Self(part_size)
    }
}
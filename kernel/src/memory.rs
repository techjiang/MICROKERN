//! Minimal frame allocator for the first kernel milestone.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Frame {
    pub index: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum AllocError {
    InvalidRegion,
    OutOfMemory,
    InvalidOrder,
}

/// Buddy allocator backed by a caller-owned static memory slice.
pub struct BuddyAllocator {
    base: usize,
    frame_count: usize,
    max_order: u8,
    free_lists: [u64; 16],
}

impl BuddyAllocator {
    pub fn new(base: usize, length: usize, max_order: u8) -> Result<Self, AllocError> {
        let slot_bits = (usize::BITS - (base | length).leading_zeros()) as usize;
        if max_order as usize >= 16 || slot_bits > 64 {
            return Err(AllocError::InvalidRegion);
        }
        let frame_count = length >> 12;
        if frame_count == 0 || base % 4096 != 0 {
            return Err(AllocError::InvalidRegion);
        }

        let mut free_lists = [0u64; 16];
        free_lists[max_order as usize] = frame_count as u64;
        Ok(Self { base, frame_count, max_order, free_lists })
    }

    pub fn allocate(&mut self, order: u8) -> Option<Frame> {
        if order > self.max_order || order as usize >= 16 {
            return None;
        }

        let block = (order as usize..=self.max_order as usize)
            .find_map(|level| self.take_bit(level))?;

        for level in (order as usize..block.level).rev() {
            let buddy = (block.index << 1) | 1;
            self.set_bit(level, buddy);
        }

        Some(Frame { index: block.index << order })
    }

    pub fn free(&mut self, frame: Frame, order: u8) {
        if order > self.max_order || order as usize >= 16 {
            return;
        }

        let mut level = order as usize;
        let mut index = frame.index >> order;
        while level < self.max_order as usize {
            let buddy = index ^ 1;
            if !self.has_bit(level, buddy) {
                self.set_bit(level, index);
                return;
            }
            self.clear_bit(level, buddy);
            index >>= 1;
            level += 1;
        }
        self.set_bit(level, index);
    }

    pub fn used_frames(&self) -> usize {
        let mut used = 0;
        for level in 0..=self.max_order as usize {
            used += self.free_lists[level].count_ones() as usize * (1 << level);
        }
        self.frame_count - used
    }

    fn take_bit(&mut self, level: usize) -> Option<Block> {
        let bit = self.free_lists[level].trailing_zeros() as usize;
        if bit == 64 {
            return None;
        }
        self.clear_bit(level, bit);
        Some(Block { level, index: bit })
    }

    fn has_bit(&self, level: usize, index: usize) -> bool {
        index < 64 && self.free_lists[level] & (1 << index) != 0
    }

    fn set_bit(&mut self, level: usize, index: usize) {
        if index < 64 {
            self.free_lists[level] |= 1 << index;
        }
    }

    fn clear_bit(&mut self, level: usize, index: usize) {
        if index < 64 {
            self.free_lists[level] &= !(1 << index);
        }
    }
}

#[derive(Clone, Copy)]
struct Block {
    level: usize,
    index: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocates_and_frees_frames() {
        let mut memory = [0u8; 4096 * 8];
        let mut allocator = BuddyAllocator::new(memory.as_ptr() as usize, memory.len(), 3).unwrap();

        let first = allocator.allocate(0).unwrap();
        assert_eq!(first.index, 0);
        assert_eq!(allocator.used_frames(), 1);

        allocator.free(first, 0);
        assert_eq!(allocator.used_frames(), 0);
    }

    #[test]
    fn rejects_empty_region() {
        assert_eq!(BuddyAllocator::new(0, 0, 0), Err(AllocError::InvalidRegion));
    }
}

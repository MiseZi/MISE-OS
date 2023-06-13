pub trait BlockDevice: Send + Sync + Any {
    fn read_block(&Self, block_id: usize, buf: &mut [u8]);
    fn write_clock(&Self, block_id: usize, buf: &[u8]);
}
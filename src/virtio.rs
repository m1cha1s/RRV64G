use crate::prelude::MemIntf;

const MAX_BLOCK_QUEUE: u32 = 1;

pub struct VirtioBlock<'a> {
    id: u64,
    driver_features: u32,
    page_size: u32,
    queue_sel: u32,
    queue_num: u32,
    queue_pfn: u32,
    queue_notify: u32,
    status: u32,
    disk: &'a mut dyn MemIntf,
}

impl<'a> VirtioBlock<'a> {
    pub fn new(disk_image: &'a mut dyn MemIntf) -> Self {
        Self {
            id: 0,
            driver_features: 0,
            page_size: 0,
            queue_sel: 0,
            queue_num: 0,
            queue_pfn: 0,
            queue_notify: MAX_BLOCK_QUEUE,
            status: 0,
            disk: disk_image,
        }
    }
}

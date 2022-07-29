use crate::prelude::{Exception, MemIntf, VIRTIO_BASE};

const MAX_BLOCK_QUEUE: u32 = 1;

pub const VIRTIO_MAGIC: u64 = VIRTIO_BASE + 0x000;
// The version. 1 is legacy.
pub const VIRTIO_VERSION: u64 = VIRTIO_BASE + 0x004;
// device type; 1 is net, 2 is disk.
pub const VIRTIO_DEVICE_ID: u64 = VIRTIO_BASE + 0x008;
// Always return 0x554d4551
pub const VIRTIO_VENDOR_ID: u64 = VIRTIO_BASE + 0x00c;
// Device features.
pub const VIRTIO_DEVICE_FEATURES: u64 = VIRTIO_BASE + 0x010;
// Driver features.
pub const VIRTIO_DRIVER_FEATURES: u64 = VIRTIO_BASE + 0x020;
// Page size for PFN, write-only.
pub const VIRTIO_GUEST_PAGE_SIZE: u64 = VIRTIO_BASE + 0x028;
// Select queue, write-only.
pub const VIRTIO_QUEUE_SEL: u64 = VIRTIO_BASE + 0x030;
// Max size of current queue, read-only. In QEMU, `VIRTIO_COUNT = 8`.
pub const VIRTIO_QUEUE_NUM_MAX: u64 = VIRTIO_BASE + 0x034;
// Size of current queue, write-only.
pub const VIRTIO_QUEUE_NUM: u64 = VIRTIO_BASE + 0x038;
// Physical page number for queue, read and write.
pub const VIRTIO_QUEUE_PFN: u64 = VIRTIO_BASE + 0x040;
// Notify the queue number, write-only.
pub const VIRTIO_QUEUE_NOTIFY: u64 = VIRTIO_BASE + 0x050;
// Device status, read and write. Reading from this register returns the current device status flags.
// Writing non-zero values to this register sets the status flags, indicating the OS/driver
// progress. Writing zero (0x0) to this register triggers a device reset.
pub const VIRTIO_STATUS: u64 = VIRTIO_BASE + 0x070;

pub const PAGE_SIZE: u64 = 4096;
pub const SECTOR_SIZE: u64 = 512;

// virtio block request type
pub const VIRTIO_BLK_T_IN: u32 = 0;
pub const VIRTIO_BLK_T_OUT: u32 = 1;

// virtqueue descriptor flags
pub const VIRTQ_DESC_F_NEXT: u16 = 1;
pub const VIRTQ_DESC_F_WRITE: u16 = 2;
pub const VIRTQ_DESC_F_INDIRECT: u16 = 4;

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

    pub fn is_interrupting(&mut self) -> bool {
        if self.queue_notify < MAX_BLOCK_QUEUE {
            self.queue_notify = MAX_BLOCK_QUEUE;
            return true;
        }
        false
    }

    pub fn get_new_id(&mut self) -> u64 {
        self.id = self.id.wrapping_add(1);
        self.id
    }

    pub fn desc_addr(&self) -> u64 {
        self.queue_pfn as u64 * self.page_size as u64
    }

    // FIXME: used uwraps from lack of energy :)
    pub fn read_disk(&mut self, addr: u64) -> u64 {
        self.disk.load(addr, 8).unwrap()
    }

    pub fn write_disk(&mut self, addr: u64, val: u64) {
        self.disk.store(addr, val, 8).unwrap();
    }
}

impl<'a> MemIntf for VirtioBlock<'a> {
    fn reset(&mut self) {
        todo!()
    }

    fn load(&mut self, addr: u64, size: u64) -> Result<u64, crate::prelude::Exception> {
        if size != 32 {
            return Err(Exception::LoadAccessFault(addr));
        }

        match addr {
            VIRTIO_MAGIC => Ok(0x74726976),
            VIRTIO_VERSION => Ok(0x1),
            VIRTIO_DEVICE_ID => Ok(0x2),
            VIRTIO_VENDOR_ID => Ok(0x554d4551),
            VIRTIO_DEVICE_FEATURES => Ok(0x0),
            VIRTIO_DRIVER_FEATURES => Ok(self.driver_features as u64),
            VIRTIO_QUEUE_NUM_MAX => Ok(MAX_BLOCK_QUEUE as u64),
            VIRTIO_QUEUE_PFN => Ok(self.status as u64),
            _ => Ok(0),
        }
    }

    fn store(&mut self, addr: u64, val: u64, size: u64) -> Result<(), crate::prelude::Exception> {
        if size != 32 {
            return Err(Exception::StoreAMOAccessFault(addr));
        }

        let value = val as u32;

        match addr {
            VIRTIO_DEVICE_FEATURES => Ok(self.driver_features = value),
            VIRTIO_GUEST_PAGE_SIZE => Ok(self.page_size = value),
            VIRTIO_QUEUE_SEL => Ok(self.queue_sel = value),
            VIRTIO_QUEUE_NUM => Ok(self.queue_num = value),
            VIRTIO_QUEUE_PFN => Ok(self.queue_pfn = value),
            VIRTIO_QUEUE_NOTIFY => Ok(self.queue_notify = value),
            VIRTIO_STATUS => Ok(self.status = value),
            _ => Ok(()),
        }
    }
}

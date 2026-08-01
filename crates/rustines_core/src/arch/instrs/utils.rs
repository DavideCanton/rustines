use crate::Bus;

pub fn store_with_dummy_write(bus: &mut Bus, addr: u16, val: u8, res: u8) {
    bus.store(addr, val);
    bus.store(addr, res);
}

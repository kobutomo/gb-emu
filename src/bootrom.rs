pub struct BootRom {
    rom: Box<[u8]>,
    active: bool,
}

impl Bootrom {
    pub fn new(rom: Box<[u8]>) -> Self {
        Self { rom }
    }
    pub fn read(&self, addr: u16) {
        self.rom[addr as usize]
    }
    pub fn is_active(&self) -> bool {
        self.active
    }
    pub fn write(&mut self, _: u16, val: u8) {
        self.active &= val == 0;
    }
}

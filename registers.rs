#[derive(Clone, Copy, Debug, Default)]
pub struct Registers {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    // 7 6 5 4 3 2 1 0
    // Z N H C 0 0 0 0
    // Z: Zero Flag
    // N: Subtract Flag
    // H: Half Carry Flag
    // C: Carry Flag
    // 0: Not used
    // 0: Not used
    // 0: Not used
    // 0: Not used
    pub f: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn write_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0b_1111_0000) as u8;
    }
    pub fn write_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = (val & 0b_1111_1111) as u8;
    }
    pub fn write_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = (val & 0b_1111_1111) as u8;
    }
    pub fn write_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = (val & 0b_1111_1111) as u8;
    }
    // 演算結果が 0 かどうかを返す
    pub fn zf(&self) -> bool {
        // フラグレジスタの 7 bit 目を見るだけ
        (self.f & 0b_1000_0000) > 0
    }
    // 実行中の命令が演算命令かを返す
    pub fn zn(&self) -> bool {
        // フラグレジスタの 6 bit 目を見るだけ
        (self.f & 0b_0100_0000) > 0
    }
    // 3bit 目で繰り上（下）が発生したかを返す
    pub fn zh(&self) -> bool {
        // フラグレジスタの 5 bit 目を見るだけ
        (self.f & 0b_0010_0000) > 0
    }
    // 7bit 目で繰り上（下）が発生したかを返す
    pub fn zc(&self) -> bool {
        // フラグレジスタの 4 bit 目を見るだけ
        (self.f & 0b_0001_0000) > 0
    }

    pub fn set_zf(&mut self, zf: bool) {
        if zf {
            self.f |= 0b_1000_0000;
        } else {
            self.f &= 0b_0111_1111;
        }
    }
}

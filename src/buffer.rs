const SIZE_EXP: usize = 7;
const SIZE: usize = 1 << SIZE_EXP;

pub struct Buffer {
    writer: usize,
    reader: usize,
    data: [u8; SIZE],
}

impl Buffer {
    pub const fn new() -> Self {
        Self {
            writer: 0,
            reader: 0,
            data: [0; SIZE],
        }
    }

    #[inline]
    pub fn push(&mut self, u: u8) -> Result<(), ()> {
        if (self.writer + 1) % SIZE != self.reader {
            self.data[self.writer % SIZE] = u;
            self.writer = (self.writer + 1) % SIZE;
            Ok(())
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn pop(&mut self) -> Result<u8, ()> {
        if self.reader != self.writer {
            let u = self.data[self.reader % SIZE];
            self.reader = (self.reader + 1) % SIZE;
            Ok(u)
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.reader == self.writer
    }
}

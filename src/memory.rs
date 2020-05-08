/// Program start address
pub const PROGRAM_START: u16 = 0x200;

// Struct that represents the RAM memory of the machine
pub struct Memory {
    ram: [u8; 0x1000],
}

impl Memory {
    /// Creates a new instance of Memory with the default interpreter values
    pub fn new() -> Memory {
        let mut mem = Memory { ram: [0; 0x1000] };

        mem.load_font_sprites();

        mem
    }

    /// Loads the default font sprites to memory
    /// The occupied memory address range from 0x00 to 0x80
    fn load_font_sprites(&mut self) {
        // Default font sprites
        let sprites: [[u8; 5]; 0x10]= [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
            [0x20, 0x60, 0x20, 0x20, 0x70], // 1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
            [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
            [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
            [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
            [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
            [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
        ];

        // Load them to memory
        let mut i = 0;
        for sprite in sprites.iter() {
            for byte in sprite.iter() {
                self.write_byte(i, *byte);
                i += 1;
            }
        }
    }

    pub fn write_byte(&mut self, index: usize, byte: u8) {
        self.ram[index] = byte;
    }

    pub fn read_byte(&mut self, index: usize) -> u8{
        self.ram[index]
    }
}

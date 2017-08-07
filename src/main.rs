fn main() {

    //setup graphics
    //setup input

    //initialise the chip8

    //emulation loop


}


struct Chip8{
    memory:[u8;4096],
    opcode:u16,//short opcode 2 byte
    v:[u8;16],//registers 16 byte
    index:u16,
    pc:u16,
    stack: [u16;16],
    sp:u16
        
}


impl Chip8{
    pub fn new() -> Chip8{
        //initialse the cpu core clear memory
        return Chip8{memory:[0;4096]};
    }

    pub fn emulate_cyle(self){//might need to change reference on self
        //fetch opcode
        //decode opcode
        //exeute opcode

        //update timers
    }
}

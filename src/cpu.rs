
    use std::fs::File;

use std::io::Read;
pub struct Chip8{
    memory:[u8;4096],
    opcode:u16,//short opcode 2 byte
    v:[u8;16],//registers 16 byte
    i:usize, //the index is 2bytes 16 bit on the original hardware, hwoever as
    //this is a memeory index usize is fine as this will settly on whatever rust
    // compiles for
    pc:usize,//the program counter points to the next instruction, In an actual
    //chip 8 this is a short at u16 but seing as this is just an index it can be
    // usize for platform inependant memory access
    stack: [u16;16],//the stack
    sp:u16//stack pointer
        
}

impl Chip8{
 
    pub fn new_with_game(game:String) -> Chip8{

        let fontset  =
[ 
    0xF0, 0x90, 0x90, 0x90, 0xF0, //0
    0x20, 0x60, 0x20, 0x20, 0x70, //1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, //3
    0x90, 0x90, 0xF0, 0x10, 0x10, //4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, //5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, //6
    0xF0, 0x10, 0x20, 0x40, 0x40, //7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, //9
    0xF0, 0x90, 0xF0, 0x90, 0x90, //A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, //B
    0xF0, 0x80, 0x80, 0x80, 0xF0, //C
    0xE0, 0x90, 0x90, 0x90, 0xE0, //D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, //E
    0xF0, 0x80, 0xF0, 0x80, 0x80  //F
];
        let mut memory = [0;4096];
        let v = [0;16];
        let stack = [0;16];
        let i = 0;
        let sp = 0;
        let opcode = 0;
        let pc = 0x200;

        let mut buf:Vec<u8> = Vec::new();
        //load the game
        //TODO this could be more rusty
        let mut f = File::open(game).ok().unwrap();
        //read into buffer
        let size = f.read_to_end(&mut buf).ok().unwrap();
        if 4096-512 < size {
            panic!("ROM was bigger than memory");
        }
        for i in 0..size{
            let byte = buf[0];
            memory[512+i] = byte;
        }
        //load the font
        for i in 0..80{
            memory[i] = fontset[i];
        }
        //TODO load the font set into the first 80 bytes of memory
        return Chip8{memory,v,stack,i,sp,pc,opcode};
    }
    pub fn new() -> Chip8{
        //initialse the cpu core clear memory
        //TODO read game file into memory at 0x200 512
        let memory = [0;4096];
        let v = [0;16];
        let stack = [0;16];
        let i = 0;
        let sp = 0;
        let opcode = 0;
        let pc = 0x200;
        //TODO load the font set into the first 80 bytes of memory
        return Chip8{memory,v,stack,i,sp,pc,opcode};
    }
    //When using the value of the opcode to to access memory it must be returned
    // as a usize variable which represents what ever the systems memeory
    // address sizes are for rust this should not affect the program behaviour
    fn get_opcode(&self) -> usize {
        //gets the x value of the opcode i think
        return ((self.opcode & 0x0F00 ) >> 8) as usize;
    }
    pub fn is_draw_flag(&self) -> bool{
        return false;
    }
    pub fn emulate_cycle(&mut self){//might need to change reference on self
        //fetch opcode, this merges two bytes into a u16
        //The original opcode decoding
        //opcode = memory[pc] << 8 | memory[pc + 1];
        //but becuase opcode is a u16 evrything had to be in that format
        self.opcode = (self.memory[self.pc] as u16) << 8 | self.memory[self.pc+1] as u16;
        //decode opcode
        match self.opcode & 0xF000 {
            0x0000 => {//special case 
                match self.opcode & 0x000F{
                    0x0000 => println!("clear screen"),//TODO 0x00E0 clears the screen
                    0x000E => println!("return from subroutine"),//TODO 0x00EE
                    0x0033 => { // Stores the Binary-coded decimal representation of VX at the addresses I, I plus 1, and I plus 2
                        self.memory[self.i]     = self.v[self.get_opcode()] / 100;
                        self.memory[self.i + 1] = (self.v[self.get_opcode()] / 10) % 10;
                        self.memory[self.i + 2] = (self.v[self.get_opcode()] % 100) % 10;
                        self.pc += 2;
                    }
                    _ => panic!("unknown opcode {:x}",self.opcode)
                }
            }
            0xA000 => {// ANNN: sets I to the address NNN
                //execute opcode
                //casting to usize when spec says i must be u16 is ok so long as
                // we never run on platform less than 16byte
                self.i = (self.opcode & 0x0FFF) as usize;
                self.pc +=2;
            }
            0x2000 =>{//0x2NNN calls subroutine adjusts the call stack
                //put the current program counter onto the stack
                self.stack[self.sp as usize] = self.pc as u16;
                self.sp +=1;
                //set the program counter to be the new subroutine start
                self.pc = (self.opcode & 0x0FFF) as usize;
            }
            0x8000 => {// All the 0x8 opcodes
                match self.opcode & 0x000F{
                    0x0004 => {//0x8XY4 adds VY to VX sets VF carry to 1 when >255
                        //carry check
                        if self.v[((self.opcode & 0x00F0) as usize) >> 4] > (0xFF - self.v[((self.opcode & 0x0F00) >> 8 )as usize]){
                            self.v[0xF] = 1;
                        }else{
                            self.v[0xF] = 0;
                        }
                        //do the addition
                        self.v[((self.opcode & 0x0F00) >> 8) as usize] += self.v[((self.opcode & 0x00F0) >> 4) as usize];
                    }
                    _ => panic!("could not match opcode to any of the 0x8 instructions {:x}",self.opcode)
                }
            }

            _ => panic!("could not match {:x} opcode ",self.opcode)
        }
        

        //update timers
        //TODO
    }
}

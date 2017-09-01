fn main() {

    //setup graphics
    //setup input

    //initialise the chip8

    //emulation loop
    let code = 0x8034;
    println!("Decoded opcode should be {:x} with 0x0004",code & 0xF000);

}


struct Chip8{
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
    pub fn emulate_cyle(&mut self){//might need to change reference on self
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

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn it_works(){
    }

    //adds y to x
    #[test]
    fn test_8xy4_opcode(){
        let mut chip = Chip8::new();
        chip.memory[512] = 0x82;
        chip.memory[513] = 0x34;
        chip.v[2] = 3;
        chip.v[3] = 5;
        chip.emulate_cyle();
        assert_eq!(chip.v[2],8);
    }
    #[test]
    fn test_annn_opcode(){
        let mut chip = Chip8::new();
        chip.memory[512] = 0xA2;
        chip.memory[513] = 0xF0;
        chip.emulate_cyle();
        assert_eq!(chip.i,0x02F0);
    }
    // test the cl subroutine opcode
    #[test]
    fn test_2nnn_opcode(){
        let mut chip = Chip8::new();
        chip.memory[512] = 0x20;
        chip.memory[513] = 0xF0;
        chip.emulate_cyle();
        //check stack pointer incremented
        assert_eq!(chip.sp,1);
        assert_eq!(chip.pc,0x0F0);
        assert_eq!(chip.stack[0],0x200);
    }

}

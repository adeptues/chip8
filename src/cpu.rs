

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
    sp:u16,//stack pointer
    pub gfx:[u8;64*32],
    drawFlag:bool,
    delay_timer:u8,
    sound_timer:u8
}

enum Bit{
    X,Y,N,NN,NNN
}

impl Chip8{
 
    pub fn new_with_game(game:String) -> Chip8{
        //TODO this should be a struct level const
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
        let gfx = [0;64*32];
        let drawFlag = false;
        let delay_timer = 0;
        let sound_timer = 0;
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
            let byte = buf[i];
            memory[512+i] = byte;
        }
        //load the font
        for i in 0..80{
            memory[i] = fontset[i];
        }
        //TODO load the font set into the first 80 bytes of memory
        return Chip8{memory,v,stack,i,sp,pc,opcode,gfx,drawFlag,delay_timer,sound_timer};
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
        let gfx = [0;64*32];
        let drawFlag = false;
        let delay_timer = 0;
        let sound_timer = 0;
        //TODO load the font set into the first 80 bytes of memory
        return Chip8{memory,v,stack,i,sp,pc,opcode,gfx,drawFlag,delay_timer,sound_timer};
    }
    //When using the value of the opcode to to access memory it must be returned
    // as a usize variable which represents what ever the systems memeory
    // address sizes are for rust this should not affect the program behaviour
    fn get_opcode(&self,letter:Bit) -> usize {
        return match letter {
            Bit::X => ((self.opcode & 0x0F00 ) >> 8) as usize,
            Bit::Y => ((self.opcode & 0x00F0) >> 4) as usize,
            Bit::N => (self.opcode & 0x000F) as usize,
            Bit::NN => (self.opcode & 0x00FF) as usize,
            Bit::NNN => (self.opcode & 0x0FFF) as usize
        }
        
        //dxyn
        // if letter == "x" {
        //     return ((self.opcode & 0x0F00 ) >> 8) as usize;
        // }
        // if letter == "y"{
        //     return ((self.opcode & 0x00F0) >> 4) as usize;
        // }
        // if letter == "n"{
        //     return (self.opcode & 0x000F) as usize;
        // }
        // if letter == "nn" {
        //     return (self.opcode & 0x00FF) as usize;
        // }
        // if letter == "nnn"{
            
        // }
//        panic!("Could not extract value for letter {} from opcode {:x}",letter,self.opcode);
        //gets the x value of the opcode i think
        //in 6xnn this would produce the value of x
    }
    pub fn is_draw_flag(&self) -> bool{
        return self.drawFlag;
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
                    0x0000 => {
                        println!("clear screen");
                    },//TODO 0x00E0 clears the screen
                    0x000E => {// return from subroutine
                        self.pc = self.stack[self.sp as usize] as usize;
                        self.sp -= 1;
                    }
                    0x0033 => { // Stores the Binary-coded decimal representation of VX at the addresses I, I plus 1, and I plus 2
                        self.memory[self.i]     = self.v[self.get_opcode(Bit::X)] / 100;
                        self.memory[self.i + 1] = (self.v[self.get_opcode(Bit::X)] / 10) % 10;
                        self.memory[self.i + 2] = (self.v[self.get_opcode(Bit::X)] % 100) % 10;
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
            },
            0x1000 => {
                self.pc = self.get_opcode(Bit::NNN);
                //no need to update programe counter as this is a jump command i think
            }
            
            0x2000 =>{//0x2NNN calls subroutine adjusts the call stack
                //put the current program counter onto the stack
                // println!("opcode {:x}",self.opcode);
                // println!("sp: {}",self.sp);
                // println!("pc: {:x}",self.pc);
                self.stack[self.sp as usize] = self.pc as u16;
                self.sp +=1;
                //set the program counter to be the new subroutine start
                self.pc = (self.opcode & 0x0FFF) as usize;
            },
            0x3000 => {//3xnn Skip the following instruction if the value of register VX equals NN
                let x = self.get_opcode(Bit::X);
                if self.v[x] == self.get_opcode(Bit::NN) as u8{
                    self.pc +=2;
                }
                self.pc+=2;
                //TODO not sure if this means skip next opcode or just move pc
                // by 2
            }
            //6xnn store NN in register Vx
            0x6000 => {
                let x = self.get_opcode(Bit::X);
                //cast needed because opcode is two bytes long at u16
                let nn = (self.opcode & 0x00FF) as u8;
                self.v[x] = nn;
                self.pc +=2;
            },
            0x7000 => {//7XNN Add the value NN to register VX
                let nn = self.get_opcode(Bit::NN);
                let x = self.get_opcode(Bit::X);
                self.v[x] = self.v[x]+nn as u8;
                self.pc +=2;
            }
            0x8000 => {// All the 0x8 opcodes
                match self.opcode & 0x000F{
                    0x0000 => {
                        //8xy0 Stores the value of register Vy in register Vx.
                        let vy = self.v[self.get_opcode(Bit::Y)];
                        self.v[self.get_opcode(Bit::X)] = vy;
                        self.pc +=2;
                    },
                    0x0001 =>{
                        //8xy1 - OR Vx, Vy
                        let out = self.v[self.get_opcode(Bit::X)] | self.v[self.get_opcode(Bit::Y)];
                        self.v[self.get_opcode(Bit::X)] = out;
                        self.pc +=2;
                    },
                    0x0002 => {//bitwise and
                        let out = self.v[self.get_opcode(Bit::X)] & self.v[self.get_opcode(Bit::Y)];
                        self.v[self.get_opcode(Bit::X)] = out;
                        self.pc +=2;
                    },
                    0x0003 => {//8xy3 - XOR Vx, Vy
                        let out = self.v[self.get_opcode(Bit::X)] ^ self.v[self.get_opcode(Bit::Y)];
                        self.v[self.get_opcode(Bit::X)] = out;
                        self.pc +=2;
                    }
                    0x0004 => {//0x8XY4 adds VY to VX sets VF carry to 1 when >255
                        //carry check
                        if self.v[((self.opcode & 0x00F0) as usize) >> 4] > (0xFF - self.v[((self.opcode & 0x0F00) >> 8 )as usize]){
                            self.v[0xF] = 1;
                        }else{
                            self.v[0xF] = 0;
                        }
                        //do the addition
                        self.v[((self.opcode & 0x0F00) >> 8) as usize] += self.v[((self.opcode & 0x00F0) >> 4) as usize];
                        self.pc +=2;
                    },
                    // 0x0005=>{
                        
                    // }
                    _ => panic!("could not match opcode to any of the 0x8 instructions {:x}",self.opcode)
                }
            },
            0xF000 => {
                match self.opcode & 0x00FF {
                    0x0007 => {
                        self.v[self.get_opcode(Bit::X)] = self.delay_timer;
                        self.pc +=2;
                    },
                    0x000A => {//Fx0a
                        //Wait for a key press, store the value of the key in
                        // Vx.
                        panic!("todokey presses");
                    },
                    0x0015 => {
                        //Fx15 - LD DT, Vx
                        //Set delay timer = Vx.
                        let vx = self.v[self.get_opcode(Bit::X)];
                        self.delay_timer = vx;
                        self.pc +=2;
                    },
                    0x0018 =>{//Fx15 set sound timer to vx;
                        let vx = self.v[self.get_opcode(Bit::X)];
                        self.sound_timer = vx;
                        self.pc +=2;
                    },
                    0x001e =>{
                        let vx = self.v[self.get_opcode(Bit::X)];
                        //Fx1E - ADD I, Vx
                        //Set I = I + Vx.
                        self.i = (self.i +vx as usize) ;
                        self.pc +=2;
                    }
                    0x0033 => {
                        //Binary coded decimal of and register value
                        //Store the BCD equivalent of
                        //the value in register VX at addresses I, I+1,
                        // and I+2
                        let vx = self.v[self.get_opcode(Bit::X)];
                        // BCD involves splitting a value up into units 100's
                        // 10's and 1's
                        // value in register vx is not in bcd format needs
                        // splitting from normal number into bcd at the mem addresses
                        let ones = (vx % 100) % 10;
                        let tens = (vx / 10) % 10;
                        let hundreds = vx/100;
                        self.memory[self.i] = hundreds;
                        self.memory[self.i+1] = tens;
                        self.memory[self.i+2] = ones;
                        self.pc +=2
                    },
                    0x0029 => {
                        //Set I to the memory address of the sprite data
                        // corresponding to the hexadecimal digit stored in
                        // register VX
                        let font_mem_width = 5;
                        
                        let vx = self.v[self.get_opcode(Bit::X)];
                        self.i = (vx * font_mem_width) as usize;
                        self.pc +=2;
                    },
                    0x0055 => {
                        // the invers of 0x0065
                        panic!("todo 0x55");
                    }
                    0x0065 =>{
                        //Fill registers V0 to VX inclusive with the values
                        // stored in memory starting at address I
                        let mut mem_counter = self.i;
                        //TODO this might not be inclusive
                        for i in 0..16{
                            self.v[i] = self.memory[mem_counter];
                            mem_counter += 1;
                        }
                        //TODO not sure if we needed to increment i in the
                        // previous loop instead of sperate counter
                        self.i = self.i+self.get_opcode(Bit::X)+1;
                        self.pc +=2;
                    }
                    _ => panic!("could not match F series opcode {:x}",self.opcode)
                }
            }
            0xD000 => {//DXYN opcode draw sprite at location not sure if 0004
                //conflicts with others
                let  x = self.v[self.get_opcode(Bit::X)];
                let y = self.v[self.get_opcode(Bit::Y)];
                let height = self.opcode & 0x000F;
                //TODO too much casting here maybe problems
                //TODO TESTME
                self.v[0xF] = 0;

                for yline in 0..height{
                    let pixel = self.memory[self.i+ yline as usize];
                    for xline in 0..8{
                        if pixel & (0x80 >> xline) != 0{
                            if self.gfx[((x+xline) as u16+((y as u16 +yline))*64) as usize] == 1{
                                self.v[0xF] = 1;
                            }
                            self.gfx[((x +xline) as u16+((y as u16+yline)*64)) as usize] ^=1;
                        }
                    }
                    
                }
                // for (int yline = 0; yline < height; yline++)
                // {
                //     pixel = memory[I + yline];
                //     for(int xline = 0; xline < 8; xline++)
                //     {
                //         if((pixel & (0x80 >> xline)) != 0)
                //         {
                //             if(gfx[(x + xline + ((y + yline) * 64))] == 1)
                //                 V[0xF] = 1;                                 
                //             gfx[x + xline + ((y + yline) * 64)] ^= 1;
                //         }
                //     }
                // }
                
                // drawFlag = true;
                self.drawFlag = true;
                self.pc += 2;
            }

            _ => panic!("could not match {:x} opcode ",self.opcode)
        }
        

        //update timers
        //TODO
    }
}


//TODO find way to move these to seperate file
#[cfg(test)]
mod tests{

    use super::*;
    //adds y to x
    #[test]
    fn test_8xy4_opcode(){
        let mut chip = Chip8::new();
        chip.memory[512] = 0x82;
        chip.memory[513] = 0x34;
        chip.v[2] = 3;
        chip.v[3] = 5;
        chip.emulate_cycle();
        assert_eq!(chip.v[2],8);
    }
    #[test]
    fn test_annn_opcode(){
        let mut chip = Chip8::new();
        chip.memory[512] = 0xA2;
        chip.memory[513] = 0xF0;
        chip.emulate_cycle();
        assert_eq!(chip.i,0x02F0);
    }
    // test the cl subroutine opcode
    #[test]
    fn test_2nnn_opcode(){
        let mut chip = Chip8::new();
        chip.memory[512] = 0x20;
        chip.memory[513] = 0xF0;
        chip.emulate_cycle();
        //check stack pointer incremented
        assert_eq!(chip.sp,1);
        assert_eq!(chip.pc,0x0F0);
        assert_eq!(chip.stack[0],0x200);
    }

    //test store number nn in x
    #[test]
    fn test_6xnn(){
        let mut chip = Chip8::new();
        chip.memory[512] = 0x6b;
        chip.memory[513] = 0x20;
        chip.emulate_cycle();
        assert_eq!(chip.v[0xb],0x20);
    }

    //Adds the value kk to the value of register Vx, then stores the result in Vx.
    #[test]
    fn test_7xkk(){
        //7c04
        let mut chip = Chip8::new();
        chip.memory[512] = 0x7c;
        chip.memory[513] = 0x04;
        chip.v[0xc] = 2;
        chip.emulate_cycle();
        assert_eq!(chip.v[0xc],6);
    }
    #[test]
    fn test_3xnn(){
        //3c20
        let mut chip = Chip8::new();
        chip.memory[512] = 0x3c;
        chip.memory[513] = 0x20;
        chip.v[0xc] = 0x20;
        chip.emulate_cycle();
        assert_eq!(chip.pc,516);
    }
    #[test]
    fn test_BCD_fx33(){
        //fe33
        let mut chip = Chip8::new();
        chip.memory[512] = 0xfe;
        chip.memory[513] = 0x33;
        chip.v[0xe] = 235;//235
        chip.emulate_cycle();
        assert_eq!(chip.memory[chip.i],2);
        assert_eq!(chip.memory[chip.i+1],3);
        assert_eq!(chip.memory[chip.i+2],5);
    }
    
    #[test]
    fn test_fx29(){
        //                        f129
        let f:[u8;5] = [0xF0, 0x10, 0xF0, 0x80, 0xF0]; //2
        let mut chip = Chip8::new_with_game("./games/PONG".to_string());
        chip.memory[512] = 0xf1;
        chip.memory[513] = 0x29;
        chip.v[0x1] = 2;//235
        chip.emulate_cycle();
        assert_eq!(chip.i,0xa);
        assert_eq!(chip.memory[chip.i],0xF0);
        assert_eq!(chip.memory[chip.i+1],0x10);
        assert_eq!(chip.memory[chip.i+2],0xF0);
        assert_eq!(chip.memory[chip.i+3],0x80);
        assert_eq!(chip.memory[chip.i+4],0xF0);
        //TODO not sure if this actually works
    }
    

    #[test]
    fn test_fx65(){
        //f265

        let mut chip = Chip8::new();
        chip.memory[512] = 0xf2;
        chip.memory[513] = 0x65;
        chip.i = 600;
        chip.memory[600] = 1;
        chip.memory[601] = 2;
        chip.memory[602] = 3;
        chip.memory[603] = 4;
        chip.memory[604] = 5;
        chip.memory[605] = 6;
        chip.memory[606] = 7;
        chip.memory[607] = 8;
        chip.memory[608] = 9;
        chip.memory[609] = 10;
        chip.memory[610] = 11;
        chip.memory[611] = 12;
        chip.memory[612] = 13;
        chip.memory[613] = 14;
        chip.memory[614] = 15;
        chip.memory[615] = 16;

        chip.emulate_cycle();


        for i in 0..16{
            assert_eq!(chip.v[i],(i+1) as u8);
        }
    }
}

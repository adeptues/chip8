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

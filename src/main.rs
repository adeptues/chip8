mod cpu;

use cpu::Chip8;

fn main() {
    let game = "/home/tom/tom/chip8/games/pong2.c8".to_string();
    //setup graphics
    //setup input

    //initialise the chip8
    let mut chip = Chip8::new_with_game(game);

    loop{
        chip.emulate_cycle();
        if chip.is_draw_flag() {
            draw_graphics(chip.gfx);
            //TODO draw graphics
            //this should just render the cpu drawbufer
        }
        //TODO set key press state based on user input
        //need to map inputs onto some kind of hex based key pad used by the
        // original chip 8 all though the input is likley to be tied to the
        // graphics context
    }
    //emulation loop
}


//TODO may have to mvoe this out to its own module / struct if it gets too
// unweildy as both keyboard input and graphics drawing will be done here
fn draw_graphics(buf:[u8;2048]){
    //y+x+width
    //draw the 64*32 byte buffer
    println!("Draw Graphics ! yay!");
    let mut count = 0;
    let width = 64;
    let height = 32;
    
    // for x in 0..width{
    //     for y in 0..height {
    //         let pixel = buf[y+x+width];
            
    //     }
    // }
    for i in 0..2048{
        let pixel = buf[i];
        if i % 64 == 0{
            //then start printing to new lined
            print!("\n");
        }
        if pixel == 1{
            print!("*");
        }else{
            print!(" ");
        }
    }
        
}                              

#[cfg(test)]
mod tests{
    #[test]
    fn it_works(){
        
    }
}

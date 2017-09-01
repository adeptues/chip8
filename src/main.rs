mod cpu;
#[cfg(test)]
mod test;
use cpu::Chip8;

fn main() {
    let game = "/home/tom/tom/chip8/games/pong2.c8".to_string();
    //setup graphics
    //setup input

    //initialise the chip8
    let mut chip = Chip8::new_with_game(game);

    loop{
        chip.emulate_cycle();
        if(chip.is_draw_flag()){
            //TODO draw graphics
        }

        //TODO set key press state based on user input
    }
    //emulation loop
}




// #[cfg(test)]
// mod tests{


// }

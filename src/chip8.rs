use std::iter;
extern crate chrono;
use chrono::*;
use std::time;
use std::thread;

use rand::*;

#[allow(dead_code)]
fn create_font_data() -> Vec<u8>
{
    let mut data : Vec<u8> = Vec::with_capacity(5*16);

    //0
    data.push(0xF0);
    data.push(0x90);
    data.push(0x90);
    data.push(0x90);
    data.push(0xF0);

    //1
    data.push(0x20);
    data.push(0x60);
    data.push(0x20);
    data.push(0x20);
    data.push(0x70);

    //2
    data.push(0xF0);
    data.push(0x10);
    data.push(0xF0);
    data.push(0x80);
    data.push(0xF0);

    //3
    data.push(0xF0);
    data.push(0x10);
    data.push(0xF0);
    data.push(0x10);
    data.push(0xF0);

    //4
    data.push(0x90);
    data.push(0x90);
    data.push(0xF0);
    data.push(0x10);
    data.push(0x10);

    //5
    data.push(0xF0);
    data.push(0x80);
    data.push(0xF0);
    data.push(0x10);
    data.push(0xF0);

    //6
    data.push(0xF0);
    data.push(0x80);
    data.push(0xF0);
    data.push(0x90);
    data.push(0xF0);

    //7
    data.push(0xF0);
    data.push(0x10);
    data.push(0x20);
    data.push(0x40);
    data.push(0x40);

    //8
    data.push(0xF0);
    data.push(0x90);
    data.push(0xF0);
    data.push(0x90);
    data.push(0xF0);

    //9
    data.push(0xF0);
    data.push(0x90);
    data.push(0xF0);
    data.push(0x10);
    data.push(0xF0);

    //A
    data.push(0xF0);
    data.push(0x90);
    data.push(0xF0);
    data.push(0x90);
    data.push(0x90);

    //B
    data.push(0xE0);
    data.push(0x90);
    data.push(0xE0);
    data.push(0x90);
    data.push(0xE0);

    //C
    data.push(0xF0);
    data.push(0x80);
    data.push(0x80);
    data.push(0x80);
    data.push(0xF0);

    //D
    data.push(0xE0);
    data.push(0x90);
    data.push(0x90);
    data.push(0x90);
    data.push(0xE0);

    //E
    data.push(0xF0);
    data.push(0x80);
    data.push(0xF0);
    data.push(0x80);
    data.push(0xF0);

    //F
    data.push(0xF0);
    data.push(0x80);
    data.push(0xF0);
    data.push(0x80);
    data.push(0x80);

    return data;
}

#[allow(dead_code)]
pub struct Chip8
{
    memory : Vec<u8>,
    registers : Vec<u8>,
    address_register : u16,
    program_counter : u16,
    delay_timer : u8,
    sound_timer : u8,
    stack : Vec<u16>,
    screen : Vec<bool>,
    keys : Vec<bool>,
    font_data_base_address : u16
}

impl Chip8
{
    #[allow(dead_code)]
    pub fn new(rom_content : &Vec<u8>) -> Chip8
    {
        let mut chip = Chip8
        {
            registers : iter::repeat(0).take(16).collect::<Vec<u8>>(),
            address_register : 0,

            delay_timer : 0,
            sound_timer : 0,

            program_counter : 0x200,

            stack : Vec::with_capacity(16),
            screen : iter::repeat(false).take(64 * 32).collect::<Vec<bool>>(),
            keys : iter::repeat(false).take(16).collect::<Vec<bool>>(),
            memory :  iter::repeat(0).take(4096).collect::<Vec<u8>>(),
            font_data_base_address : 0
        };

        let font_data = create_font_data();
        for i in 0 .. font_data.len()
        {
            chip.memory[i] = font_data[i];
        }

        for i in 0 .. rom_content.len()
        {
            chip.memory[i + chip.program_counter as usize] = rom_content[i];
        }

        return chip;
    }

    #[allow(dead_code)]
    pub fn run_one_cycle(&mut self)
    {
        let utc : chrono::DateTime<UTC> = UTC::now();
        let opcode = self.fetch_opcode();

        println!("opcode            {:#06X}", opcode);
        println!("opcode            {}", opcode);
        println!("program_counter   {:#06X}", self.program_counter);
        println!("program_counter   {}", self.program_counter);
        println!("address_register  {}", self.address_register);
        println!("delay_timer       {}", self.delay_timer);

        for i in 0 .. 16
        {
            println!("register v{} : {}", i , self.registers[i]);
        }
        println!("");


        self.execute_opcode(opcode);


        //thread::sleep(time::Duration::from_millis(16));
        let utc2 : chrono::DateTime<UTC> = UTC::now();
        let nb_milli = (utc2 - utc).num_milliseconds();
        let w = nb_milli as f64 / 16.666666666666666666666666666667;
        //let new_timer_value = (self.delay_timer as f64 - w).round() as i32;
        let new_timer_value : i32 = self.delay_timer as i32 - 1;
        if new_timer_value <= 0
        {
            self.delay_timer = 0;
        }
        else
        {
            self.delay_timer = new_timer_value as u8;
        }

        //TODO manage input
        //TODO handle update differently
        //TODO sound timer
    }

    #[allow(dead_code)]
    fn fetch_opcode(&self) -> u16
    {
        let upper_byte_opcode  = self.memory[self.program_counter as usize] as u16;
        let lower_byte_opcode = self.memory[(self.program_counter + 1) as usize] as u16;
        upper_byte_opcode << 8 | lower_byte_opcode
    }

    #[allow(dead_code)]
    fn execute_opcode(&mut self, opcode : u16)
    {
        //00E0
        if opcode == 0x00E0
        {
            for i in 0 .. self.screen.len()
            {
                self.screen[i] = false;
            }
            self.program_counter += 2;
        }
        //00EE
        else if opcode == 0x00EE
        {
            self.program_counter = self.stack.pop().unwrap();
            self.program_counter += 2;
        }
        //1NNN
        else if opcode & 0xF000 == 0x1000
        {
            self.program_counter = opcode & 0x0FFF;
        }
        //2NNN
        else if opcode & 0xF000 == 0x2000
        {
            let nnn = (opcode & 0x0FFF) as u16;
            self.stack.push(self.program_counter);
            self.program_counter = nnn;
        }
        //3XNN
        else if opcode & 0xF000 == 0x3000
        {
            let nn = (opcode & 0x00FF) as u8;
            let x = (opcode & 0x0F00) >> 8;
            if self.registers[x as usize] == nn
            {
                self.program_counter += 4;
            }
            else
            {
                self.program_counter += 2;
            }
        }
        //4XNN
        else if opcode & 0xF000 == 0x4000
        {
            let nn = (opcode & 0x00FF) as u8;
            let x = (opcode & 0x0F00) >> 8;
            if self.registers[x as usize] != nn
            {
                self.program_counter += 4;
            }
            else
            {
                self.program_counter += 2;
            }
        }
        //5XY0
        else if opcode & 0xF00F == 0x5000
        {
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            if self.registers[x as usize] == self.registers[y as usize]
            {
                self.program_counter += 4;
            }
            else
            {
                self.program_counter += 2;
            }
        }
        //6XNN
        else if opcode & 0xF000 == 0x6000
        {
            let nn = (opcode & 0x00FF) as u8;
            let x = (opcode & 0x0F00) >> 8;
            self.registers[x as usize] = nn;
            self.program_counter += 2;
        }
        //7XNN
        else if opcode & 0xF000 == 0x7000
        {
            let nn = (opcode & 0x00FF) as u16;
            let x = (opcode & 0x0F00) >> 8;
            if nn + self.registers[x as usize] as u16 > 255
            {
                self.registers[x as usize] = ((nn + self.registers[x as usize] as u16) % 256) as u8;
            }
            else
            {
                self.registers[x as usize] += nn as u8;
            }
            self.program_counter += 2;
        }
        //8XY0
        else if opcode & 0xF00F == 0x8000
        {
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            self.registers[x as usize] = self.registers[y as usize];
            self.program_counter += 2;
        }
        //8XY1
        else if opcode & 0xF00F == 0x8001
        {
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
            self.program_counter += 2;
        }
        //8XY2
        else if opcode & 0xF00F == 0x8002
        {
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize];
            self.program_counter += 2;
        }
        //8XY3
        else if opcode & 0xF00F == 0x8003
        {
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize];
            self.program_counter += 2;
        }
        //8XY4
        else if opcode & 0xF00F == 0x8004
        {
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            let has_carry = (self.registers[x as usize] as u16 + self.registers[y as usize] as u16) > 255;
            if has_carry
            {   self.registers[x as usize] = ((self.registers[x as usize] as u16) + (self.registers[y as usize] as u16) - 256) as u8;
                self.registers[15] = 1;
            }
            else
            {
                self.registers[x as usize] = self.registers[x as usize] + self.registers[y as usize];
                self.registers[15] = 0;
            }
            self.program_counter += 2;
        }
        //8XY5
        else if opcode & 0xF00F == 0x8005
        {
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            let has_borrow = (self.registers[x as usize] as u16) > (self.registers[y as usize] as u16);
            if has_borrow
            {
                self.registers[x as usize] = self.registers[x as usize] - self.registers[y as usize];
                self.registers[15] = 1;
            }
            else
            {
                self.registers[x as usize] = ((self.registers[x as usize] as i32 - self.registers[y as usize] as i32 ) % 256) as u8;
                self.registers[15] = 0;
            }
            self.program_counter += 2;
        }
        //8XY6
        else if opcode & 0xF00F == 0x8006
        {
            let x = (opcode & 0x0F00) >> 8;
            let lest_significant_bit = self.registers[x as usize] & 0b00000001;
            self.registers[x as usize] = self.registers[x as usize] >> 1;
            self.registers[15] = lest_significant_bit;
            self.program_counter += 2;
        }
        //8XY7
        else if opcode & 0xF00F == 0x8007
        {
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            let has_borrow = (self.registers[y as usize] as u16) < (self.registers[x as usize] as u16);
            self.registers[x as usize] = self.registers[y as usize] - self.registers[x as usize];
            if has_borrow
            {
                self.registers[15] = 0;
            }
            else
            {
                self.registers[15] = 1;
            }
            self.program_counter += 2;
        }
        //8XYE
        else if opcode & 0xF00F == 0x800E
        {
            let x = (opcode & 0x0F00) >> 8;
            let most_significant_bit = self.registers[x as usize] & 0b10000000;
            self.registers[x as usize] = self.registers[x as usize] << 1;
            self.registers[15] = most_significant_bit;
            self.program_counter += 2;
        }
        //5XY0
        else if opcode & 0xF00F == 0x9000
        {
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            if self.registers[x as usize] != self.registers[y as usize]
            {
                self.program_counter += 4;
            }
            else
            {
                self.program_counter += 2;
            }
        }
        //ANNN
        else if opcode & 0xF000 == 0xA000
        {
            let nnn = (opcode & 0x0FFF) as u16;
            self.address_register = nnn;
            self.program_counter += 2;
        }
        //BNNN
        else if opcode & 0xF000 == 0xB000
        {
            let nnn = (opcode & 0x0FFF) as u16;
            self.program_counter = nnn + self.registers[0] as u16;
        }
        //CXNN
        else if opcode & 0xF000 == 0xC000
        {
            let nn = (opcode & 0x00FF) as u8;
            let x = (opcode & 0x0F00) >> 8;
            let random_number = thread_rng().gen::<u8>();
            self.registers[x as usize] =  random_number & nn;
            self.program_counter += 2;
        }
        //DXYN
        else if opcode & 0xF000 == 0xD000
        {
            //println!("address_register is {}", self.address_register);
            /*
            Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in I
            Set VF to 01 if any set pixels are changed to unset, and 00 otherwise.
            CHIP-8 sprites are always eight pixels wide and between one to fifteen pixels high.
            */
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            let n = (opcode & 0x000F) >> 0;

            let mut vx = self.registers[x as usize];
            let mut vy = self.registers[y as usize];


            vx = vx % 64;
            vy = vy % 32;
            //println!("x {}", x);
            //println!("y {}", y);
            //println!("self.registers[x as usize] {}", self.registers[x as usize]);
            //println!("self.registers[y as usize] {}", self.registers[y as usize]);

            let mut has_changed_set_pixel_to_unset = false;

            for i in 0 .. n
            {
                let sprite_row = self.memory[self.address_register as usize + i as usize];
                //println!("sprite row is {:b}", sprite_row);
                for j in 0 .. 8
                {
                    let sprite_pixel = sprite_row & (0b1 << (7-j));
                    let current_pixel = self.screen[vy as usize * 64 + vx as usize];
                    if current_pixel == true
                    {
                        if sprite_pixel == 0
                        {
                            has_changed_set_pixel_to_unset = true;
                        }
                    }
                    if sprite_pixel != 0
                    {
                        self.screen[vy as usize * 64 + vx as usize] = (sprite_pixel != 0) ^ self.screen[vy as usize * 64 + vx as usize];
                    }
                    vx += 1;
                    vx = vx % 64;
                }
                vx = self.registers[x as usize];
                vy += 1;
                vy = vy % 32;
            }
            if has_changed_set_pixel_to_unset
            {
                self.registers[15] = 1;
            }
            else
            {
                self.registers[15] = 0;
            }
            self.program_counter += 2;
        }
        //EX9E
        else if opcode & 0xF0FF == 0xE09E
        {
            let x = (opcode & 0x0F00) >> 8;
            if self.keys[self.registers[x as usize] as usize] ==  true
            {
                self.program_counter += 4;
            }
            else
            {
                self.program_counter += 2;
            }
        }
        //EXA1
        else if opcode & 0xF0FF == 0xE0A1
        {
            let x = (opcode & 0x0F00) >> 8;
            if self.keys[self.registers[x as usize] as usize] !=  true
            {
                self.program_counter += 4;
            }
            else
            {
                self.program_counter += 2;
            }
        }
        //FX07
        else if opcode & 0xF0FF == 0xF007
        {
            let x = (opcode & 0x0F00) >> 8;
            self.registers[x as usize] = self.delay_timer;
            self.program_counter += 2;
        }
        //FX0A
        else if opcode & 0xF0FF == 0xF00A
        {
            let x = (opcode & 0x0F00) >> 8;
            //TODO
            self.registers[x as usize] = 0;
            //panic!("waiting for input");
            self.program_counter += 2;
        }
        //FX15
        else if opcode & 0xF0FF == 0xF015
        {
            let x = (opcode & 0x0F00) >> 8;
            self.delay_timer = self.registers[x as usize];
            self.program_counter += 2;
        }
        //FX15
        else if opcode & 0xF0FF == 0xF018
        {
            let x = (opcode & 0x0F00) >> 8;
            self.sound_timer = self.registers[x as usize];
            self.program_counter += 2;
        }
        //FX1E
        else if opcode & 0xF0FF == 0xF01E
        {
            let x = (opcode & 0x0F00) >> 8;
            self.address_register += self.registers[x as usize] as u16;
            self.program_counter += 2;
        }
        //FX29
        else if opcode & 0xF0FF == 0xF029
        {
            let x = (opcode & 0x0F00) >> 8;
            self.address_register = self.font_data_base_address + (5 * self.registers[x as usize] as u16);
            self.program_counter += 2;
        }
        //FX33
        else if opcode & 0xF0FF == 0xF033
        {
            let x = (opcode & 0x0F00) >> 8;
            let hundreds : u8 = self.registers[x as usize] / 100;
            let tens : u8 = (self.registers[x as usize] - hundreds * 100) / 10;
            let ones : u8 = (self.registers[x as usize] - hundreds * 100) - tens * 10;
            self.memory[self.address_register as usize + 0] = hundreds;
            self.memory[self.address_register as usize + 1] = tens;
            self.memory[self.address_register as usize + 2] = ones;
            self.program_counter += 2;
        }
        //FX55
        else if opcode & 0xF0FF == 0xF055
        {
            let x = (opcode & 0x0F00) >> 8;
            for i in 0 .. x + 1
            {
                self.memory[self.address_register as usize + i as usize] = self.registers[i as usize];
            }
            self.program_counter += 2;
        }
        //FX65
        else if opcode & 0xF0FF == 0xF065
        {
            let x = (opcode & 0x0F00) >> 8;
            for i in 0 .. x + 1
            {
                self.registers[i as usize] = self.memory[self.address_register as usize + i as usize];
            }
            self.program_counter += 2;
        }
        //other
        else
        {
            //panic!("Not found opcode.  {:#06X} ", opcode);
            self.program_counter += 2;
        }
    }

    #[allow(dead_code)]
    pub fn screen_width(&self) -> u32
    {
        64
    }

    #[allow(dead_code)]
    pub fn screen_height(&self) -> u32
    {
        32
    }

    #[allow(dead_code)]
    pub fn get_video_buffer_as_rgba(&self) -> Vec<u8>
    {
        let print_debug = false;
        let mut image_data : Vec<u8> = Vec::with_capacity( (self.screen_width() * self.screen_height() * 4) as usize );
        for j in 0 .. self.screen_height()
        {
            for i in 0 .. self.screen_width()
            {
                let u = i;
                let v = self.screen_height() -1 - j;
                if self.screen[(u + v * 64) as usize] == false
                {
                    image_data.push(0);
                    image_data.push(0);
                    image_data.push(0);
                    image_data.push(255);
                }
                else
                {
                    image_data.push(255);
                    image_data.push(255);
                    image_data.push(255);
                    image_data.push(255);
                }

                if print_debug
                {
                    if self.screen[(i + j * 64) as usize] == false
                    {
                        print!("0");
                    }
                    else
                    {
                        print!("1");
                    }
                }
            }
            if print_debug
            {
                print!("\r\n");
            }
        }
        if print_debug
        {
            print!("\r\n");
        }
        return image_data;
    }
}

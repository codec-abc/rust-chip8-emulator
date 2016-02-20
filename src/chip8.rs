use std::iter;
use rand::*;

#[allow(dead_code)]
pub struct Chip8
{
    memory : Vec<u8>,
    registers : Vec<u8>,
    address_register : u16,
    program_counter : u16,
    delay_timer : u8,
    sound_timer : u8,
    stack_pointer : u16,
    stack : Vec<u16>,
    screen : Vec<bool>,
    keys : Vec<bool>,
}

impl Chip8
{
    #[allow(dead_code)]
    pub fn new() -> Chip8
    {
        Chip8
        {
            registers : iter::repeat(0).take(16).collect::<Vec<u8>>(),
            address_register : 0,

            delay_timer : 0,
            sound_timer : 0,

            program_counter : 0x200,

            stack_pointer : 0,
            stack : iter::repeat(0).take(16).collect::<Vec<u16>>(),
            screen : iter::repeat(false).take(64 * 32).collect::<Vec<bool>>(),
            keys : iter::repeat(false).take(16).collect::<Vec<bool>>(),
            memory :  iter::repeat(0).take(4096).collect::<Vec<u8>>(),
        }
    }

    #[allow(dead_code)]
    pub fn fetch_opcode(&mut self) -> u16
    {
        let upper_byte_opcode = self.memory[self.program_counter as usize] as u16;
        let lower_byte_opcode = self.memory[(self.program_counter + 1) as usize] as u16;
        //self.program_counter = self.program_counter + 2;
        upper_byte_opcode << 8 | lower_byte_opcode
    }

    #[allow(dead_code)]
    pub fn execute_opcode(&mut self, opcode : u16)
    {
        if opcode == 0x00E0
        {
            for i in 0 .. self.screen.len()
            {
                self.screen[i] = false;
            }
        }
        else if opcode == 0x00EE
        {
            // todo
        }
        else if opcode & 0x1000 == 0x1000
        {
            self.program_counter = opcode & 0x0FFF;
            //??
        }
        else if opcode & 0x1000 == 0x2000
        {
            //??
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
            let y = (opcode & 0x00F0) >> 8;
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
            let nn = (opcode & 0x00FF) as u8;
            let x = (opcode & 0x0F00) >> 8;
            self.registers[x as usize] += nn;
            self.program_counter += 2;
        }
        //8XY0
        else if opcode & 0xF00F == 0x8000
        {
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 8;
            self.registers[x as usize] = self.registers[y as usize];
            self.program_counter += 2;
        }
        //8XY1
        else if opcode & 0xF00F == 0x8001
        {
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 8;
            self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
            self.program_counter += 2;
        }
        //8XY2
        else if opcode & 0xF00F == 0x8002
        {
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 8;
            self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize];
            self.program_counter += 2;
        }
        //8XY3
        else if opcode & 0xF00F == 0x8003
        {
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 8;
            self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize];
            self.program_counter += 2;
        }
        //8XY4
        else if opcode & 0xF00F == 0x8004
        {
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 8;
            let has_carry = (self.registers[x as usize] as u16 + self.registers[y as usize] as u16) > 255;
            self.registers[x as usize] = self.registers[x as usize] + self.registers[y as usize];
            if has_carry
            {
                self.registers[15] = 1;
            }
            else
            {
                self.registers[15] = 0;
            }
            self.program_counter += 2;
        }
        //8XY5
        else if opcode & 0xF00F == 0x8005
        {
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 8;
            let has_borrow = (self.registers[x as usize] as u16) < (self.registers[y as usize] as u16);
            self.registers[x as usize] = self.registers[x as usize] - self.registers[y as usize];
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
            let y = (opcode & 0x00F0) >> 8;
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
            let y = (opcode & 0x00F0) >> 8;
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
            self.registers[x as usize] = thread_rng().gen::<u8>() & nn;
            self.program_counter += 2;
        }
        //DXYN
    }
}

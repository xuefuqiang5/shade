use crossterm::{cursor::MoveTo, execute, terminal};
use core::error;
use std::{io::{Result, Stdout, stdout}, process::Stdio};
use crate::triangle::{Point, Triangle};
pub struct Screen {
    height: u16,
    width: u16, 
    stdout: Stdout,
}
const RAMP_70: &[u8] = br#" 
    .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczMWO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^'`. 
"#;
impl Screen {
    pub fn new(x: u16, y: u16) -> Result<Self> {
        let (width, height) = terminal::size().unwrap();
        if x > width || y > height {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Mismatched window size"
            ));
        }
        Ok(Self {
            height: x,
            width: y,
            stdout: stdout(),
        })
    }
    pub fn depth2char(depth: &f32) -> Result<char>{
        if !(0.0..1.0).contains(depth) {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "out of valid range"
                )
            );
        } 
        let mut index = (depth * RAMP_70.len() as f32) as usize;
        if index >= RAMP_70.len() { index -= 1; }
        Ok(RAMP_70[index] as char)
    }
    pub fn show_at(&mut self, p: &Point) -> std::io::Result<()> {
        // top left cell cor = (1, 1);
        let (width, height) = terminal::size().unwrap();
        let(offset_w, offset_h) = ((width-self.width)/2, (height-self.height)/2); 
        execute!(
            self.stdout, 
            MoveTo(
                (p.position[1] as u16) + offset_w, 
                (p.position[0] as u16) + offset_h,  
            ),
        )?;
        Ok(())
    }
}

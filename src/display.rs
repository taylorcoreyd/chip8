pub struct Display {
    buff: [[bool; 64]; 32]
}

impl Display {
    pub fn new(display_buffer: [[bool; 64]; 32]) -> Display {
        let disp = Display { buff: display_buffer };
        return disp;
    }

    pub fn render(&self) {
        for row in self.buff.iter() {
            for pix in row.iter() {
                if *pix { print!("x"); } else { print!(" "); }
            }
            print!("\n");
        }
    }
}
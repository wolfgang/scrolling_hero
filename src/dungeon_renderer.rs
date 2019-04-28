use std::io::{Write};
use std::rc::Rc;
use std::cell::{RefCell};
use std::cmp::{min, max};

pub struct DungeonRenderer<'a> {
    dungeon: &'a Vec<Vec<u16>>,
    player_pos: Rc<RefCell<(u32, u32)>>
}

impl<'a> DungeonRenderer<'a> {
    pub fn new(dungeon: &'a Vec<Vec<u16>>, player_pos: &Rc<RefCell<(u32, u32)>>) -> DungeonRenderer<'a> {
        DungeonRenderer {dungeon: dungeon, player_pos: Rc::clone(player_pos)}
    }
    pub fn render(&self, writer: &mut Write, from: i32, count: u32) -> std::io::Result<()> {
        let end_row = min(self.dungeon.len(), (from  + count as i32) as usize);
        let start_row = max(from, 0) as usize;
        for (y, row) in self.dungeon[start_row..end_row].iter().enumerate() {
            let mut row_str = String::with_capacity(row.len());
            for (x, cell) in row.iter().enumerate() {
                if (x as u32, y as u32 + start_row as u32) == *self.player_pos.borrow() {
                    row_str.push('@');
                }
                else {
                    if *cell == 0 { row_str.push('.')}
                    if *cell == 1 { row_str.push('#')}                
                }
            }

            writer.write_fmt(format_args!("{}\n", &row_str))?;
        }

        Ok(())
    }
}

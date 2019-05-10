use std::cell::RefCell;
use std::rc::Rc;

use crate::types::{DungeonLayout, Position};

pub type DungeonProvider = Iterator<Item=(DungeonLayout, Position)>;

pub struct SingleDungeonProvider {
    dungeon: DungeonLayout,
    player_position: Position
}

impl SingleDungeonProvider {
    pub fn new(dungeon: DungeonLayout, player_position: Position) -> SingleDungeonProvider {
        SingleDungeonProvider { dungeon, player_position }
    }

    pub fn shared(dungeon: DungeonLayout, player_position: Position) -> Rc<RefCell<DungeonProvider>> {
        Rc::new(RefCell::new(SingleDungeonProvider::new(dungeon, player_position))) as Rc<RefCell<DungeonProvider>>
    }

}

impl Iterator for SingleDungeonProvider {
    type Item = (DungeonLayout, Position);

    fn next(&mut self) -> Option<(DungeonLayout, Position)> {
        Some((self.dungeon.clone(), self.player_position))
    }
}

pub struct MultiDungeonProvider {
    current_index: usize,
    dungeons: Vec<(DungeonLayout, Position)>,
}

impl MultiDungeonProvider {
    pub fn new(dungeons: Vec<(DungeonLayout, Position)>) -> MultiDungeonProvider {
        MultiDungeonProvider { current_index: 0, dungeons }
    }

    pub fn shared(dungeons: Vec<(DungeonLayout, Position)>) -> Rc<RefCell<DungeonProvider>> {
        Rc::new(RefCell::new(MultiDungeonProvider::new(dungeons))) as Rc<RefCell<DungeonProvider>>
    }
}

impl Iterator for MultiDungeonProvider {
    type Item = (DungeonLayout, Position);

    fn next(&mut self) -> Option<(DungeonLayout, Position)> {
        let index = self.current_index;
        self.current_index += 1;
        Some(self.dungeons[index].clone())
    }
}

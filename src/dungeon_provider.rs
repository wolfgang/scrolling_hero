use std::cell::RefCell;
use std::rc::Rc;

use crate::types::{Dungeon, Position};

pub type DungeonProvider = Iterator<Item=(Dungeon, Position)>;

pub struct SingleDungeonProvider {
    dungeon: Dungeon,
    player_position: Position
}

impl SingleDungeonProvider {
    pub fn new(dungeon: Dungeon, player_position: Position) -> SingleDungeonProvider {
        SingleDungeonProvider { dungeon, player_position }
    }

    pub fn shared(dungeon: Dungeon, player_position: Position) -> Rc<RefCell<DungeonProvider>> {
        Rc::new(RefCell::new(SingleDungeonProvider::new(dungeon, player_position))) as Rc<RefCell<DungeonProvider>>
    }

}

impl Iterator for SingleDungeonProvider {
    type Item = (Dungeon, Position);

    fn next(&mut self) -> Option<(Dungeon, Position)> {
        Some((self.dungeon.clone(), self.player_position))
    }
}

pub struct MultiDungeonProvider {
    current_index: usize,
    dungeons: Vec<(Dungeon, Position)>,
}

impl MultiDungeonProvider {
    pub fn new(dungeons: Vec<(Dungeon, Position)>) -> MultiDungeonProvider {
        MultiDungeonProvider { current_index: 0, dungeons }
    }

    pub fn shared(dungeons: Vec<(Dungeon, Position)>) -> Rc<RefCell<DungeonProvider>> {
        Rc::new(RefCell::new(MultiDungeonProvider::new(dungeons))) as Rc<RefCell<DungeonProvider>>
    }
}

impl Iterator for MultiDungeonProvider {
    type Item = (Dungeon, Position);

    fn next(&mut self) -> Option<(Dungeon, Position)> {
        let index = self.current_index;
        self.current_index += 1;
        Some(self.dungeons[index].clone())
    }
}

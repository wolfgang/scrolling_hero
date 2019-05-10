use crate::types::Dungeon;

pub trait DungeonProvider {
    fn next(&self) -> &Dungeon;
}


pub struct SingleDungeonProvider {
    dungeon: Dungeon
}

impl DungeonProvider for SingleDungeonProvider {
    fn next(&self) -> &Dungeon {
        &self.dungeon
    }
}

impl SingleDungeonProvider {
    pub fn new(dungeon: Dungeon) -> SingleDungeonProvider {
        SingleDungeonProvider { dungeon }
    }
}

use crate::types::Dungeon;

pub trait DungeonProvider {
    fn next(&mut self) -> Dungeon;
}


pub struct SingleDungeonProvider {
    dungeon: Dungeon
}

impl SingleDungeonProvider {
    pub fn new(dungeon: Dungeon) -> SingleDungeonProvider {
        SingleDungeonProvider { dungeon }
    }
}

impl DungeonProvider for SingleDungeonProvider {
    fn next(&mut self) -> Dungeon {
        self.dungeon.clone()
    }
}

pub struct MultiDungeonProvider {
    current_index: usize,
    dungeons: Vec<Dungeon>,
}

impl MultiDungeonProvider {
    pub fn new(dungeons: Vec<Dungeon>) -> MultiDungeonProvider {
        MultiDungeonProvider { current_index: 0, dungeons }
    }
}

impl DungeonProvider for MultiDungeonProvider {
    fn next(&mut self) -> Dungeon {
        let index = self.current_index;
        self.current_index += 1;
        self.dungeons[index].clone()
    }
}

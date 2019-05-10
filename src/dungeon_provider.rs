use crate::types::Dungeon;

pub type DungeonProvider = Iterator<Item=Dungeon>;

pub struct SingleDungeonProvider {
    dungeon: Dungeon
}

impl SingleDungeonProvider {
    pub fn new(dungeon: Dungeon) -> SingleDungeonProvider {
        SingleDungeonProvider { dungeon }
    }
}

impl Iterator for SingleDungeonProvider {
    type Item = Dungeon;

    fn next(&mut self) -> Option<Dungeon> {
        Some(self.dungeon.clone())
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

impl Iterator for MultiDungeonProvider {
    type Item = Dungeon;

    fn next(&mut self) -> Option<Dungeon> {
        let index = self.current_index;
        self.current_index += 1;
        Some(self.dungeons[index].clone())
    }
}

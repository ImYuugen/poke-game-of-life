use crate::engine::game::Game;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Type {
    Normal,
    Fire,
    Water,
    Grass,
    Electric,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dark,
    Dragon,
    Steel,
    Fairy,
}

impl Type {
    pub const TYPES: [Self; 18] = [
        Self::Normal,
        Self::Fire,
        Self::Water,
        Self::Grass,
        Self::Electric,
        Self::Ice,
        Self::Fighting,
        Self::Poison,
        Self::Ground,
        Self::Flying,
        Self::Psychic,
        Self::Bug,
        Self::Rock,
        Self::Ghost,
        Self::Dark,
        Self::Dragon,
        Self::Steel,
        Self::Fairy,
    ];

    /// Pokemon types weaknesses and strengths.
    ///
    /// If a type doesn't have an entry for another type in its HashMap,
    /// the multiplier is 1x.
    /// If a type has an entry for another type in its HashMap,
    /// the multiplier is the value of the entry.
    pub fn type_table() -> HashMap<Self, HashMap<Self, f32>> {
        let mut table: HashMap<Self, HashMap<Self, f32>> = HashMap::new();

        //Normal
        table.insert(Self::Normal, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Rock, 0.5);
            hmap.insert(Self::Ghost, 0.0);
            hmap.insert(Self::Steel, 0.5);
            hmap
        });
        //Fire
        table.insert(Self::Fire, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Fire, 0.5);
            hmap.insert(Self::Water, 0.5);
            hmap.insert(Self::Grass, 2.0);
            hmap.insert(Self::Ice, 2.0);
            hmap.insert(Self::Bug, 2.0);
            hmap.insert(Self::Rock, 0.5);
            hmap.insert(Self::Dragon, 0.5);
            hmap.insert(Self::Steel, 2.0);
            hmap
        });
        //Water
        table.insert(Self::Water, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Fire, 2.0);
            hmap.insert(Self::Water, 0.5);
            hmap.insert(Self::Grass, 0.5);
            hmap.insert(Self::Ground, 2.0);
            hmap.insert(Self::Rock, 2.0);
            hmap.insert(Self::Dragon, 0.5);
            hmap
        });
        //Grass
        table.insert(Self::Grass, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Fire, 0.5);
            hmap.insert(Self::Water, 2.0);
            hmap.insert(Self::Grass, 0.5);
            hmap.insert(Self::Poison, 0.5);
            hmap.insert(Self::Ground, 2.0);
            hmap.insert(Self::Flying, 0.5);
            hmap.insert(Self::Bug, 0.5);
            hmap.insert(Self::Rock, 2.0);
            hmap.insert(Self::Dragon, 0.5);
            hmap.insert(Self::Steel, 0.5);
            hmap
        });
        //Electric
        table.insert(Self::Electric, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Water, 2.0);
            hmap.insert(Self::Electric, 0.5);
            hmap.insert(Self::Grass, 0.5);
            hmap.insert(Self::Ground, 0.0);
            hmap.insert(Self::Flying, 2.0);
            hmap.insert(Self::Dragon, 0.5);
            hmap
        });
        //Ice
        table.insert(Self::Ice, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Fire, 0.5);
            hmap.insert(Self::Water, 0.5);
            hmap.insert(Self::Grass, 2.0);
            hmap.insert(Self::Ice, 0.5);
            hmap.insert(Self::Ground, 2.0);
            hmap.insert(Self::Flying, 2.0);
            hmap.insert(Self::Dragon, 2.0);
            hmap.insert(Self::Steel, 0.5);
            hmap
        });
        //Fighting
        table.insert(Self::Fighting, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Normal, 2.0);
            hmap.insert(Self::Ice, 2.0);
            hmap.insert(Self::Poison, 0.5);
            hmap.insert(Self::Flying, 0.5);
            hmap.insert(Self::Psychic, 0.5);
            hmap.insert(Self::Bug, 0.5);
            hmap.insert(Self::Rock, 2.0);
            hmap.insert(Self::Ghost, 0.0);
            hmap.insert(Self::Dark, 2.0);
            hmap.insert(Self::Steel, 2.0);
            hmap.insert(Self::Fairy, 0.5);
            hmap
        });
        //Poison
        table.insert(Self::Poison, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Grass, 2.0);
            hmap.insert(Self::Poison, 0.5);
            hmap.insert(Self::Ground, 0.5);
            hmap.insert(Self::Rock, 0.5);
            hmap.insert(Self::Ghost, 0.5);
            hmap.insert(Self::Steel, 0.0);
            hmap
        });
        //Ground
        table.insert(Self::Ground, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Fire, 2.0);
            hmap.insert(Self::Electric, 2.0);
            hmap.insert(Self::Grass, 0.5);
            hmap.insert(Self::Poison, 2.0);
            hmap.insert(Self::Flying, 0.0);
            hmap.insert(Self::Bug, 0.5);
            hmap.insert(Self::Rock, 2.0);
            hmap
        });
        //Flying
        table.insert(Self::Flying, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Electric, 0.5);
            hmap.insert(Self::Grass, 2.0);
            hmap.insert(Self::Fighting, 2.0);
            hmap.insert(Self::Bug, 2.0);
            hmap.insert(Self::Rock, 0.5);
            hmap
        });
        //Psychic
        table.insert(Self::Psychic, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Fighting, 2.0);
            hmap.insert(Self::Psychic, 0.5);
            hmap.insert(Self::Ghost, 2.0);
            hmap.insert(Self::Dark, 0.0);
            hmap
        });
        //Bug
        table.insert(Self::Bug, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Fire, 0.5);
            hmap.insert(Self::Grass, 2.0);
            hmap.insert(Self::Fighting, 0.5);
            hmap.insert(Self::Poison, 0.5);
            hmap.insert(Self::Flying, 0.5);
            hmap.insert(Self::Psychic, 2.0);
            hmap.insert(Self::Ghost, 0.5);
            hmap.insert(Self::Dark, 2.0);
            hmap.insert(Self::Steel, 0.5);
            hmap.insert(Self::Fairy, 0.5);
            hmap
        });
        //Rock
        table.insert(Self::Rock, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Fire, 2.0);
            hmap.insert(Self::Ice, 2.0);
            hmap.insert(Self::Fighting, 0.5);
            hmap.insert(Self::Ground, 0.5);
            hmap.insert(Self::Flying, 2.0);
            hmap.insert(Self::Bug, 2.0);
            hmap
        });
        //Ghost
        table.insert(Self::Ghost, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Normal, 0.0);
            hmap.insert(Self::Psychic, 2.0);
            hmap.insert(Self::Ghost, 2.0);
            hmap.insert(Self::Dark, 0.5);
            hmap
        });
        //Dragon
        table.insert(Self::Dragon, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Dragon, 2.0);
            hmap.insert(Self::Steel, 0.5);
            hmap.insert(Self::Fairy, 0.0);
            hmap
        });
        //Dark
        table.insert(Self::Dark, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Fighting, 0.5);
            hmap.insert(Self::Psychic, 2.0);
            hmap.insert(Self::Ghost, 2.0);
            hmap.insert(Self::Dark, 0.5);
            hmap.insert(Self::Fairy, 0.5);
            hmap
        });
        //Steel
        table.insert(Self::Steel, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Fire, 0.5);
            hmap.insert(Self::Water, 0.5);
            hmap.insert(Self::Electric, 0.5);
            hmap.insert(Self::Ice, 2.0);
            hmap.insert(Self::Rock, 2.0);
            hmap.insert(Self::Steel, 0.5);
            hmap.insert(Self::Fairy, 2.0);
            hmap
        });
        //Fairy
        table.insert(Self::Fairy, {
            let mut hmap: HashMap<Self, f32> = HashMap::new();
            hmap.insert(Self::Fighting, 2.0);
            hmap.insert(Self::Dragon, 2.0);
            hmap.insert(Self::Dark, 2.0);
            hmap.insert(Self::Steel, 0.5);
            hmap
        });

        table
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub cell_type: Type,
    pub health: f32,
    pub changed: bool,
}

impl Cell {
    pub fn new(cell_type: Type) -> Self {
        Self {
            cell_type,
            health: 5.0,
            changed: true,
        }
    }

    pub fn get_damage(game: &Game, c1: Self, c2: Self) -> &f32 {
        if let Some(hmap) = game.type_table.get(&c1.cell_type) {
            if let Some(damage) = hmap.get(&c2.cell_type) {
                damage
            } else {
                &1.0
            }
        } else {
            &1.0
        }
    }
}

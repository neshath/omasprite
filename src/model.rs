use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Workspace {
    Sprite,
    World,
    Logic,
    Character,
    Effects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapStyle {
    Overworld,
    Town,
    Interior,
}

impl MapStyle {
    pub fn label(self) -> &'static str {
        match self {
            Self::Overworld => "OVERWORLD",
            Self::Town => "TOWN MAP",
            Self::Interior => "INTERIOR",
        }
    }
}

impl Workspace {
    pub fn label(self) -> &'static str {
        match self {
            Self::Sprite => "SPRITE LAB",
            Self::World => "WORLD BUILDER",
            Self::Logic => "VISUAL LOGIC",
            Self::Character => "CHARACTER WORKSHOP",
            Self::Effects => "LIGHTING & FX",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Pencil,
    Fill,
    Eraser,
    Select,
    Stamp,
    Light,
    Shadow,
    Path,
    Particle,
}

impl Tool {
    pub fn label(self) -> &'static str {
        match self {
            Self::Pencil => "Pencil",
            Self::Fill => "Fill",
            Self::Eraser => "Eraser",
            Self::Select => "Select",
            Self::Stamp => "Stamp",
            Self::Light => "Pixel light",
            Self::Shadow => "Shadow brush",
            Self::Path => "Path / collision",
            Self::Particle => "Particles",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub level: u8,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: "little-adventure".into(),
            path: "~/Games/little-adventure".into(),
            level: 3,
        }
    }
}

pub const LEVELS: &[(u8, &str, &str)] = &[
    (1, "MAKE A SPRITE", "Pencil, fill, erase, play"),
    (2, "BRING IT TO LIFE", "Layers, palettes, animation"),
    (3, "BUILD A WORLD", "Tilemaps, collision, camera"),
    (5, "SET THE MOOD", "Lighting, shadows, weather"),
    (10, "MAKE IT REACT", "NPCs, dialogue, quests"),
    (20, "GO DEEPER", "Visual logic, Lua, plugins"),
];

//! Deterministic tilemap authoring primitives shared by editor and runtime.
use crate::world::Scene;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Camera {
    pub center: [f32; 2],
    pub viewport: [f32; 2],
    pub zoom: f32,
}
impl Camera {
    pub fn new(viewport: [f32; 2]) -> Self {
        Self {
            center: [8.0, 8.0],
            viewport,
            zoom: 1.0,
        }
    }
    pub fn follow(&mut self, target: [f32; 2], map: [usize; 2]) {
        let half = [
            self.viewport[0] / self.zoom / 2.0,
            self.viewport[1] / self.zoom / 2.0,
        ];
        self.center[0] = target[0].clamp(half[0], map[0] as f32 - half[0]);
        self.center[1] = target[1].clamp(half[1], map[1] as f32 - half[1]);
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TileStamp {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<u8>,
    pub collision: Vec<bool>,
}
impl TileStamp {
    pub fn apply(&self, scene: &mut Scene, origin: [usize; 2]) -> Result<(), String> {
        if self.width == 0
            || self.height == 0
            || self.tiles.len() != self.width * self.height
            || self.collision.len() != self.tiles.len()
        {
            return Err("Invalid tile stamp".into());
        }
        for sy in 0..self.height {
            for sx in 0..self.width {
                let x = origin[0] + sx;
                let y = origin[1] + sy;
                if x >= 16 || y >= 16 {
                    continue;
                }
                let i = sy * self.width + sx;
                let d = y * 16 + x;
                if self.tiles[i] > 4 {
                    return Err("Stamp contains invalid tile".into());
                }
                scene.tiles[d] = self.tiles[i];
                scene.collision[d] = self.collision[i];
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stamp_paints_tiles_and_collision_with_clipping() {
        let mut scene = Scene::default();
        let stamp = TileStamp {
            width: 2,
            height: 2,
            tiles: vec![1, 3, 4, 0],
            collision: vec![false, true, true, false],
        };
        stamp.apply(&mut scene, [15, 15]).unwrap();
        assert_eq!(scene.tiles[15 * 16 + 15], 1);
        assert!(!scene.collision[15 * 16 + 15]);
        assert!(stamp.apply(&mut scene, [0, 0]).is_ok());
    }
    #[test]
    fn camera_follows_and_clamps_to_map() {
        let mut camera = Camera::new([6.0, 4.0]);
        camera.follow([15.0, 15.0], [16, 16]);
        assert_eq!(camera.center, [13.0, 14.0]);
        camera.follow([0.0, 0.0], [16, 16]);
        assert_eq!(camera.center, [3.0, 2.0]);
    }
}

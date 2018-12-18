use std::f32::consts::PI;
use std::f32;

use amethyst_core::{
    nalgebra::{Vector2},
    specs::{Component, DenseVecStorage},
};

type Basis = [Vector2<f32>, Vector2<f32>]; 

pub struct Tile {
  coord: Vector2<i16>
}

pub struct Tiles {
  basis: Basis,
  tiles: Vec<Tile>
}

pub enum TileMap {
  QuadMap(Tiles),
  HexMap(Tiles)
}

impl Component for TileMap {
  type Storage = DenseVecStorage<Self>;
}

struct TileMapBuilder {
  side_len: u16,
  size: u16,
  is_hex: bool,
}

impl TileMapBuilder {

  fn quad(&mut self) -> Self {
    self.hex = false;
    self
  }

  fn hex(&mut self) -> Self {
    self.hex = true;
    self
  }

  fn with_side_len(&mut self, side_len: i16) -> Self 
    self.side_len = side_len;
    self
  }

  fn with_size(&mut self, size: i16) -> Self 
    self.size = true;
    self
  }

  build(&self) -> TileMap {
    if self.is_hex {
      let basis = [
        Vector2::<f32>::new(f32::cos(angle), f32::sin(angle)), 
        Vector2::<f32>::new(0.0, 1.0)
      ];
      let tiles = get_hex_map_tiles(basis, self.side_len);
      HexMap(Tiles {basis, tiles})
    } else {
      let basis = [
        Vector2::<f32>::new(1.0, 0.0), 
        Vector2::<f32>::new(0.0, 1.0)
      ];
      let tiles = get_quad_map_tiles(basis, self.side_len);
      QuadMap(Tiles {basis, tiles})
    }
  }
}

fn get_quad_map_tiles(basis: &basis, uside: usize) -> Vec<Tile> {
  let [&basis_x, &basis_y] = basis;
  let angle = PI / 6.0;
  let side = uside as i16;
  let mut faces:Vec<Face> = Vec::new();
  let x_start: i16 = -(side - 1);
  let x_end: i16 = side;
  for x in x_start..x_end {
    let y_start = -(side-1);
    let y_end = side;
    for y in y_start..y_end {
      let b = Vector2::<i16>::new(x, y);
      let world_point = basis_x * x as f32 + basis_y * y as f32;
      faces.push(Tile {coord: b);
    }
  }
  faces
}
fn get_hex_map_faces(basis: &basis, uside: usize) -> Vec<Tile> {
  let [&basis_x, &basis_y] = basis;
  let angle = PI / 6.0;
  let side = uside as i16;
  let mut faces:Vec<Face> = Vec::new();
  let x_start: i16 = -(side - 1);
  let x_end: i16 = side;
  for x in x_start..x_end {
    let y_start = if x < 0 {
      -x_start + x
    } else {
      -x_start
    };
    let y_end = if x > 0 {
      x_end - x - 1
    } else {
      x_end - 1
    };
    for y in y_start..y_end {
      let b = Vector2::<i16>::new(x, y);
      
      let world_point = basis_x * x as f32 + basis_y * y as f32;
      faces.push(Tile {coord: b});
    }
  }
  faces
}

impl TileMap {
  fn build() -> TileMapBuilder {
    TileMapBuilder {
      size:8,
      is_hex: true
    }
  }
}


use crate::prelude::*;
use log::error;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Tilemap<Image: Debug + Clone> {
    ///index to `image`
    tiles: Vec<usize>,
    ///flags per tile, same size as `tiles`
    flags: Vec<u32>,
    size: MapSize,
    ///number of tiles visible on screen
    visible_size: MapSize,
    ///top left offset for rendering (in tiles)
    offset: MapPosition,
    images: Vec<Rc<Image>>,
    tile_size: (u32, u32),
    subtile_offset: (i16, i16),
    default_start: MapPosition,
    exits: Vec<MapExit>,
}

impl<Image: Debug + Clone> Tilemap<Image> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tiles: Vec<usize>,
        flags: Vec<u32>,
        size: MapSize,
        tile_idx_image_name: Vec<String>,
        tileset: Tileset<Image>,
        render_size: (u32, u32),
        default_start: MapPosition,
        exits: Vec<MapExit>,
    ) -> Result<Self, GameUtilError> {
        let mut images = vec![];

        let mut missing = vec![];
        for name in tile_idx_image_name {
            if let Some(img) = tileset.find_by_name(&name) {
                images.push(Rc::new(img.clone()));
            } else {
                missing.push(name);
            }
        }
        if !missing.is_empty() {
            return Err(GameUtilError::InvalidTileset(
                String::from("from code"),
                missing,
            ));
        }

        let mut visible_size = MapSize::new(
            render_size.0 / tileset.tilesize().0,
            render_size.1 / tileset.tilesize().1,
        );
        visible_size.w = visible_size.w.min(size.w);
        visible_size.h = visible_size.h.min(size.h);

        Ok(Self {
            tiles,
            flags,
            size,
            tile_size: tileset.tilesize(),
            visible_size,
            offset: MapPosition::new(0, 0),
            images,
            subtile_offset: (0, 0),
            default_start,
            exits,
        })
    }
}

impl<Image: Debug + Clone> Tilemap<Image> {
    /// Pixel coord for tile
    /// Result may be offscreen, before or after
    pub fn px_for_tile<P: Into<MapPosition>>(&self, tile: P) -> (isize, isize) {
        let tile = tile.into();
        (
            (self.tile_size.0 * tile.x) as isize - (self.tile_size.0 * self.offset.x) as isize + self.subtile_offset.0 as isize,
            (self.tile_size.1 * tile.y) as isize - (self.tile_size.1 * self.offset.y) as isize + self.subtile_offset.1 as isize,
        )
    }

    /// Pixel coord for tile, ignoring subtile offset
    /// Result may be offscreen, before or after
    pub fn orig_px_for_tile<P: Into<MapPosition>>(&self, tile: P) -> (isize, isize) {
        let tile = tile.into();
        (
            (self.tile_size.0 * tile.x) as isize - (self.tile_size.0 * self.offset.x) as isize,
            (self.tile_size.1 * tile.y) as isize - (self.tile_size.1 * self.offset.y) as isize,
        )
    }

    /// Returns the pos of the first tile with at least one pixel visible
    ///
    /// Will match [Tilemap::first_visible_tile] unless a subtile offset is set
    pub fn first_visible_tile(&self) -> MapPosition {
        if self.subtile_offset == (0, 0) {
            self.offset
        } else {
            let (x_offset, y_offset) = self.tiles_visible_from_subtile_offset();
            let mut offset = self.offset;
            offset.x = offset.x.saturating_add_signed(x_offset);
            offset.y = offset.y.saturating_add_signed(y_offset);
            offset
        }
    }

    fn tiles_visible_from_subtile_offset(&self) -> (i32, i32) {
        let x_offset = self.subtile_offset.0 as f64 / self.tile_size.0 as f64;
        let x_offset = if x_offset.is_sign_positive() {
            x_offset.ceil()
        } else {
            x_offset.floor()
        } as i32;
        let y_offset = self.subtile_offset.1 as f64 / self.tile_size.1 as f64;
        let y_offset = if y_offset.is_sign_positive() {
            y_offset.ceil()
        } else {
            y_offset.floor()
        } as i32;
        (x_offset, y_offset)
    }

    /// Returns the pos of the first fully visible tile
    ///
    /// See [Tilemap::first_visible_tile]
    pub fn first_fully_visible_tile(&self) -> MapPosition {
        self.offset
    }

    /// Returns the pos of the last tile with at least one pixel visible
    ///
    /// Will match [Tilemap::last_visible_tile] unless a subtile offset is set
    pub fn last_visible_tile(&self) -> MapPosition {
        let mut offset = self.offset;
        offset.x += self.visible_size.w;
        offset.y += self.visible_size.h;
        if self.subtile_offset == (0, 0) {
            offset
        } else {
            let (x_offset, y_offset) = self.tiles_visible_from_subtile_offset();
            offset.x = offset.x.saturating_add_signed(x_offset);
            offset.y = offset.y.saturating_add_signed(y_offset);
            offset
        }
    }

    /// Returns the pos of the last fully visible tile
    ///
    /// See [Tilemap::last_visible_tile]
    pub fn last_fully_visible_tile(&self) -> MapPosition {
        let mut offset = self.offset;
        offset.x += self.visible_size.w;
        offset.y += self.visible_size.h;
        offset
    }

    /// Returns true if `tile` is inside the map
    pub fn is_inside<P: Into<MapPosition>>(&self, tile: P) -> bool {
        let tile = tile.into();
        tile.x < self.size.w && tile.y < self.size.h
    }

    fn tile_idx<P: Into<MapPosition>>(&self, tile: P) -> Option<usize> {
        let tile = tile.into();
        let i = tile.to_idx(self.size);
        if i < self.tiles.len() {
            Some(i)
        } else {
            None
        }
    }

    fn tile_pos(&self, tile: usize) -> Option<MapPosition> {
        if tile < self.tiles.len() {
            Some(MapPosition::from_idx(tile, self.size))
        } else {
            None
        }
    }

    /// Loops through all visible tiles
    /// calling `render` with the image and px coord
    pub fn draw<F: FnMut(&Image, (isize, isize))>(&self, mut render: F) {
        for x in 0..self.visible_size.w {
            for y in 0..self.visible_size.h {
                let x = x.saturating_add(self.offset.x);
                let y = y.saturating_add(self.offset.y);
                let i = (x + y * self.size.w) as usize;
                if i < self.tiles.len() {
                    render(
                        &self.images[self.tiles[i]],
                        self.update_pos_with_offset(self.onscreen_px_for_tile((x, y))),
                    )
                }
            }
        }
    }

    #[inline]
    pub fn update_pos_with_offset(&self, pos: (isize, isize)) -> (isize, isize) {
        (
            pos.0 - self.subtile_offset.0 as isize,
            pos.1 - self.subtile_offset.1 as isize,
        )
    }

    /// Moves center of visible map to `pos`
    pub fn center_on<P: Into<MapPosition>>(&mut self, pos: P) {
        let pos = pos.into();
        self.offset.x = pos.x.saturating_sub(self.visible_size.w / 2);
        self.offset.y = pos.y.saturating_sub(self.visible_size.h / 2);
        self.offset.x = self.offset.x.min(self.size.w - self.visible_size.w);
        self.offset.y = self.offset.y.min(self.size.h - self.visible_size.h);
    }

    /// Returns a list of tiles matching `flag`
    pub fn all_tiles_with_flag(&self, flag: u32) -> Vec<MapPosition> {
        self.flags
            .iter()
            .enumerate()
            .filter_map(|(i, tile_flag)| {
                if *tile_flag & flag == flag {
                    Some(self.tile_pos(i))
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    /// Returns true if `tile` has a flag of `value`
    pub fn tile_has_flag<P: Into<MapPosition>>(&self, tile: P, value: u32) -> bool {
        let tile = tile.into();
        if let Some(i) = self.tile_idx(tile) {
            value & self.flags[i] == value
        } else {
            false
        }
    }

    /// Returns flag value for `tile`
    pub fn flags_for_tile<P: Into<MapPosition>>(&self, tile: P) -> u32 {
        let tile = tile.into();
        if let Some(i) = self.tile_idx(tile) {
            self.flags[i]
        } else {
            0
        }
    }

    /// Sets the flag value for `tile`
    pub fn set_flag<P: Into<MapPosition>>(&mut self, tile: P, value: u32) {
        let tile = tile.into();
        if let Some(i) = self.tile_idx(tile) {
            self.flags[i] |= value;
        } else {
            error!("set_flag({tile:?}, {value}) outside of map")
        }
    }

    /// Removes specified flags for `tile`
    pub fn clear_flag<P: Into<MapPosition>>(&mut self, tile: P, value: u32) {
        let tile = tile.into();
        if let Some(i) = self.tile_idx(tile) {
            self.flags[i] -= value;
        } else {
            error!("clear_flag({tile:?}, {value}) outside of map")
        }
    }

    /// The default, safe start position on the map
    pub fn default_start(&self) -> MapPosition {
        self.default_start
    }

    /// All exits to other maps
    pub fn exits(&self) -> &Vec<MapExit> {
        &self.exits
    }

    pub fn tile_size(&self) -> (u32, u32) {
        self.tile_size
    }

    pub fn size(&self) -> MapSize {
        self.size
    }

    /// Sets a pixel offset for drawing
    /// Primarily designed for smoothing animation the map when a character or camera is moving
    pub fn set_subtile_offset(&mut self, subtile_offset: (i16, i16)) {
        self.subtile_offset = subtile_offset;
    }

    /// Returns a pixel offset for drawing
    pub fn subtile_offset(&self) -> (i16, i16) {
        self.subtile_offset
    }
}

impl TilemapFile {
    pub fn into_tilemap<Image: Debug + Clone>(
        self,
        tileset: &Tileset<Image>,
        visible_area_px: (u32, u32),
    ) -> Result<Tilemap<Image>, GameUtilError> {
        let mut images = vec![];
        let mut flag_map = HashMap::new();
        let mut missing = vec![];
        for (i, tile) in self.tiles.iter().enumerate() {
            if let Some(img) = tileset.find_by_name(&tile.image) {
                flag_map.insert(i, tile.flags);
                images.push(Rc::new(img.clone()));
            } else {
                missing.push(tile.image.clone());
            }
        }
        if !missing.is_empty() {
            return Err(GameUtilError::InvalidTileset(self.name.clone(), missing));
        }
        let size: MapSize = (self.map[0].len() as u32, self.map.len() as u32).into();
        let mut visible_size: MapSize = (
            visible_area_px.0 / tileset.tilesize().0,
            visible_area_px.1 / tileset.tilesize().1,
        )
            .into();
        visible_size.w = visible_size.w.min(size.w);
        visible_size.h = visible_size.h.min(size.h);
        let mut flags = vec![];
        let mut tiles = vec![];
        for row in &self.map {
            for tile_idx in row {
                let idx = *tile_idx as usize;
                tiles.push(idx);
                flags.push(flag_map[&idx]);
            }
        }
        Ok(Tilemap {
            tiles,
            flags,
            size,
            visible_size,
            offset: MapPosition::new(0, 0),
            images,
            tile_size: tileset.tilesize(),
            subtile_offset: (0, 0),
            default_start: self.data.start.into(),
            exits: self
                .data
                .exits
                .into_iter()
                .map(MapExit::from_file)
                .collect(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_RON: &str = r#"(
        name: "Desert Temple",
        tileset: "desert",
        flags: {
            1: "wall",
            2: "trap"
        },
        tiles: [
            (
                image: "sand",
                flags: 0
            ),
            (
                image: "temple_floor",
                flags: 0
            ),
            (
                image: "temple_wall",
                flags: 1
            ),
        ],
        map: [
            [2,2,2,2],
            [2,0,0,2],
            [2,0,0,2],
            [2,1,2,2],
        ],
        data: (
            start: (1,2),
            exits: [
                (1,3,"desert",4,5),
            ]
        )
    )"#;

    #[test]
    fn test_loading() {
        let tilemap_file: TilemapFile = ron::from_str(SAMPLE_RON).unwrap();
        assert_eq!(tilemap_file.name, String::from("Desert Temple"));
        assert_eq!(tilemap_file.tileset, String::from("desert"));
        assert_eq!(
            tilemap_file.data,
            TilemapDataDescriptor {
                start: (1, 2),
                exits: vec![(1, 3, "desert".to_string(), 4, 5)],
            }
        );
        assert_eq!(
            tilemap_file.flags,
            HashMap::from([(1, "wall".to_string()), (2, "trap".to_string())])
        );
        assert_eq!(
            tilemap_file.map,
            vec![
                vec![2, 2, 2, 2],
                vec![2, 0, 0, 2],
                vec![2, 0, 0, 2],
                vec![2, 1, 2, 2]
            ]
        );
        assert_eq!(
            tilemap_file.tiles,
            vec![
                TileDescriptor {
                    image: "sand".to_string(),
                    flags: 0,
                },
                TileDescriptor {
                    image: "temple_floor".to_string(),
                    flags: 0,
                },
                TileDescriptor {
                    image: "temple_wall".to_string(),
                    flags: 1,
                }
            ]
        );
    }

    #[test]
    fn init() {
        let tileset =
            Tileset::<&'static str>::new(vec![Rc::new("img")], vec!["img".to_string()], (16, 16));
        let idx_map = vec!["img".to_string()];
        let tilemap = Tilemap::new(
            vec![0; 400],
            vec![0; 400],
            MapSize::new(20, 20),
            idx_map,
            tileset,
            (300, 200),
            MapPosition::new(0, 0),
            vec![],
        )
        .unwrap();

        assert_eq!(tilemap.offset, MapPosition::new(0, 0));
        assert_eq!(tilemap.visible_size, MapSize::new(18, 12));
        assert_eq!(tilemap.onscreen_px_for_tile((0_u32, 0)), (0, 0));
        assert_eq!(tilemap.onscreen_px_for_tile((4_u32, 4)), (64, 64));
        assert_eq!(tilemap.first_visible_tile(), MapPosition::new(0, 0));
    }

    #[test]
    fn offset() {
        let tileset =
            Tileset::<&'static str>::new(vec![Rc::new("img")], vec!["img".to_string()], (16, 16));
        let idx_map = vec!["img".to_string()];
        let mut tilemap = Tilemap::new(
            vec![0; 400],
            vec![0; 400],
            MapSize::new(20, 20),
            idx_map,
            tileset,
            (300, 200),
            MapPosition::new(0, 0),
            vec![],
        )
        .unwrap();
        tilemap.center_on(MapPosition::new(10, 6));

        assert_eq!(tilemap.offset, MapPosition::new(1, 0));
        assert_eq!(tilemap.visible_size, MapSize::new(18, 12));
        assert_eq!(tilemap.onscreen_px_for_tile((0_u32, 0)), (-16, 0));
        assert_eq!(tilemap.onscreen_px_for_tile((4_u32, 4)), (48, 64));
        assert_eq!(tilemap.first_visible_tile(), MapPosition::new(1, 0));
    }

    #[allow(non_snake_case)]
    #[test]
    fn flags() {
        let tileset =
            Tileset::<&'static str>::new(vec![Rc::new("img")], vec!["img".to_string()], (16, 16));
        let idx_map = vec!["img".to_string()];
        let FLAG_WALL = 0b0001;
        let FLAG_TRAP = 0b0010;
        let mut flags = vec![0; 20];
        flags[1] = FLAG_WALL;
        flags[2] = FLAG_WALL;
        flags[5] = FLAG_TRAP;
        let mut tilemap = Tilemap::new(
            vec![0; 20],
            flags,
            MapSize::new(4, 5),
            idx_map,
            tileset,
            (300, 200),
            MapPosition::new(0, 0),
            vec![],
        )
        .unwrap();
        tilemap.set_flag((3_u32, 3), FLAG_WALL);
        tilemap.clear_flag((2_u32, 0), FLAG_WALL);
        assert!(tilemap.tile_has_flag((1_u32, 0), FLAG_WALL));
        assert!(!tilemap.tile_has_flag((1_u32, 0), FLAG_TRAP));
        assert_eq!(tilemap.flags_for_tile((1_u32, 0)), FLAG_WALL);
        assert_eq!(tilemap.flags_for_tile((3_u32, 0)), 0);
        assert_eq!(
            tilemap.all_tiles_with_flag(FLAG_WALL),
            vec![MapPosition::new(1, 0), MapPosition::new(3, 3)]
        );
    }
}

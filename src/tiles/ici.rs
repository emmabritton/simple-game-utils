use std::collections::HashMap;
use std::rc::Rc;

use ici_files::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IciTileset {
    pub name: String,
    tile_size: (u8, u8),
    palette: Vec<Color>,
    tiles: HashMap<String, Vec<u8>>,
}

impl IciTileset {
    pub fn new(
        name: String,
        tile_size: (u8, u8),
        palette: Vec<Color>,
        tiles: HashMap<String, Vec<u8>>,
    ) -> Self {
        Self {
            name,
            tile_size,
            palette,
            tiles,
        }
    }
}

impl IciTileset {
    pub fn add_image(&mut self, name: &str, image: &IndexedImage) {
        if image.get_palette() == self.palette {
            self.tiles
                .insert(name.to_string(), image.get_pixels().to_vec());
        } else {
            panic!("Image has invalid palette");
        }
    }

    pub fn tile_size(&self) -> (u8, u8) {
        self.tile_size
    }

    pub fn palette(&self) -> &Vec<Color> {
        &self.palette
    }

    pub fn tiles(&self) -> &HashMap<String, Vec<u8>> {
        &self.tiles
    }
}

impl IciTileset {
    pub fn into_tileset(self) -> Result<Tileset<IndexedImage>, String> {
        let mut images = vec![];
        let mut names = vec![];

        let palette = self.palette;

        for (name, pixels) in self.tiles {
            names.push(name);
            images.push(Rc::new(
                IndexedImage::new(self.tile_size.0, self.tile_size.1, palette.clone(), pixels)
                    .map_err(|e| e.to_string())?,
            ));
        }

        Ok(Tileset::new(
            images,
            names,
            (self.tile_size.0 as u32, self.tile_size.1 as u32),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_RON: &str = r#"(
        name: "sample",
        tile_size: (3,3),
        palette: [0,4278190335,16711935,65535],
        tiles: {
            "trans": [0,0,0,0,0,0,0,0,0],
            "red":   [1,1,1,1,1,1,1,1,1],
            "green": [2,2,2,2,2,2,2,2,2],
            "blue":  [3,3,3,3,3,3,3,3,3],
        }
    )
    "#;

    #[test]
    fn serde() {
        let tileset: IciTileset = ron::from_str(SAMPLE_RON).unwrap();
        assert_eq!(tileset.name, String::from("sample"));
        assert_eq!(tileset.palette, vec![TRANSPARENT, RED, GREEN, BLUE]);
        assert_eq!(tileset.tile_size, (3, 3));
        assert_eq!(
            tileset.tiles,
            HashMap::from([
                ("trans".to_string(), vec![0; 9]),
                ("red".to_string(), vec![1; 9]),
                ("green".to_string(), vec![2; 9]),
                ("blue".to_string(), vec![3; 9]),
            ])
        );
    }

    #[test]
    fn converting() {
        let tileset: Tileset<IndexedImage> = ron::from_str::<IciTileset>(SAMPLE_RON)
            .unwrap()
            .into_tileset()
            .unwrap();
        assert_eq!(tileset.tilesize(), (3, 3));
        assert_eq!(tileset.images().len(), 4);
        assert_eq!(tileset.find_by_name("red").unwrap().get_palette()[3], BLUE);
        assert_eq!(tileset.find_by_name("blue").unwrap().get_palette()[1], RED);
        assert!(tileset
            .find_by_name("trans")
            .unwrap()
            .get_pixels()
            .iter()
            .all(|&p| p == 0));
        assert!(tileset
            .find_by_name("green")
            .unwrap()
            .get_pixels()
            .iter()
            .all(|&p| p == 2));
    }
}

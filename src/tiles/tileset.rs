use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

///collection of named tiles
#[derive(Debug, Clone)]
pub struct Tileset<Image: Debug + Clone> {
    ///images
    images: Vec<Rc<Image>>,
    ///names
    names: Vec<String>,
    ///size of tiles in px
    size: (u32, u32),
}

impl<Image: Debug + Clone> Tileset<Image> {
    pub fn new(images: Vec<Rc<Image>>, names: Vec<String>, size: (u32, u32)) -> Self {
        Self {
            images,
            names,
            size,
        }
    }
}

impl<Image: Debug + Clone> Tileset<Image> {
    pub fn as_hashmap(&self) -> HashMap<String, Rc<Image>> {
        let mut map = HashMap::new();
        for (i, name) in self.names.iter().enumerate() {
            map.insert(name.clone(), self.images[i].clone());
        }
        map
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Image> {
        if let Some(i) = self.names.iter().position(|value| value == name) {
            return Some(&self.images[i]);
        }
        None
    }

    #[inline]
    pub fn names(&self) -> &[String] {
        &self.names
    }

    #[inline]
    pub fn images(&self) -> &Vec<Rc<Image>> {
        &self.images
    }

    #[inline]
    pub fn tilesize(&self) -> (u32, u32) {
        self.size
    }
}

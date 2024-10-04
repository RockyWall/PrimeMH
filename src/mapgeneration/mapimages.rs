use notan::prelude::RenderTexture;

use super::{mapgrid::MapGrid, pois::POI};

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct LevelImage {
    pub map_grid: Option<MapGrid>,
    pub map_edges: Option<MapGrid>,
    pub rendered_image: Option<u32>,
    pub pois: Vec<POI>,
    pub map_image: Option<RenderTexture>,
}

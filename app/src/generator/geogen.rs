use geojson::{FeatureCollection, GeoJson};
use geo::{Contains};
use geo_types::Coord;
use rand::Rng;

const COUNTRIES: &str = include_str!("../../priv/ne_110m_admin_0_countries.json");

pub struct GeoGen {
    geom: geo_types::Geometry<f32>,
    bbox: Vec<f64>
}

impl GeoGen {
    pub fn new(country: &str) -> GeoGen {
        let geojson: GeoJson = COUNTRIES.parse::<GeoJson>().unwrap();
        let collection = FeatureCollection::try_from(geojson).unwrap();

        let feature = collection
            .into_iter()
            .find(|f| f.properties.as_ref().unwrap().get("ADMIN").unwrap().as_str().unwrap() == country);

        if let None = feature {
            panic!("Can not find county {}", country)
        }

        let mut feature = feature.unwrap();
        let bbox = std::mem::take(&mut feature.bbox).unwrap();
        let geom: geo_types::Geometry<f32> = feature.try_into().unwrap();

        GeoGen {
            geom,
            bbox,
        }
    }

    pub fn random_point(&self) -> (f32, f32) {
        let mut rng = rand::thread_rng();

        loop {
            let x = rng.gen_range(self.bbox[0] as f32..self.bbox[2] as f32);
            let y = rng.gen_range(self.bbox[1] as f32..self.bbox[3] as f32);
            if self.geom.contains(&Coord {x, y}) {
               return (x, y)
            }
        }
    }
}
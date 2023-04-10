use crate::db::DbPool;
use geojson::{FeatureCollection, GeoJson};
use serde_json::Value;
use std::convert::TryFrom;

pub fn import_geojson(from: String, db: DbPool) -> anyhow::Result<()> {
    let conn = db.get()?;

    let mut s1 = conn.prepare(
        r#"
            INSERT into geo_feature (type, name, bbox, geometry)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT DO UPDATE SET bbox=?3, geometry=?4
            RETURNING id"#,
    )?;

    let mut s2 = conn.prepare(
        r#"
            INSERT into geo_feature_property (geo_feature_id, key, value)
            VALUES (?1, ?2, ?3)
            ON CONFLICT DO UPDATE SET value=?3"#,
    )?;

    let j: GeoJson = std::fs::read_to_string(from)?.parse()?;
    let f = FeatureCollection::try_from(j)?;

    for f in &f.features {
        let p = f.properties.as_ref().unwrap();

        let feature_type = p["TYPE"].as_str().unwrap();
        let feature_name = p["NAME"].as_str().unwrap();
        let bbox = serde_json::to_string(f.bbox.as_ref().unwrap())?;
        let geometry = serde_json::to_string(f.geometry.as_ref().unwrap())?;

        let geo_feature_id: usize = s1
            .query_row((feature_type, feature_name, &bbox, &geometry), |row| {
                row.get(0)
            })?;

        for (key, v) in p {
            if !INTERESTING_KEYS.contains(&key.as_str()) {
                continue;
            }

            let s = match v {
                Value::Number(n) => Some(n.to_string()),
                Value::String(s) => Some(s.clone()),
                _ => None,
            };

            if let Some(s) = s {
                s2.execute((geo_feature_id, key, s))?;
            }
        }
    }

    Ok(())
}

const INTERESTING_KEYS: &[&str] = &[
    "LABEL_X",
    "LABEL_Y",
    "NAME_LONG",
    "CONTINENT",
    "NAME_AR",
    "NAME_BN",
    "NAME_DE",
    "NAME_EN",
    "NAME_ES",
    "NAME_FA",
    "NAME_FR",
    "NAME_EL",
    "NAME_HE",
    "NAME_HI",
    "NAME_HU",
    "NAME_ID",
    "NAME_IT",
    "NAME_JA",
    "NAME_KO",
    "NAME_NL",
    "NAME_PL",
    "NAME_PT",
    "NAME_RU",
    "NAME_SV",
    "NAME_TR",
    "NAME_UK",
    "NAME_UR",
    "NAME_VI",
    "NAME_ZH",
    "NAME_ZHT",
];

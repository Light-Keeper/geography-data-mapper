use std::fs::File;
use std::io;
use std::io::BufRead;
use anyhow::bail;
use crate::db::DbPool;

pub fn import_geonames(from: String, name: String, db: DbPool) -> anyhow::Result<()> {
    let file = File::open(from)?;

    let cities = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.split('\t')
            .map(String::from)
            .zip(GEONAMES_FIELDS.iter().map(|s| *s))
            .collect::<Vec<(String, &str)>>()
        );

    let name_index = GEONAMES_FIELDS.iter().position(|&s| s == "name").unwrap();
    let lat_index = GEONAMES_FIELDS.iter().position(|&s| s == "latitude").unwrap();
    let lng_index = GEONAMES_FIELDS.iter().position(|&s| s == "longitude").unwrap();
    let population_index = GEONAMES_FIELDS.iter().position(|&s| s == "population").unwrap();

    let mut conn = db.get()?;
    let tx = conn.transaction()?;

    //language=SQLite
    let dataset_id: usize = tx.query_row(
        r#"INSERT INTO datasets (name) VALUES (?1) RETURNING id"#,
        [&name],
        |r| r.get(0)
    )?;

    for city in cities {
        let name = &city[name_index].0;
        let lat = &city[lat_index].0;
        let lng = &city[lng_index].0;
        let population = &city[population_index].0;

        let lat = lat.parse::<f32>()?;
        let lng = lng.parse::<f32>()?;
        let population = population.parse::<i64>()?;

        //language=SQLite
        let datapoint_id: usize = tx.query_row(r#"
            INSERT INTO datapoints (dataset_id, lng, lat)
            VALUES (?1, ?2, ?3)
            RETURNING id
            "#, (dataset_id, lng, lat), |r| r.get(0))?;

        //language=SQLite
        tx.execute(r#"
            INSERT INTO attributes (dataset_id, datapoint_id, name, value)
            VALUES (?1, ?2, 'Name', ?3)
            "#, (dataset_id, datapoint_id, name))?;

        //language=SQLite
        tx.execute(r#"
            INSERT INTO attributes (dataset_id, datapoint_id, name, value)
            VALUES (?1, ?2, 'Population', ?3)
            "#, (dataset_id, datapoint_id, population))?;
    }

    tx.commit()?;
    Ok(())
}

const GEONAMES_FIELDS: &[&str] = &[
    // integer id of record in geonames database
    "geonameid",
    // name of geographical point (utf8) varchar(200)
    "name",
    // name of geographical point in plain ascii characters, varchar(200)
    "asciiname",
    // alternatenames, comma separated, ascii names automatically transliterated,
    // convenience attribute from alternatename table, varchar(10000)
    "alternatenames",
    // latitude in decimal degrees (wgs84)
    "latitude",
    // longitude in decimal degrees (wgs84)
    "longitude",
    // see http://www.geonames.org/export/codes.html, char(1)
    "feature class",
    // see http://www.geonames.org/export/codes.html, varchar(10)
    "feature code",
    // ISO-3166 2-letter country code, 2 characters
    "country code",
    // alternate country codes, comma separated, ISO-3166 2-letter country code, 200 characters
    "cc2",
    // fipscode (subject to change to iso code), see exceptions below, see file admin1Codes.txt for display names of this code; varchar(20)
    "admin1 code",
    // code for the second administrative division, a county in the US, see file admin2Codes.txt; varchar(80)
    "admin2 code",
    // code for third level administrative division, varchar(20)
    "admin3 code",
    // code for fourth level administrative division, varchar(20)
    "admin4 code",
    // bigint (8 byte int)
    "population",
    // in meters, integer
    "elevation",
    // digital elevation model, srtm3 or gtopo30, average elevation of 3''x3'' (ca 90mx90m) or 30''x30'' (ca 900mx900m) area in meters, integer. srtm processed by cgiar/ciat.
    "dem",
    // the iana timezone id (see file timeZone.txt) varchar(40)
    "timezone",
    // date of last modification in yyyy-MM-dd format
    "modification date",
];

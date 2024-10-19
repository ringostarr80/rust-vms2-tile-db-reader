use crate::data_type::DataType;
use byteorder::{LittleEndian, WriteBytesExt};
use rusqlite::types::ValueRef;
use rusqlite::{params, Connection, OpenFlags, Result, ToSql};
use std::io::{Cursor, Write};
use std::path::Path;

pub trait Source {
    fn get_raw_data(
        &self,
        x: u32,
        y: u32,
        z: u8,
        key: String,
        value: Option<String>,
        data_type: Option<DataType>,
    ) -> Result<Vec<u8>>;
}

pub struct SQLite {
    pub conn: Connection,
}

impl SQLite {
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        Ok(Self { conn })
    }

    fn get_detail_zoom(z: u8, value: &String, data_type: &DataType) -> u8 {
        let mut detail_zooms = vec![0, 0, 2, 2, 4, 4, 6, 6, 8, 8, 10, 10, 12, 12, 14];

        match value.as_str() {
            "terrain" | "depth" => {
                detail_zooms = vec![0, 0, 2, 2, 4, 4, 6, 6, 8, 8, 10, 10, 12, 12, 12];
            }
            "bathymetry" | "blue_marble" | "elevation" => {
                detail_zooms = vec![0, 0, 2, 2, 4, 4, 6, 6, 8, 8, 10, 10, 10, 10, 10];
            }
            _ => {}
        }

        let clamped_z = z.clamp(0, 14);
        let mut detail_zoom = detail_zooms[clamped_z as usize];

        if let DataType::Points = data_type {
            detail_zoom = 14;
        }

        detail_zoom
    }

    fn query_data(
        &self,
        query: &str,
        query_params: &[&dyn ToSql],
    ) -> Result<Vec<(u32, u32, u8, Vec<u8>)>> {
        let mut stmt = self.conn.prepare(query)?;
        let mut rows = stmt.query(query_params)?;

        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            let result_data = row.get_ref(3);
            let mut data = Vec::new();
            if let Ok(unwrapped_result_data) = result_data {
                data = match unwrapped_result_data {
                    ValueRef::Text(text) => text.to_vec(),
                    ValueRef::Blob(blob) => blob.to_vec(),
                    _ => Vec::new(),
                }
            }

            result.push((
                row.get_unwrap(0),
                row.get_unwrap(1),
                row.get_unwrap(2),
                data,
            ));
        }

        Ok(result)
    }
}

impl Source for SQLite {
    /// Get raw data from the SQLite database.
    ///
    /// # Example
    /// ```
    /// use std::path::Path;
    /// use vms2_tile_db_reader::data_type::DataType;
    /// use vms2_tile_db_reader::sources::{SQLite, Source};
    ///
    /// let tile_db = SQLite::new(Path::new("./tests/data/braunschweig.sqlite")).unwrap();
    /// let tile_data = tile_db
    ///     .get_raw_data(
    ///         34686,
    ///         21566,
    ///         16,
    ///         String::from("building"),
    ///         Some(String::from("*")),
    ///         Some(DataType::Polygons),
    ///     ).unwrap();
    ///
    /// assert!(tile_data.len() >= 4);
    /// ```
    fn get_raw_data(
        &self,
        x: u32,
        y: u32,
        z: u8,
        mut key: String,
        mut value: Option<String>,
        mut data_type: Option<DataType>,
    ) -> Result<Vec<u8>> {
        match key.as_str() {
            "land" | "terrain" | "blue_marble" | "elevation" | "bathymetry" | "depth" => {
                value = Some(key);
                key = String::from("locr");
                data_type = Some(DataType::Polygons);
            }
            _ => {}
        }

        let data_type = data_type.unwrap_or(DataType::Polygons);
        let value = value.unwrap_or(String::new());

        let detail_zoom = Self::get_detail_zoom(z, &value, &data_type);

        let mut data_buffer = Cursor::new(Vec::new());
        let data_type_int = data_type as u32;
        let max_tile_zoom = 16_u8;
        let mut number_of_tiles = 0_u32;
        let mut tile_weight = 0_u64;

        let single_tile_query = "SELECT x, y, z, data \
            FROM tiles \
            WHERE detail_zoom = ?1 \
                AND object_type = ?2 \
                AND osm_key = ?3 \
                AND osm_value = ?4 \
                AND x = ?5 \
                AND y = ?6 \
                AND z = ?7";
        let multi_tile_query = "SELECT x, y, z, data \
            FROM tiles \
            WHERE detail_zoom = ?1 \
                AND object_type = ?2 \
                AND osm_key = ?3 \
                AND osm_value = ?4 \
                AND x >= ?5 \
                AND x < ?6 \
                AND y >= ?7 \
                AND y < ?8 \
                AND z = ?9";

        for query_z in 0..=max_tile_zoom {
            let queried_data: Vec<(u32, u32, u8, Vec<u8>)>;

            if query_z <= z {
                let query_x = x >> (z - query_z);
                let query_y = y >> (z - query_z);

                let query_params = params![
                    detail_zoom,
                    data_type_int,
                    key,
                    &value,
                    query_x,
                    query_y,
                    query_z
                ];
                queried_data = self.query_data(single_tile_query, query_params)?;
            } else {
                let query_left_x = x << (query_z - z);
                let query_top_y = y << (query_z - z);

                let query_right_x = query_left_x + (1 << (query_z - z));
                let query_bottom_y = query_top_y + (1 << (query_z - z));

                let query_params = params![
                    detail_zoom,
                    data_type_int,
                    key,
                    &value,
                    query_left_x,
                    query_right_x,
                    query_top_y,
                    query_bottom_y,
                    query_z
                ];
                queried_data = self.query_data(multi_tile_query, query_params)?;
            }

            if queried_data.len() > 0 {
                number_of_tiles += queried_data.len() as u32;
                tile_weight += 4_u64.pow((max_tile_zoom - query_z) as u32);

                for (tile_x, tile_y, tile_z, tile_data) in queried_data {
                    let _ = data_buffer.write_u32::<LittleEndian>(tile_x);
                    let _ = data_buffer.write_u32::<LittleEndian>(tile_y);
                    let _ = data_buffer.write_u32::<LittleEndian>(tile_z as u32);
                    let _ = data_buffer.write_u32::<LittleEndian>(detail_zoom as u32);
                    let _ = data_buffer.write_u32::<LittleEndian>(tile_data.len() as u32);

                    if tile_data.len() > 0 {
                        let _ = data_buffer.write_all(&tile_data);
                    }
                }
            }

            println!("max_tile_zoom: {}, z: {}, tile_weight: {}", max_tile_zoom, query_z, tile_weight);
            if tile_weight >= 4_u64.pow((max_tile_zoom - z) as u32) {
                break;
            }
        }

        let mut buffer = Cursor::new(Vec::new());
        let _ = buffer.write_u32::<LittleEndian>(number_of_tiles);
        let _ = buffer.write_all(data_buffer.get_ref());

        Ok(buffer.into_inner())
    }
}

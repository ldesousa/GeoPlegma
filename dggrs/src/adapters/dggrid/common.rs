// Copyright 2025 iontributors to the GeoPlegma project.
// Originally authored by Michael Jendryke (GeoInsight GmbH, michael.jendryke@geoinsight.ai)
//
// Licenced under the Apache Licence, Version 2.0 <LICENCE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENCE-MIT or http://opensource.org/licenses/MIT>, at your
// discretion. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod dggrid {
    use rand::distributions::{Alphanumeric, DistString};
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;

    pub fn setup(workdir: &PathBuf) -> (PathBuf, PathBuf, PathBuf, PathBuf, PathBuf, PathBuf) {
        let code = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let meta_path = workdir.join(&code).with_extension("meta"); // metafile
        let aigen_path = workdir.join(&code).with_extension("gen"); // AIGEN
        let children_path = workdir.join(&code).with_extension("chd"); // Children
        let neighbor_path = workdir.join(&code).with_extension("nbr"); // Neightbors
        let bbox_path = workdir.join(&code).with_extension("bbox"); // BBox
        let input_path = workdir.join(&code).with_extension("txt"); // Input file for e.g. points
        (
            meta_path,
            aigen_path,
            children_path,
            neighbor_path,
            bbox_path,
            input_path,
        )
    }
    pub fn execute(dggrid_path: &PathBuf, meta_path: &PathBuf) {
        let _ = Command::new(&dggrid_path).arg(&meta_path).output(); // FIX: Better handling of output and raise DggridError::DggridExecutionFailed
    }
}

pub mod write {
    use crate::models::common::RefinementLevel;
    use geo::Rect;
    use std::fs;
    use std::io::{self, Write};
    use std::path::PathBuf;
    use tracing::debug;

    pub const DENSIFICATION: u8 = 50; // DGGRID option

    pub fn metafile(
        metafile: &PathBuf,
        refinement_level: &RefinementLevel,
        cell_output_file_name: &PathBuf,
        children_output_file_name: &PathBuf,
        neighbor_output_file_name: &PathBuf,
        densify: bool,
    ) -> io::Result<()> {
        debug!("Writing to {:?}", metafile);
        let mut file = fs::File::create(metafile)?;
        writeln!(file, "longitude_wrap_mode UNWRAP_EAST")?;
        writeln!(file, "cell_output_type AIGEN")?;
        writeln!(file, "unwrap_points FALSE")?;
        writeln!(file, "output_cell_label_type OUTPUT_ADDRESS_TYPE")?;
        writeln!(file, "precision 7")?;
        writeln!(file, "dggs_res_spec {}", refinement_level.get())?;
        writeln!(file, "z3_invalid_digit 3")?;

        writeln!(
            file,
            "cell_output_file_name {}",
            cell_output_file_name.to_string_lossy().into_owned()
        )?;

        writeln!(file, "neighbor_output_type TEXT")?;
        writeln!(
            file,
            "neighbor_output_file_name {}",
            neighbor_output_file_name.to_string_lossy().into_owned()
        )?;
        writeln!(file, "children_output_type TEXT")?;
        writeln!(
            file,
            "children_output_file_name {}",
            children_output_file_name.to_string_lossy().into_owned()
        )?;

        if densify == true {
            writeln!(file, "densification {}", DENSIFICATION)?;
        }

        Ok(())
    }

    pub fn bbox(bbox: &Rect<f64>, bboxfile: &PathBuf) -> io::Result<()> {
        let min = bbox.min();
        let max = bbox.max();

        let (minx, miny) = (min.x, min.y);
        let (maxx, maxy) = (max.x, max.y);

        // define the 5 vertices (closing the polygon)
        let vertices = vec![
            (minx, miny), // lower-left
            (maxx, miny), // lower-right
            (maxx, maxy), // upper-right
            (minx, maxy), // upper-left
            (minx, miny), // close
        ];
        let mut file = fs::File::create(bboxfile)?;

        // First line: ID and center of the bbox (NOT part of the ring)
        let center_x = (minx + maxx) / 2.0;
        let center_y = (miny + maxy) / 2.0;
        writeln!(file, "1 {:.7} {:.7}", center_x, center_y)?;

        for (x, y) in &vertices {
            writeln!(file, "{:.7} {:.7}", x, y)?;
        }

        writeln!(file, "END")?;
        writeln!(file, "END")?;

        Ok(())
    }

    pub fn file(file: PathBuf) {
        if let Ok(lines) = super::read::lines(file) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines.flatten() {
                debug!("{}", line);
            }
        }
    }
}
pub mod read {
    use crate::error::dggrid::DggridError;
    use crate::error::port::GeoPlegmaError;
    use crate::models::common::{Zone, ZoneId, Zones};
    use core::f64;
    use geo::{LineString, Point, Polygon};
    use std::fs;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::{Path, PathBuf};

    #[derive(Debug)]
    pub struct IdArray {
        pub id: Option<String>,
        pub arr: Option<Vec<String>>,
    }

    pub fn dggrid_parse(
        aigen_path: &PathBuf,
        children_path: &PathBuf,
        neighbor_path: &PathBuf,
    ) -> Result<Zones, GeoPlegmaError> {
        let aigen_data = file(&aigen_path)?;
        let mut result = aigen(&aigen_data)?;
        let children_data = file(&children_path)?;
        let children = children(&children_data)?;
        assign_field(&mut result, children, "children");

        let neighbor_data = file(&neighbor_path)?;
        let neighbors = neighbors(&neighbor_data)?;
        assign_field(&mut result, neighbors, "neighbors");
        Ok(result)
    }

    pub fn aigen(data: &String) -> Result<Zones, GeoPlegmaError> {
        let mut zone_id = ZoneId::new_str(&"0")?; // FIX: Use ZoneId::new_hex
        let mut zones = Zones { zones: Vec::new() };

        let mut raw_coords: Vec<(f64, f64)> = vec![];
        let mut region: Polygon;
        let mut center = Point::new(0.0, 0.0);
        let mut v_count = 0u32;

        // loop over the entire AIGEN file
        for line in data.lines() {
            // println!("{:?}", line);
            let line_parts: Vec<&str> = line.split_whitespace().collect();
            // The first line of each hexagon is always 3 strings, the first is the ID and the
            // second two are the center point

            if line_parts.len() == 3 {
                zone_id = ZoneId::new_hex(&line_parts[0])?;
                center = Point::new(
                    line_parts[1]
                        .parse::<f64>()
                        .expect("cannot parse floating point number"),
                    line_parts[2]
                        .parse::<f64>()
                        .expect("cannot parse floating point number"),
                );
            // these are coordinate pairs for the region
            } else if line_parts.len() == 2 {
                v_count += 1;
                raw_coords.push((
                    line_parts[0]
                        .parse::<f64>()
                        .expect("cannot parse floating point number"),
                    line_parts[1]
                        .parse::<f64>()
                        .expect("cannot parse floating point number"),
                ))
            // if it just 1 part AND it is END AND if the vertex count is larger than 1
            } else if line_parts.len() == 1 && line_parts[0] == "END" && v_count > 1 {
                region = Polygon::new(LineString::from(raw_coords.clone()), vec![]);

                let zone = Zone {
                    id: zone_id.clone(),
                    region: Some(region),
                    center: Some(center),
                    vertex_count: Some(v_count - 1),
                    children: Some(children),
                    neighbors: Some(neighbors),
                    area_sqm: Some(area_sqm),
                };
                zones.zones.push(zone);

                // reset
                raw_coords.clear();
                v_count = 0;
            }
        }
        Ok(zones)
    }

    pub fn children(data: &String) -> Result<Vec<IdArray>, DggridError> {
        Ok(data
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.is_empty() {
                    return None;
                }

                let id = Some(format!("{}", parts[0]));
                let arr = parts.iter().skip(1).map(|s| format!("{}", s)).collect();

                Some(IdArray { id, arr: Some(arr) })
            })
            .collect())
    }

    pub fn neighbors(data: &String) -> Result<Vec<IdArray>, DggridError> {
        Ok(data
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.is_empty() {
                    return None;
                }

                let id = Some(format!("{}", parts[0]));
                let arr = parts.iter().skip(1).map(|s| format!("{}", s)).collect();

                Some(IdArray { id, arr: Some(arr) })
            })
            .collect())
    }

    pub fn assign_field(zones: &mut Zones, data: Vec<IdArray>, field: &str) {
        for item in data {
            if let Some(ref id_str) = item.id {
                let target_id = ZoneId::StrId(id_str.clone());
                if let Some(cell) = zones.zones.iter_mut().find(|c| c.id == target_id) {
                    match field {
                        "children" => {
                            cell.children = item
                                .arr
                                .clone()
                                .map(|v| v.into_iter().map(ZoneId::StrId).collect())
                        }
                        "neighbors" => {
                            cell.neighbors = item
                                .arr
                                .clone()
                                .map(|v| v.into_iter().map(ZoneId::StrId).collect())
                        }
                        _ => panic!("Unknown field: {}", field),
                    }
                }
            }
        }
    }
    // Read aigen file produced by DGGRID
    // Todo: this is inefficient, use the read_lines function as in print_file
    // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    pub fn file(path: &Path) -> Result<String, DggridError> {
        Ok(fs::read_to_string(path).map_err(|e| DggridError::FileRead {
            path: path.display().to_string(),
            source: e,
        })?)
    }

    pub fn lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

use std::fs;
use std::path::PathBuf;
pub fn cleanup(
    meta_path: &PathBuf,
    aigen_path: &PathBuf,
    children_path: &PathBuf,
    neighbor_path: &PathBuf,
    bbox_path: &PathBuf,
) {
    let _ = fs::remove_file(meta_path);
    let _ = fs::remove_file(aigen_path);
    let _ = fs::remove_file(children_path);
    let _ = fs::remove_file(neighbor_path);
    let _ = fs::remove_file(bbox_path);
}

// Copyright 2025 contributors to the GeoPlegma project.
// Originally authored by Michael Jendryke (GeoInsight GmbH, michael.jendryke@geoinsight.ai)
//
// Licenced under the Apache Licence, Version 2.0 <LICENCE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENCE-MIT or http://opensource.org/licenses/MIT>, at your
// discretion. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::adapters::dggrid::common;
use crate::adapters::dggrid::dggrid::DggridAdapter;
use crate::error::port::GeoPlegmaError;
use crate::models::common::{RefinementLevel, RelativeDepth, Zones};
use crate::ports::dggrs::DggrsPort;
use core::f64;
use geo::geometry::Point;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::PathBuf;
use tracing::debug;
pub const CLIP_CELL_DENSIFICATION: u8 = 50; // DGGRID option
use geo::Rect;

pub struct Igeo7Impl {
    pub adapter: DggridAdapter,
}

impl Igeo7Impl {
    // Optional: allow custom paths too
    pub fn new(executable: PathBuf, workdir: PathBuf) -> Self {
        Self {
            adapter: DggridAdapter::new(executable, workdir),
        }
    }
}

impl Default for Igeo7Impl {
    fn default() -> Self {
        Self {
            adapter: DggridAdapter::default(),
        }
    }
}

impl DggrsPort for Igeo7Impl {
    fn zones_from_bbox(
        &self,
        refinement_level: RefinementLevel,
        densify: bool,
        bbox: Option<Rect<f64>>,
    ) -> Result<Zones, GeoPlegmaError> {
        let (meta_path, aigen_path, children_path, neighbor_path, bbox_path, _input_path) =
            common::dggrid_setup(&self.adapter.workdir);

        let _ = common::dggrid_metafile(
            &meta_path,
            &u8::try_from(refinement_level)?,
            &aigen_path.with_extension(""),
            &children_path.with_extension(""),
            &neighbor_path.with_extension(""),
            densify,
        );

        let _ = igeo7_metafile(&meta_path);

        if let Some(bbox) = &bbox {
            let _ = common::bbox_to_aigen(bbox, &bbox_path);

            // Append to metafile
            let mut meta_file = OpenOptions::new()
                .append(true)
                .write(true)
                .open(&meta_path)
                .expect("cannot open file");

            let _ = writeln!(meta_file, "clip_subset_type AIGEN");
            let _ = writeln!(
                meta_file,
                "clip_region_files {}",
                &bbox_path.to_string_lossy()
            );
        }

        common::print_file(meta_path.clone());
        common::dggrid_execute(&self.adapter.executable, &meta_path);
        let result = common::dggrid_parse(&aigen_path, &children_path, &neighbor_path)?;
        common::dggrid_cleanup(
            &meta_path,
            &aigen_path,
            &children_path,
            &neighbor_path,
            &bbox_path,
        );
        Ok(result)
    }

    fn zone_from_point(
        &self,
        refinement_level: RefinementLevel,
        point: Point,
        densify: bool,
    ) -> Result<Zones, GeoPlegmaError> {
        let (meta_path, aigen_path, children_path, neighbor_path, bbox_path, input_path) =
            common::dggrid_setup(&self.adapter.workdir);

        let _ = common::dggrid_metafile(
            &meta_path,
            &u8::try_from(refinement_level)?,
            &aigen_path.with_extension(""),
            &children_path.with_extension(""),
            &neighbor_path.with_extension(""),
            densify,
        );

        let _ = igeo7_metafile(&meta_path);

        // Append to metafile
        let mut meta_file = OpenOptions::new()
            .append(true)
            .write(true)
            .open(&meta_path)
            .expect("cannot open file");

        let _ = writeln!(meta_file, "dggrid_operation TRANSFORM_POINTS");
        let _ = writeln!(meta_file, "input_address_type GEO");
        let _ = writeln!(
            meta_file,
            "input_file_name {}",
            &input_path.to_string_lossy()
        );

        // File with one point
        let mut input_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&input_path)
            .expect("cannot open file");
        let _ = writeln!(input_file, "{} {}", point.y(), point.x())
            .expect("Cannot create point input file");

        common::print_file(meta_path.clone());
        common::dggrid_execute(&self.adapter.executable, &meta_path);
        let result = common::dggrid_parse(&aigen_path, &children_path, &neighbor_path)?;
        common::dggrid_cleanup(
            &meta_path,
            &aigen_path,
            &children_path,
            &neighbor_path,
            &bbox_path,
        );
        let _ = fs::remove_file(&input_path);
        Ok(result)
    }
    fn zones_from_parent(
        &self,
        relative_depth: RelativeDepth,
        parent_zone_id: String, // ToDo: needs validation function
        densify: bool,
    ) -> Result<Zones, GeoPlegmaError> {
        let (meta_path, aigen_path, children_path, neighbor_path, bbox_path, _input_path) =
            common::dggrid_setup(&self.adapter.workdir);

        let _ = common::dggrid_metafile(
            &meta_path,
            &u8::try_from(relative_depth)?,
            &aigen_path.with_extension(""),
            &children_path.with_extension(""),
            &neighbor_path.with_extension(""),
            densify,
        );

        let _ = igeo7_metafile(&meta_path);

        // Append to metafile format
        let mut meta_file = OpenOptions::new()
            .append(true)
            .write(true)
            .open(&meta_path)
            .expect("cannot open file");

        let parent_zone_res = get_refinement_level_from_z7_zone_id(&parent_zone_id).unwrap();

        let _ = writeln!(meta_file, "clip_subset_type COARSE_CELLS");
        let _ = writeln!(meta_file, "clip_cell_res {:?}", parent_zone_res);
        let _ = writeln!(
            meta_file,
            "clip_cell_densification {}",
            CLIP_CELL_DENSIFICATION
        );
        let _ = writeln!(meta_file, "clip_cell_addresses \"{}\"", parent_zone_id);
        let _ = writeln!(meta_file, "input_address_type Z7");
        common::print_file(meta_path.clone());
        common::dggrid_execute(&self.adapter.executable, &meta_path);
        let result = common::dggrid_parse(&aigen_path, &children_path, &neighbor_path)?;
        common::dggrid_cleanup(
            &meta_path,
            &aigen_path,
            &children_path,
            &neighbor_path,
            &bbox_path,
        );
        Ok(result)
    }
    fn zone_from_id(
        &self,
        zone_id: String, // ToDo: needs validation function
        densify: bool,
    ) -> Result<Zones, GeoPlegmaError> {
        let (meta_path, aigen_path, children_path, neighbor_path, bbox_path, input_path) =
            common::dggrid_setup(&self.adapter.workdir);

        let refinement_level = get_refinement_level_from_z7_zone_id(&zone_id).unwrap();
        let _ = common::dggrid_metafile(
            &meta_path,
            &refinement_level,
            &aigen_path.with_extension(""),
            &children_path.with_extension(""),
            &neighbor_path.with_extension(""),
            densify,
        );

        let _ = igeo7_metafile(&meta_path);

        // Append to metafile format
        let mut meta_file = OpenOptions::new()
            .append(true)
            .write(true)
            .open(&meta_path)
            .expect("cannot open file");

        let zone = &zone_id[2..]; // strip first two characters. ToDo: only if we attached the res to the front

        let _ = writeln!(
            meta_file,
            "input_file_name {}",
            &input_path.to_string_lossy()
        );

        // File with one point
        let mut input_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&input_path)
            .expect("cannot open file");
        let _ = writeln!(input_file, "{}", zone).expect("Cannot create zone id input file");

        let _ = writeln!(meta_file, "dggrid_operation TRANSFORM_POINTS");
        let _ = writeln!(meta_file, "input_address_type Z7");
        common::print_file(meta_path.clone());
        common::dggrid_execute(&self.adapter.executable, &meta_path);
        let result = common::dggrid_parse(&aigen_path, &children_path, &neighbor_path)?;
        common::dggrid_cleanup(
            &meta_path,
            &aigen_path,
            &children_path,
            &neighbor_path,
            &bbox_path,
        );
        Ok(result)
    }

    fn min_refinement_level(&self) -> Result<RefinementLevel, GeoPlegmaError> {
        Ok(RefinementLevel::new(0)?)
    }

    fn max_refinement_level(&self) -> Result<RefinementLevel, GeoPlegmaError> {
        Ok(RefinementLevel::new(18)?)
    }

    fn default_refinement_level(&self) -> Result<RefinementLevel, GeoPlegmaError> {
        Ok(RefinementLevel::new(2)?)
    }

    fn max_relative_depth(&self) -> Result<RelativeDepth, GeoPlegmaError> {
        Ok(RelativeDepth::new(3)?)
    }

    fn default_relative_depth(&self) -> Result<RelativeDepth, GeoPlegmaError> {
        Ok(RelativeDepth::new(1)?)
    }
}

pub fn igeo7_metafile(meta_path: &PathBuf) -> io::Result<()> {
    debug!("Writing to {:?}", meta_path);
    // Append to metafile format
    let mut meta_file = OpenOptions::new()
        .append(true)
        .write(true)
        .open(&meta_path)
        .expect("cannot open file");
    writeln!(meta_file, "dggs_type {}", "IGEO7")?;
    writeln!(meta_file, "dggs_aperture 7")?;
    writeln!(meta_file, "output_address_type Z7")?;

    Ok(())
}

/// Extract resolution from IGEO7 ID (Z7)
pub fn get_refinement_level_from_z7_zone_id(dggrid_z7_id: &str) -> Result<u8, String> {
    // Accept optional 0x prefix
    let dggrid_z7_id = dggrid_z7_id
        .strip_prefix("0x")
        .or_else(|| dggrid_z7_id.strip_prefix("0X"))
        .unwrap_or(dggrid_z7_id);

    let v = u64::from_str_radix(dggrid_z7_id, 16)
        .map_err(|_| "Invalid hex for Z7 INT64".to_string())?;

    // Bits 63..60 = base cell
    let base_cell = ((v >> 60) & 0xF) as u8;
    if base_cell > 11 {
        return Err(format!("Invalid base cell {} (>11)", base_cell));
    }

    // Bits 59..0 = 20 × 3-bit digits. d1 at bits 59..57, ..., d20 at 2..0.
    let mut resolution: u8 = 20; // default if no padding 7 is found
    for i in 0..20 {
        let shift = 60 - 3 * (i + 1); // i=0 => 57 .. i=19 => 0
        let digit = ((v >> shift) & 0b111) as u8;

        if digit > 7 {
            return Err(format!("Invalid Z7 digit {} at position {}", digit, i + 1));
        }
        if digit == 7 {
            resolution = i as u8;
            break;
        }
    }

    Ok(resolution)
}

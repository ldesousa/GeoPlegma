// Copyright 2025 contributors to the GeoPlegmata project.
// Originally authored by Michael Jendryke (GeoInsight GmbH, michael.jendryke@geoinsight.ai)
//
// Licenced under the Apache Licence, Version 2.0 <LICENCE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENCE-MIT or http://opensource.org/licenses/MIT>, at your
// discretion. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::adapters::dggal::common::{bbox_to_geoextent, ids_to_zones, to_geo_point};
use crate::constants::whole_earth_bbox;
use crate::error::dggal::DggalError;
use crate::error::port::GeoPlegmaError;
use crate::models::common::{RefinementLevel, RelativeDepth, Zones};
use crate::ports::dggrs::DggrsPort;
use dggal::DGGRS;
use dggal_rust::dggal;
use geo::{Point, Rect};

// fn get_dggrs(grid_name: &str) -> Result<DGGRS, DggalError> {
//     let args: Vec<String> = env::args().collect();
//     let my_app = Application::new(&args);
//     let dggal = DGGAL::new(&my_app);
//     DGGRS::new(&dggal, grid_name).map_err(|_| DggalError::UnknownGrid {
//         grid_name: grid_name.to_string(),
//     })
// }

pub struct DggalImpl {
    pub grid_name: String,
}

impl DggalImpl {
    pub fn new(grid_name: &str) -> Self {
        Self {
            grid_name: grid_name.to_string(),
        }
    }
}

use crate::adapters::dggal::context::GLOBAL_DGGAL;

fn get_dggrs(grid_name: &str) -> Result<DGGRS, DggalError> {
    let dggal = GLOBAL_DGGAL.lock().map_err(|_| DggalError::LockFailure)?;
    DGGRS::new(&*dggal, grid_name).map_err(|_| DggalError::UnknownGrid {
        grid_name: grid_name.to_string(),
    })
}

impl DggrsPort for DggalImpl {
    fn zones_from_bbox(
        &self,
        depth: RefinementLevel,
        densify: bool,
        bbox: Option<Rect<f64>>,
    ) -> Result<Zones, GeoPlegmaError> {
        if depth > self.max_refinement_level()? {
            return Err(GeoPlegmaError::DepthLimitReached {
                grid_name: self.grid_name.clone(),
                requested: depth,
                maximum: self.max_refinement_level()?,
            });
        };

        let geo_extent = if let Some(b) = bbox {
            bbox_to_geoextent(&b)
        } else {
            bbox_to_geoextent(&whole_earth_bbox())
        };

        let dggrs = get_dggrs(&self.grid_name)?;

        let zones = dggrs.listZones(i32::from(depth), &geo_extent);
        Ok(ids_to_zones(dggrs, zones)?)
    }
    fn zone_from_point(
        &self,
        depth: RefinementLevel,
        point: Point,
        densify: bool,
    ) -> Result<Zones, GeoPlegmaError> {
        let dggrs = get_dggrs(&self.grid_name)?;
        let zone = dggrs.getZoneFromWGS84Centroid(i32::from(depth), &to_geo_point(point));
        let zones = vec![zone];
        Ok(ids_to_zones(dggrs, zones)?)
    }
    fn zones_from_parent(
        &self,
        relative_depth: RelativeDepth,
        parent_zone_id: String,
        densify: bool,
    ) -> Result<Zones, GeoPlegmaError> {
        if relative_depth > self.max_relative_depth()? {
            return Err(GeoPlegmaError::RelativeDepthLimitReached {
                grid_name: self.grid_name.clone(),
                requested: relative_depth,
                maximum: self.max_refinement_level()?,
            });
        };
        let parent = parent_zone_id.parse::<u64>().expect("Invalid u64 string");

        let dggrs = get_dggrs(&self.grid_name)?;
        let target_depth =
            RefinementLevel::try_from(dggrs.getZoneLevel(parent) + i32::from(relative_depth))?; //FIX: probably better to implement Add for Depth
        if target_depth > self.max_refinement_level()? {
            return Err(GeoPlegmaError::RelativeDepthLimitReached {
                grid_name: self.grid_name.clone(),
                requested: relative_depth,
                maximum: self.max_refinement_level()?,
            });
        };

        let zones = dggrs.getSubZones(parent, i32::from(relative_depth));

        Ok(ids_to_zones(dggrs, zones)?)
    }
    fn zone_from_id(&self, zone_id: String, densify: bool) -> Result<Zones, GeoPlegmaError> {
        let dggrs = get_dggrs(&self.grid_name)?;
        let num: u64 = zone_id.parse::<u64>().expect("Invalid u64 string"); // FIX: parent_zone_id needs to be the ZoneID enum not String
        let zones = vec![num];

        Ok(ids_to_zones(dggrs, zones)?)
    }

    fn min_refinement_level(&self) -> Result<RefinementLevel, GeoPlegmaError> {
        Ok(RefinementLevel::new(0)?)
    }

    fn max_refinement_level(&self) -> Result<RefinementLevel, GeoPlegmaError> {
        let dggrs = get_dggrs(&self.grid_name).map_err(|_| DggalError::UnknownGrid {
            grid_name: self.grid_name.clone(),
        })?;

        let d = dggrs.getMaxDepth();

        Ok(RefinementLevel::new(d)?)
    }

    fn default_refinement_level(&self) -> Result<RefinementLevel, GeoPlegmaError> {
        Ok(RefinementLevel::new(2)?)
    }

    fn max_relative_depth(&self) -> Result<RelativeDepth, GeoPlegmaError> {
        Ok(RelativeDepth::new(4)?)
    }

    fn default_relative_depth(&self) -> Result<RelativeDepth, GeoPlegmaError> {
        Ok(RelativeDepth::new(2)?)
    }
}

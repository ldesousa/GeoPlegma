// Copyright 2025 contributors to the GeoPlegmata project.
// Originally authored by Michael Jendryke (GeoInsight GmbH, michael.jendryke@geoinsight.ai)
//
// Licenced under the Apache Licence, Version 2.0 <LICENCE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENCE-MIT or http://opensource.org/licenses/MIT>, at your
// discretion. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::adapters::h3o::common::{cells_to_zones, refinement_level_to_h3_resolution};
use crate::adapters::h3o::h3o::H3oAdapter;
use crate::error::h3o::H3oError;
use crate::error::port::GeoPlegmaError;
use crate::models::common::{RefinementLevel, RelativeDepth, ZoneId, Zones};
use crate::ports::dggrs::DggrsPort;
use geo::{Point, Rect};
use h3o::geom::{ContainmentMode, TilerBuilder};
use h3o::{CellIndex, LatLng, Resolution};
use std::str::FromStr;

pub const MAX_DEPTH: u8 = 10;

pub struct H3Impl {
    pub adapter: H3oAdapter,
}

impl H3Impl {
    pub fn new() -> Self {
        Self {
            adapter: H3oAdapter::new(),
        }
    }
}

impl Default for H3Impl {
    fn default() -> Self {
        Self {
            adapter: H3oAdapter::default(),
        }
    }
}

impl DggrsPort for H3Impl {
    fn zones_from_bbox(
        &self,
        refinement_level: RefinementLevel,
        _densify: bool,
        bbox: Option<Rect<f64>>,
    ) -> Result<Zones, GeoPlegmaError> {
        let cells: Vec<CellIndex>;

        let mut tiler =
            TilerBuilder::new(refinement_level_to_h3_resolution(RefinementLevel::new(2)?)?)
                .containment_mode(ContainmentMode::Covers)
                .build();

        if let Some(b) = bbox {
            // NOTE: adapt resolution dynamically based on bbox size & depth
            let _ = tiler.add(b.to_polygon());
            cells = tiler.into_coverage().collect::<Vec<_>>();
        } else {
            if refinement_level > self.default_refinement_level()? {
                return Err(GeoPlegmaError::DepthTooLarge(refinement_level));
            }
            cells = CellIndex::base_cells()
                .flat_map(|base| {
                    base.children(
                        refinement_level_to_h3_resolution(refinement_level)
                            .expect("Cannot translate to H3 Resolution"), // NOTE: expect() because flat_map does not understand Result?
                    )
                })
                .collect::<Vec<_>>();
        }
        Ok(cells_to_zones(cells)?)
    }
    fn zone_from_point(
        &self,
        refinement_level: RefinementLevel,
        point: Point, // TODO: we should support multiple points at once.
        _densify: bool,
    ) -> Result<Zones, GeoPlegmaError> {
        let coord = LatLng::new(point.x(), point.y()).expect("valid coord");

        let cell = coord.to_cell(refinement_level_to_h3_resolution(refinement_level)?);

        Ok(cells_to_zones(vec![cell])?)
    }
    fn zones_from_parent(
        &self,
        relative_depth: RelativeDepth,
        parent_zone_id: ZoneId,
        _densify: bool,
    ) -> Result<Zones, GeoPlegmaError> {
        let parent = CellIndex::from_str(&parent_zone_id.to_string()).map_err(|e| {
            GeoPlegmaError::H3o(H3oError::InvalidZoneID {
                zone_id: parent_zone_id.to_string(),
                source: e,
            })
        })?;

        let target_level = RefinementLevel::new(parent.resolution() as i32)?.add(relative_depth)?;

        if target_level > self.max_refinement_level()? {
            return Err(GeoPlegmaError::H3o(H3oError::ResolutionLimitReached {
                zone_id: parent.to_string(),
            }));
        }

        let children: Vec<CellIndex> = parent
            .children(refinement_level_to_h3_resolution(target_level)?)
            .collect();

        Ok(cells_to_zones(children)?)
    }
    fn zone_from_id(
        &self,
        zone_id: ZoneId, // ToDo: needs validation function
        _densify: bool,
    ) -> Result<Zones, GeoPlegmaError> {
        let cell = CellIndex::from_str(&zone_id.to_string()).map_err(|e| {
            GeoPlegmaError::H3o(H3oError::InvalidZoneID {
                zone_id: zone_id.to_string(),
                source: e,
            })
        })?;

        Ok(cells_to_zones(vec![cell])?)
    }

    fn min_refinement_level(&self) -> Result<RefinementLevel, GeoPlegmaError> {
        Ok(RefinementLevel::new(0)?) //NOTE: This is hardcoded from the Resolution Enum https://docs.rs/h3o/latest/h3o/enum.Resolution.html
    }

    fn max_refinement_level(&self) -> Result<RefinementLevel, GeoPlegmaError> {
        Ok(RefinementLevel::new(15)?) //NOTE: This is hardcoded from the Resolution Enum https://docs.rs/h3o/latest/h3o/enum.Resolution.html
    }

    fn default_refinement_level(&self) -> Result<RefinementLevel, GeoPlegmaError> {
        Ok(RefinementLevel::new(1)?)
    }

    fn max_relative_depth(&self) -> Result<RelativeDepth, GeoPlegmaError> {
        Ok(RelativeDepth::new(2)?)
    }

    fn default_relative_depth(&self) -> Result<RelativeDepth, GeoPlegmaError> {
        Ok(RelativeDepth::new(1)?)
    }
}

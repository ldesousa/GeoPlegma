// Copyright 2025 contributors to the GeoPlegmata project.
// Originally authored by Michael Jendryke (GeoInsight GmbH, michael.jendryke@geoinsight.ai)
//
// Licenced under the Apache Licence, Version 2.0 <LICENCE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENCE-MIT or http://opensource.org/licenses/MIT>, at your
// discretion. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::adapters::h3o::common::{cells_to_zones, res};
use crate::adapters::h3o::h3o::H3oAdapter;
use crate::error::h3o::H3oError;
use crate::error::port::GeoPlegmaError;
use crate::models::common::{Depth, RelativeDepth, Zones};
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
        depth: Depth,
        _densify: bool,
        bbox: Option<Rect<f64>>,
    ) -> Result<Zones, GeoPlegmaError> {
        let cells: Vec<CellIndex>;

        let mut tiler = TilerBuilder::new(res(2))
            .containment_mode(ContainmentMode::Covers)
            .build();

        if let Some(b) = bbox {
            // NOTE: adapt resolution dynamically based on bbox size & depth
            let _ = tiler.add(b.to_polygon());
            cells = tiler.into_coverage().collect::<Vec<_>>();
        } else {
            if depth > self.default_depth()? {
                return Err(GeoPlegmaError::DepthTooLarge(depth));
            }
            let r = u8::try_from(depth)?;
            cells = CellIndex::base_cells()
                .flat_map(|base| base.children(res(r)))
                .collect::<Vec<_>>();
        }
        Ok(cells_to_zones(cells)?)
    }
    fn zone_from_point(
        &self,
        depth: Depth,
        point: Point,
        _densify: bool,
    ) -> Result<Zones, GeoPlegmaError> {
        let coord = LatLng::new(point.x(), point.y()).expect("valid coord");

        let cell = coord.to_cell(res(u8::try_from(depth)?)); // TODO: we should support multiple points at once.

        Ok(cells_to_zones(vec![cell])?)
    }
    fn zones_from_parent(
        &self,
        relative_depth: RelativeDepth,
        zone_id: String, // ToDo: needs validation function
        _densify: bool,
    ) -> Result<Zones, GeoPlegmaError> {
        let parent = CellIndex::from_str(&zone_id).map_err(|e| {
            GeoPlegmaError::H3o(H3oError::InvalidZoneID {
                zone_id: zone_id.clone(),
                source: e,
            })
        })?;

        let parent_res = parent.resolution();
        let next = u8::from(parent_res) + u8::try_from(relative_depth)?;

        let target_res = Resolution::try_from(next).map_err(|e| {
            GeoPlegmaError::H3o(H3oError::MaxDepthReached {
                zone_id: zone_id.clone(),
                depth: next,
                source: e,
            })
        })?;

        let children: Vec<CellIndex> = parent.children(target_res).collect();

        Ok(cells_to_zones(children)?)
    }
    fn zone_from_id(
        &self,
        zone_id: String, // ToDo: needs validation function
        _densify: bool,
    ) -> Result<Zones, GeoPlegmaError> {
        let cell = CellIndex::from_str(&zone_id).map_err(|e| {
            GeoPlegmaError::H3o(H3oError::InvalidZoneID {
                zone_id: zone_id.clone(),
                source: e,
            })
        })?;

        Ok(cells_to_zones(vec![cell])?)
    }

    fn min_depth(&self) -> Result<Depth, GeoPlegmaError> {
        Ok(Depth::new(0)?) //NOTE: This is hardcoded from the Resolution Enum https://docs.rs/h3o/latest/h3o/enum.Resolution.html
    }

    fn max_depth(&self) -> Result<Depth, GeoPlegmaError> {
        Ok(Depth::new(15)?) //NOTE: This is hardcoded from the Resolution Enum https://docs.rs/h3o/latest/h3o/enum.Resolution.html
    }

    fn default_depth(&self) -> Result<Depth, GeoPlegmaError> {
        Ok(Depth::new(1)?)
    }

    fn max_relative_depth(&self) -> Result<RelativeDepth, GeoPlegmaError> {
        Ok(RelativeDepth::new(2)?)
    }
}

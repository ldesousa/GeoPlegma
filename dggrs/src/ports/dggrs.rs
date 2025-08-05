// Copyright 2025 contributors to the GeoPlegma project.
// Originally authored by Michael Jendryke (GeoInsight GmbH, michael.jendryke@geoinsight.ai)
//
// Licenced under the Apache Licence, Version 2.0 <LICENCE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENCE-MIT or http://opensource.org/licenses/MIT>, at your
// discretion. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::error::port::GeoPlegmaError;
use crate::models::common::{Depth, RelativeDepth, Zones};
use geo::Point;
use geo::Rect;

/// The DGGRS port trait. Each adapter can only implment the functions defined here.
pub trait DggrsPort: Send + Sync {
    /// Get zones for geo::Rect bounding box. If no bbox is supplied the whole world is taken.
    fn zones_from_bbox(
        &self,
        depth: Depth,
        densify: bool,
        bbox: Option<Rect<f64>>,
    ) -> Result<Zones, GeoPlegmaError>;

    /// Get zones for a geo::Point.
    fn zone_from_point(
        &self,
        depth: Depth,
        point: Point,
        densify: bool,
    ) -> Result<Zones, GeoPlegmaError>; // NOTE:Consider accepting a vector of Points.

    /// Get zones based on a parent ZoneID.
    fn zones_from_parent(
        &self,
        relative_depth: RelativeDepth, // FIX: This needs to be relative depth!
        parent_zone_id: String, // FIX: This needs to be ZoneID (so integer or string), see relevant enum.
        densify: bool,
    ) -> Result<Zones, GeoPlegmaError>;

    /// Get a zone based on a ZoneID
    fn zone_from_id(&self, zone_id: String, densify: bool) -> Result<Zones, GeoPlegmaError>; // NOTE: Consider accepting a vector of ZoneIDs

    /// Get the minimum depth of a DGGRS
    fn min_depth(&self) -> Result<Depth, GeoPlegmaError>;

    /// Get the maximum depth of a DGGRS
    fn max_depth(&self) -> Result<Depth, GeoPlegmaError>;

    /// Get the default depth of a DGGRS
    fn default_depth(&self) -> Result<Depth, GeoPlegmaError>;

    /// Get the  max relative depth of a DGGRS
    fn max_relative_depth(&self) -> Result<RelativeDepth, GeoPlegmaError>;
}

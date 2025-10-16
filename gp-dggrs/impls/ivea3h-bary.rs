// Copyright 2025 contributors to the GeoPlegma project.
// Originally authored by Luís Moreira de Sousa, Técnico, ULisboa 
// (luis.moreira.de.sousa [at] tecnico.ulisboa.pt)
//
// Licenced under the Apache Licence, Version 2.0 <LICENCE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENCE-MIT or http://opensource.org/licenses/MIT>, at your
// discretion. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::error::DggrsError;
use crate::models::common::{RefinementLevel, RelativeDepth, ZoneId, Zones};
use geo::{Point, Rect};

pub struct IVEA3H-Bary {
 
    const APERTURE: i32 = 3;
}

impl IVEA3H-Bary {

    // Denominator is a power of the aperture, but only increases every other resolution.
    fn compute_denom(refinement_level:RefinementLevel) {
        APERTURE.pow((level+level/2)/2); 
    }

    // Fake method for the time being - then use the Projection module
    fn project(Point) {
        [0.45; 0.22; (1-0.45-0.22)];
    }

    fn bundle_index(i: int32, j:int32, refinement_level:RefinementLevel, face:int32){
        // do something
    }

    // Computes distance with barycentric coordinates defined by an equilateral triangle.
    fn bary_distance(i1:float64, j1:float64, i2:float64, j2:float64) {
        let d1 = i1 - j1;
        let d2 = i2 - j2;
        d1.pow(2) + d2.pow(2) + d1 * d2;
    }
}

impl DggrsApi for IVEA3H-Bary {
    
    fn zone_from_point(
        &self,
        refinement_level: RefinementLevel,
        point: Point, 
        config: Option<DggrsApiConfig>,
    ) -> Result<Zones, DggrsError> {

        let bary = project(Point);
        let denom = compute_denom(refinement_level);
        let mut zone_centre = [1;1]; // the result

        let mut candidates = Vec::new();
        
        let i_down = (bary[0]*denom).floor();
        let i_up   = (bary[0]*denom).ceil();

        // Even case
        if (refinement_level % 2) > 0 {
            let j_down = (bary[1]/denom).floor();
            let j_up = (bary[1]/denom).ceil();
            candidates.push([i_down; j_down]);
            candidates.push([i_down; j_up]);
            candidates.push([i_up; j_down]);
            candidates.push([i_up; j_up]);
        }
        // Odd case
        else {
            let start_down = i_down % aperture; 
            let start_up = i_up % aperture; 
            let num_hops = bary[1] * denom / aperture; // integer division
            let j_down = start_down + num_hops * aperture;
            let j_up = start_up + num_hops * aperture;
            candidates.push([i_down; j_down]);
            candidates.push([i_down; j_down + 1]);
            candidates.push([i_up; j_up]);
            candidates.push([i_up; j_up + 1]);
        }

        // Find closest cell centre
        let current_dist = i32::MAX
        while candidates::length() > 0 {
            centre = candidates.pop();
            dist = bary_distance(centre[0], bary[0], centre[1], bary[]);
            if (dist < current_dist) {
                current_dist = dist;
                zone_centre = centre;
            }
        }

        // Bundle coords into index
        bundle_index(zone_centre[0], zone_centre[1], refinement_level, bary[3]);
    }
}


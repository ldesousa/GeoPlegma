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

        //x,y - reals between 0 and 1
        //n,m - unknown, but integers between 0 and denom
        //i - numerator which over d equates to x 
        //j - numerator which over d equates to y

        let i_down = (bary[0]*denom).floor();
        let i_up   = (bary[0]*denom).ceil();

        //v - p < 3 (level) p - w < 3 (level)

        // Even case
        if (refinement_level % 2) > 0 {
            let j_down = (bary[1]/denom).floor();
            let j_up = (bary[1]/denom).ceil();
        }
        // Odd case
        else {
            let start_down = i_down % aperture; 
            let start_up = i_up % aperture; 
            let num_hops = (bary[1]*denom/aperture); // integer division
            let j_down = start_down + num_hops * aperture;
            let j_up = start_up + (num_hops+1) * aperture;
        }

        // Find closest cell centre

        // Bundle coords into index
        bundle_index(i, j, refinement_level, bary[3])
    }
}


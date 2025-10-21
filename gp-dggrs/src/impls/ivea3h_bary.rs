// Copyright 2025 contributors to the GeoPlegma project.
// Originally authored by Luís Moreira de Sousa, Técnico, ULisboa 
// (luis.moreira.de.sousa [at] tecnico.ulisboa.pt)
//
// Licenced under the Apache Licence, Version 2.0 <LICENCE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENCE-MIT or http://opensource.org/licenses/MIT>, at your
// discretion. This file may not be copied, modified, or distributed
// except according to those terms.

mod ivea3hbary {

    use crate::sys_api::DggrsSysApi;
    use api::error::DggrsError;
    use api::models::common::{RefinementLevel, RelativeDepth, ZoneId, Zones};
    use geo::Point;

    pub struct IVEA3HBary {}
    
    impl IVEA3HBary {
    
        // Denominator is a power of the APERTURE, but only increases every other resolution.
        fn compute_denom(refinement_level:RefinementLevel) {
            Self::APERTURE.pow((refinement_level+refinement_level/2)/2); 
        }
    
        // Fake method for the time being - then use the Projection module
        fn project(point:Point) {
            [0.45, 0.22, (1.0-0.45-0.22)];
        }
    
        fn bundle_index(i:i32, j:i32, refinement_level:RefinementLevel, face:i32){
            // do something
        }
    
        // Computes distance with barycentric coordinates defined by an equilateral triangle.
        fn bary_distance(i1:f64, j1:f64, i2:f64, j2:f64) -> f64 {
            let d1 = i1 - j1;
            let d2 = i2 - j2;
            return d1.pow(2) + d2.pow(2) + d1 * d2;
        }
    }
    
    impl DggrsSysApi for IVEA3HBary {
        
        const APERTURE: i32 = 3;
        
        fn zone_from_point(
            &self,
            refinement_level: RefinementLevel,
            point: Point, 
            //config: Option<DggrsApiConfig>,
        ) -> Result<Zones, DggrsError> {
    
            let bary = project(point);
            let denom = compute_denom(refinement_level);
            let mut zone_centre = [1;1]; // the result
    
            let mut candidates = Vec::new();
            
            let i_down = (bary.0 * denom).floor();
            let i_up   = (bary.0 * denom).ceil();
    
            // Even case
            if (refinement_level % 2) > 0 {
                let j_down:i32 = (bary[1]/denom).floor();
                let j_up:i32 = (bary[1]/denom).ceil();
                candidates.push([i_down, j_down]);
                candidates.push([i_down, j_up]);
                candidates.push([i_up, j_down]);
                candidates.push([i_up, j_up]);
            }
            // Odd case
            else {
                let start_down = i_down % Self::APERTURE; 
                let start_up = i_up % Self::APERTURE; 
                let num_hops = bary[1] * denom / Self::APERTURE; // integer division
                let j_down:i32 = start_down + num_hops * Self::APERTURE;
                let j_up:i32 = start_up + num_hops * Self::APERTURE;
                candidates.push([i_down, j_down]);
                candidates.push([i_down, j_down + 1]);
                candidates.push([i_up, j_up]);
                candidates.push([i_up, j_up + 1]);
            }
    
            // Find closest cell centre
            let current_dist = f64::MAX;
            while candidates.length() > 0 {
                let centre = candidates.pop();
                let dist = Self::bary_distance(centre[0], bary.0, centre[1], bary.1);
                if dist < current_dist {
                    current_dist = dist;
                    zone_centre = centre;
                }
            }
    
            // Bundle coords into index
            // bundle_index(zone_centre[0], zone_centre[1], refinement_level, bary[3]);

            zone_centre;
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn zone_from_point() {
    
         let zone = zone_from_point(3, Point(0.45,0.22));
         assert_eq!(zone[0], 4);
         assert_eq!(zone[1], 1);   
    }

}

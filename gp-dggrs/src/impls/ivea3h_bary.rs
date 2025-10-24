// Copyright 2025 contributors to the GeoPlegma project.
// Originally authored by Luís Moreira de Sousa, Técnico, ULisboa 
// (luis.moreira.de.sousa [at] tecnico.ulisboa.pt)
//
// Licenced under the Apache Licence, Version 2.0 <LICENCE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENCE-MIT or http://opensource.org/licenses/MIT>, at your
// discretion. This file may not be copied, modified, or distributed
// except according to those terms.

mod ivea3h_bary {

    use crate::sys_api::DggrsSysApi;
    //use api::error::DggrsError;
    use api::models::common::{RefinementLevel};//, Zones};
    use geo::Point;

    pub struct IVEA3HBary {}
    
    #[allow(dead_code)]
    impl IVEA3HBary {
    
        // Denominator is a power of the APERTURE, but only increases every other resolution.
        fn compute_denom(refinement_level:RefinementLevel) -> u32 {
            return Self::APERTURE.pow((refinement_level.get() as u32 +refinement_level.get() as u32 / 2) / 2) as u32; 
        }
    
        // Fake method for the time being - then use the Projection module
        pub fn project(_point:Point) -> (f64, f64, f64){
            return (0.45, 0.22, (1.0-0.45-0.22));
        }
    
        fn bundle_index(_i:i32, _j:i32, _refinement_level:RefinementLevel, _face:i32){
            // do something
        }
    
        // Computes distance with barycentric coordinates defined by an equilateral triangle.
        fn bary_distance(i1:f64, j1:f64, i2:f64, j2:f64) -> f64 {
            let d1 = i1 - j1;
            let d2 = i2 - j2;
            return d1.powi(2) + d2.powi(2) + d1 * d2;
        }
    }
    
    impl DggrsSysApi for IVEA3HBary {
        
        const APERTURE: u32 = 3;
        
        fn zone_from_point(
            &self,
            refinement_level: RefinementLevel,
            point: Point, 
            //config: Option<DggrsApiConfig>,
        ) -> (u32, u32) {
    
            let bary = IVEA3HBary::project(point);
            let denom = IVEA3HBary::compute_denom(refinement_level);
            let mut zone_centre = (1 as u32, 1 as u32); // the result
    
            //let mut candidates = Vec<(u32, u32)>::new();
            let mut candidates: Vec<(u32, u32)> = Vec::new();
            
            let i_down = (bary.0 * denom as f64).floor() as u32;
            let i_up   = (bary.0 * denom as f64).ceil() as u32;
    
            // Even case
            if (refinement_level.get() % 2) > 0 {
                let j_down:u32 = (bary.1 / denom as f64).floor() as u32;
                let j_up:u32 = (bary.1 / denom as f64).ceil() as u32;
                candidates.push((i_down, j_down));
                candidates.push((i_down, j_up));
                candidates.push((i_up, j_down));
                candidates.push((i_up, j_up));
            }
            // Odd case
            else {
                let start_down = i_down % Self::APERTURE; 
                let start_up = i_up % Self::APERTURE; 
                let num_hops = (bary.1 * denom as f64 / Self::APERTURE as f64).floor() as u32; // integer division
                let j_down:u32 = start_down + num_hops * Self::APERTURE;
                let j_up:u32 = start_up + num_hops * Self::APERTURE;
                candidates.push((i_down, j_down));
                candidates.push((i_down, j_down + 1));
                candidates.push((i_up, j_up));
                candidates.push((i_up, j_up + 1));
            }
    
            // Find closest cell centre
            let mut current_dist = f64::MAX;
            while candidates.len() > 0 {
                let centre = candidates.pop().unwrap();
                let dist = Self::bary_distance(centre.0.into(), bary.0, centre.1.into(), bary.1);
                if dist < current_dist {
                    current_dist = dist;
                    zone_centre = centre;
                }
            }
    
            // Bundle coords into index
            // bundle_index(zone_centre.0, zone_centre.1, refinement_level, bary.3);

            return zone_centre;
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_zone_from_point() {

        let system = IVEA3HBary {};        
        let zone = system.zone_from_point(3, Point(0.45,0.22));
        assert_eq!(zone.0, 4);
        assert_eq!(zone.1, 1);   
    }

}

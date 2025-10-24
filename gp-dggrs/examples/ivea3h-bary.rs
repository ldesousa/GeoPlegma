// Copyright 2025 contributors to the GeoPlegma project.
// Originally authored by Luís Moreira de Sousa, Técnico, ULisboa 
// (luis.moreira.de.sousa [at] tecnico.ulisboa.pt)
//
// Licenced under the Apache Licence, Version 2.0 <LICENCE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENCE-MIT or http://opensource.org/licenses/MIT>, at your
// discretion. This file may not be copied, modified, or distributed
// except according to those terms.
use gp_dggrs::impls::ivea3h_bary::IVEA3HBary;
use geo::Point;

/// This is just an example and basic testing function if there is output or not
//pub fn main() -> Result<(), std::io::Error> {
pub fn main() {

    let p1 = Point::new(0.45, 0.22);
    let p2 = Point::new(0.21, 0.64);

    let system = IVEA3HBary {};

    let zone1 = system.zone_from_point(3, p1);
    assert_eq!(zone1.0, 4);
    assert_eq!(zone1.1, 1);   

    let zone2 = system.zone_from_point(3, p2);
    assert_eq!(zone2.0, 2);
    assert_eq!(zone2.1, 5);   
}

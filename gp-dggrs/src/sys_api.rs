//use api::error::DggrsError;
use api::models::common::{RefinementLevel};//, Zones};
use geo::Point;

pub trait DggrsSysApi {

    const APERTURE: u32;

    fn zone_from_point(
        &self,
        _refinement_level: RefinementLevel,
        _point: Point, 
        //config: Option<DggrsApiConfig>,
    ) -> (u32, u32) {

        return (0, 0); 
    }

    fn get_children(){}

}

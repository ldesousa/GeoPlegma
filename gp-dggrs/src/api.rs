use crate::api::error::DggrsError;
use crate::api::models::common::{RefinementLevel, Zones};
use geo::Point;

pub trait DggrsSysApi {

    const APERTURE: i32;

    fn zone_from_point(
        &self,
        refinement_level: RefinementLevel,
        point: Point, 
        //config: Option<DggrsApiConfig>,
    ) -> Result<Zones, DggrsError> {}

    fn get_children(){}

}

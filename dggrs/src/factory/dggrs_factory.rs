// Copyright 2025 contributors to the GeoPlegma project.
// Originally authored by Michael Jendryke, GeoInsight (michael.jendryke@geoinsight.ai)
//
// Licenced under the Apache Licence, Version 2.0 <LICENCE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENCE-MIT or http://opensource.org/licenses/MIT>, at your
// discretion. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::adapters::{
    dggal::grids::DggalImpl, dggrid::igeo7::Igeo7Impl, dggrid::isea3h::Isea3hImpl, h3o::h3::H3Impl,
};
use crate::error::factory::FactoryError;
use crate::models::common::{DggrsId, DggrsTool};
use crate::ports::dggrs::DggrsPort;
use std::sync::Arc;

pub fn get(id: DggrsId) -> Result<Arc<dyn DggrsPort>, FactoryError> {
    match id.spec().tool {
        DggrsTool::DGGRID => match id {
            DggrsId::ISEA3HDGGRID => Ok(Arc::new(Isea3hImpl::default())),
            DggrsId::IGEO7 => Ok(Arc::new(Igeo7Impl::default())),
            _ => Err(FactoryError::UnsupportedCombination {
                tool: DggrsTool::DGGRID,
                id,
            }),
        },

        DggrsTool::H3O => match id {
            DggrsId::H3 => Ok(Arc::new(H3Impl::default())),
            _ => Err(FactoryError::UnsupportedCombination {
                tool: DggrsTool::H3O,
                id,
            }),
        },

        DggrsTool::DGGAL => match id {
            // All the DGGAL-backed IDs you support:
            DggrsId::ISEA3HDGGAL
            | DggrsId::IVEA3H
            | DggrsId::ISEA9R
            | DggrsId::IVEA9R
            | DggrsId::RTEA3H
            | DggrsId::RTEA9R => Ok(Arc::new(DggalImpl::new(id))), // change DggalImpl::new to take DggrsId
            _ => Err(FactoryError::UnsupportedCombination {
                tool: DggrsTool::DGGAL,
                id,
            }),
        },

        DggrsTool::Native => Err(FactoryError::UnsupportedCombination {
            tool: DggrsTool::Native,
            id,
        }),
    }
}

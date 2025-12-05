// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __EventGenerator struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __EventGenerator {
    #[serde(flatten)]
    pub base: __IndicationRelated,
}

impl __EventGenerator {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __IndicationRelated::new(),
        }
    }

}


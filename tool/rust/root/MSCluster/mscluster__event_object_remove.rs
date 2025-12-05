// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_EventObjectRemove struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_EventObjectRemove {
    #[serde(flatten)]
    pub base: MSCluster_Event,
}

impl MSCluster_EventObjectRemove {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_Event::new(),
        }
    }

}


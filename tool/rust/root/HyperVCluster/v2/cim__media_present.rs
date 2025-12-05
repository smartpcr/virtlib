// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_MediaPresent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_MediaPresent {
    #[serde(flatten)]
    pub base: CIM_Dependency,

/// Boolean indicating that the accessed StorageExtent is fixed in the MediaAccessDevice and can not be ejected.
    #[serde(rename = "FixedMedia")]
    pub fixed_media: Option<bool>,
}

impl CIM_MediaPresent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Dependency::new(),
            fixed_media: None,
        }
    }


    /// Sets the value of FixedMedia
    pub fn set_fixed_media(&mut self, value: bool) {
        self.fixed_media = Some(value);
    }

    /// Gets the value of FixedMedia
    pub fn get_fixed_media(&self) -> Option<&bool> {
        self.fixed_media.as_ref()
    }
}


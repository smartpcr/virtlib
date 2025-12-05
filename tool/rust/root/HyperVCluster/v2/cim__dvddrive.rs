// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DVDDrive struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DVDDrive {
    #[serde(flatten)]
    pub base: CIM_MediaAccessDevice,

/// The CD and DVD formats that are supported by this Device. For example, the Drive may support "CD-ROM" and "DVD-RAM". In this case, the values 16 and 24 would be written to the array. This property's values align with those defined in PhysicalMedia.MediaType.
    #[serde(rename = "FormatsSupported")]
    pub formats_supported: Vec<DVDDrive_FormatsSupported>,
}

impl CIM_DVDDrive {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_MediaAccessDevice::new(),
            formats_supported: Vec::new(),
        }
    }


    /// Sets the value of FormatsSupported
    pub fn set_formats_supported(&mut self, value: Vec<DVDDrive_FormatsSupported>) {
        self.formats_supported = value;
    }

    /// Gets the value of FormatsSupported
    pub fn get_formats_supported(&self) -> &Vec<DVDDrive_FormatsSupported> {
        &self.formats_supported
    }
}


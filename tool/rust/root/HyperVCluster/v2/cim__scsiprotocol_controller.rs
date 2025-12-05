// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SCSIProtocolController struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SCSIProtocolController {
    #[serde(flatten)]
    pub base: CIM_ProtocolController,

/// The NameFormat property identifies how the Name of the SCSIProtocolController is selected. 
/// For Fibre Channel, the NameFormat is 'FC Port WWN'. 
/// For iSCSI, Name can use any of the 3 iSCSI formats (iqn, eui, naa) which include the iSCSI format as as a prefix in the name, so they are not ambiguous.
    #[serde(rename = "NameFormat")]
    pub name_format: Option<SCSIProtocolController_NameFormat>,

/// A string describing how the ProtocolController is identified when the NameFormat is "Other".
    #[serde(rename = "OtherNameFormat")]
    pub other_name_format: Option<String>,
}

impl CIM_SCSIProtocolController {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ProtocolController::new(),
            name_format: None,
            other_name_format: None,
        }
    }


    /// Sets the value of NameFormat
    pub fn set_name_format(&mut self, value: SCSIProtocolController_NameFormat) {
        self.name_format = Some(value);
    }

    /// Gets the value of NameFormat
    pub fn get_name_format(&self) -> Option<&SCSIProtocolController_NameFormat> {
        self.name_format.as_ref()
    }

    /// Sets the value of OtherNameFormat
    pub fn set_other_name_format(&mut self, value: String) {
        self.other_name_format = Some(value);
    }

    /// Gets the value of OtherNameFormat
    pub fn get_other_name_format(&self) -> Option<&String> {
        self.other_name_format.as_ref()
    }
}


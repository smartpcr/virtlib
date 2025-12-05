// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_IPAssignmentSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_IPAssignmentSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 689
    #[serde(rename = "AddressOrigin")]
    pub address_origin: Option<IPAssignmentSettingData_AddressOrigin>,

/// 693
    #[serde(rename = "ProtocolIFType")]
    pub protocol_iftype: Option<IPAssignmentSettingData_ProtocolIFType>,
}

impl CIM_IPAssignmentSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            address_origin: None,
            protocol_iftype: None,
        }
    }


    /// Sets the value of AddressOrigin
    pub fn set_address_origin(&mut self, value: IPAssignmentSettingData_AddressOrigin) {
        self.address_origin = Some(value);
    }

    /// Gets the value of AddressOrigin
    pub fn get_address_origin(&self) -> Option<&IPAssignmentSettingData_AddressOrigin> {
        self.address_origin.as_ref()
    }

    /// Sets the value of ProtocolIFType
    pub fn set_protocol_iftype(&mut self, value: IPAssignmentSettingData_ProtocolIFType) {
        self.protocol_iftype = Some(value);
    }

    /// Gets the value of ProtocolIFType
    pub fn get_protocol_iftype(&self) -> Option<&IPAssignmentSettingData_ProtocolIFType> {
        self.protocol_iftype.as_ref()
    }
}


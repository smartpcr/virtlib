// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_EthernetPortAllocationSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_EthernetPortAllocationSettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// The desired VLAN mode that is requested for use. This property is usedto set the initial OperationalEndpointMode property value in theinstance of CIM_VLANEndpoint associated with the targeted Ethernet Port.Refer to the description for the property OperationalEndpointMode inCIM_VLANEndpoint for a description of the values
    #[serde(rename = "DesiredVLANEndpointMode")]
    pub desired_vlanendpoint_mode: Option<EthernetPortAllocationSettingData_DesiredVLANEndpointMode>,

/// A string describing the type of VLAN endpoint model that is supported by this VLANEndpoint, when the value of the mode property is set to 1 (i.e., "Other"). This property should be set to NULL when the mode property is any value other than 1.
    #[serde(rename = "OtherEndpointMode")]
    pub other_endpoint_mode: Option<String>,
}

impl CIM_EthernetPortAllocationSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            desired_vlanendpoint_mode: None,
            other_endpoint_mode: None,
        }
    }


    /// Sets the value of DesiredVLANEndpointMode
    pub fn set_desired_vlanendpoint_mode(&mut self, value: EthernetPortAllocationSettingData_DesiredVLANEndpointMode) {
        self.desired_vlanendpoint_mode = Some(value);
    }

    /// Gets the value of DesiredVLANEndpointMode
    pub fn get_desired_vlanendpoint_mode(&self) -> Option<&EthernetPortAllocationSettingData_DesiredVLANEndpointMode> {
        self.desired_vlanendpoint_mode.as_ref()
    }

    /// Sets the value of OtherEndpointMode
    pub fn set_other_endpoint_mode(&mut self, value: String) {
        self.other_endpoint_mode = Some(value);
    }

    /// Gets the value of OtherEndpointMode
    pub fn get_other_endpoint_mode(&self) -> Option<&String> {
        self.other_endpoint_mode.as_ref()
    }
}


// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_VLANEndpointSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_VLANEndpointSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// The access VLAN for the referenced VLANEndpoint.
    #[serde(rename = "AccessVLAN")]
    pub access_vlan: Option<u16>,

/// Default value for the native VLAN on this trunk endpoint/port. This property is applicable only when the endpoint is operating in trunking mode (determined by examining the OperationalEndpointMode property).
    #[serde(rename = "DefaultVLAN")]
    pub default_vlan: Option<u16>,

/// VLAN Id that is used to tag untagged traffic received on this trunk endpoint/port. This property is applicable only when the endpoint is operating in trunking mode (determined by examining the SwitchEndpointMode property).
    #[serde(rename = "NativeVLAN")]
    pub native_vlan: Option<u16>,

/// If a VLAN Id is part of this array, then the system MAY prune that VLAN on the related trunk endpoint/port. This property is applicable only when the endpoint is operating in trunking mode (determined by examining the OperationalEndpointMode property).
    #[serde(rename = "PruneEligibleVLANList")]
    pub prune_eligible_vlanlist: Vec<u16>,

/// If a VLAN Id is part of this array, then the system MAY trunk the traffic on the related endpoint/port. This property is applicable only when the endpoint is operating in trunking mode (determined by examining the OperationalEndpointMode property).
    #[serde(rename = "TrunkedVLANList")]
    pub trunked_vlanlist: Vec<u16>,
}

impl CIM_VLANEndpointSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            access_vlan: None,
            default_vlan: None,
            native_vlan: None,
            prune_eligible_vlanlist: Vec::new(),
            trunked_vlanlist: Vec::new(),
        }
    }


    /// Sets the value of AccessVLAN
    pub fn set_access_vlan(&mut self, value: u16) {
        self.access_vlan = Some(value);
    }

    /// Gets the value of AccessVLAN
    pub fn get_access_vlan(&self) -> Option<&u16> {
        self.access_vlan.as_ref()
    }

    /// Sets the value of DefaultVLAN
    pub fn set_default_vlan(&mut self, value: u16) {
        self.default_vlan = Some(value);
    }

    /// Gets the value of DefaultVLAN
    pub fn get_default_vlan(&self) -> Option<&u16> {
        self.default_vlan.as_ref()
    }

    /// Sets the value of NativeVLAN
    pub fn set_native_vlan(&mut self, value: u16) {
        self.native_vlan = Some(value);
    }

    /// Gets the value of NativeVLAN
    pub fn get_native_vlan(&self) -> Option<&u16> {
        self.native_vlan.as_ref()
    }

    /// Sets the value of PruneEligibleVLANList
    pub fn set_prune_eligible_vlanlist(&mut self, value: Vec<u16>) {
        self.prune_eligible_vlanlist = value;
    }

    /// Gets the value of PruneEligibleVLANList
    pub fn get_prune_eligible_vlanlist(&self) -> &Vec<u16> {
        &self.prune_eligible_vlanlist
    }

    /// Sets the value of TrunkedVLANList
    pub fn set_trunked_vlanlist(&mut self, value: Vec<u16>) {
        self.trunked_vlanlist = value;
    }

    /// Gets the value of TrunkedVLANList
    pub fn get_trunked_vlanlist(&self) -> &Vec<u16> {
        &self.trunked_vlanlist
    }
}


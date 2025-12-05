// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_DeviceInstallation02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_DeviceInstallation02 {

/// 
    #[serde(rename = "AllowInstallationOfMatchingDeviceIDs")]
    pub allow_installation_of_matching_device_ids: Option<String>,

/// 
    #[serde(rename = "AllowInstallationOfMatchingDeviceInstanceIDs")]
    pub allow_installation_of_matching_device_instance_ids: Option<String>,

/// 
    #[serde(rename = "AllowInstallationOfMatchingDeviceSetupClasses")]
    pub allow_installation_of_matching_device_setup_classes: Option<String>,

/// 
    #[serde(rename = "EnableInstallationPolicyLayering")]
    pub enable_installation_policy_layering: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PreventDeviceMetadataFromNetwork")]
    pub prevent_device_metadata_from_network: Option<String>,

/// 
    #[serde(rename = "PreventInstallationOfDevicesNotDescribedByOtherPolicySettings")]
    pub prevent_installation_of_devices_not_described_by_other_policy_settings: Option<String>,

/// 
    #[serde(rename = "PreventInstallationOfMatchingDeviceIDs")]
    pub prevent_installation_of_matching_device_ids: Option<String>,

/// 
    #[serde(rename = "PreventInstallationOfMatchingDeviceInstanceIDs")]
    pub prevent_installation_of_matching_device_instance_ids: Option<String>,

/// 
    #[serde(rename = "PreventInstallationOfMatchingDeviceSetupClasses")]
    pub prevent_installation_of_matching_device_setup_classes: Option<String>,
}

impl MDM_Policy_Result01_DeviceInstallation02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_installation_of_matching_device_ids: None,
            allow_installation_of_matching_device_instance_ids: None,
            allow_installation_of_matching_device_setup_classes: None,
            enable_installation_policy_layering: None,
            instance_id: None,
            parent_id: None,
            prevent_device_metadata_from_network: None,
            prevent_installation_of_devices_not_described_by_other_policy_settings: None,
            prevent_installation_of_matching_device_ids: None,
            prevent_installation_of_matching_device_instance_ids: None,
            prevent_installation_of_matching_device_setup_classes: None,
        }
    }


    /// Sets the value of AllowInstallationOfMatchingDeviceIDs
    pub fn set_allow_installation_of_matching_device_ids(&mut self, value: String) {
        self.allow_installation_of_matching_device_ids = Some(value);
    }

    /// Gets the value of AllowInstallationOfMatchingDeviceIDs
    pub fn get_allow_installation_of_matching_device_ids(&self) -> Option<&String> {
        self.allow_installation_of_matching_device_ids.as_ref()
    }

    /// Sets the value of AllowInstallationOfMatchingDeviceInstanceIDs
    pub fn set_allow_installation_of_matching_device_instance_ids(&mut self, value: String) {
        self.allow_installation_of_matching_device_instance_ids = Some(value);
    }

    /// Gets the value of AllowInstallationOfMatchingDeviceInstanceIDs
    pub fn get_allow_installation_of_matching_device_instance_ids(&self) -> Option<&String> {
        self.allow_installation_of_matching_device_instance_ids.as_ref()
    }

    /// Sets the value of AllowInstallationOfMatchingDeviceSetupClasses
    pub fn set_allow_installation_of_matching_device_setup_classes(&mut self, value: String) {
        self.allow_installation_of_matching_device_setup_classes = Some(value);
    }

    /// Gets the value of AllowInstallationOfMatchingDeviceSetupClasses
    pub fn get_allow_installation_of_matching_device_setup_classes(&self) -> Option<&String> {
        self.allow_installation_of_matching_device_setup_classes.as_ref()
    }

    /// Sets the value of EnableInstallationPolicyLayering
    pub fn set_enable_installation_policy_layering(&mut self, value: String) {
        self.enable_installation_policy_layering = Some(value);
    }

    /// Gets the value of EnableInstallationPolicyLayering
    pub fn get_enable_installation_policy_layering(&self) -> Option<&String> {
        self.enable_installation_policy_layering.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PreventDeviceMetadataFromNetwork
    pub fn set_prevent_device_metadata_from_network(&mut self, value: String) {
        self.prevent_device_metadata_from_network = Some(value);
    }

    /// Gets the value of PreventDeviceMetadataFromNetwork
    pub fn get_prevent_device_metadata_from_network(&self) -> Option<&String> {
        self.prevent_device_metadata_from_network.as_ref()
    }

    /// Sets the value of PreventInstallationOfDevicesNotDescribedByOtherPolicySettings
    pub fn set_prevent_installation_of_devices_not_described_by_other_policy_settings(&mut self, value: String) {
        self.prevent_installation_of_devices_not_described_by_other_policy_settings = Some(value);
    }

    /// Gets the value of PreventInstallationOfDevicesNotDescribedByOtherPolicySettings
    pub fn get_prevent_installation_of_devices_not_described_by_other_policy_settings(&self) -> Option<&String> {
        self.prevent_installation_of_devices_not_described_by_other_policy_settings.as_ref()
    }

    /// Sets the value of PreventInstallationOfMatchingDeviceIDs
    pub fn set_prevent_installation_of_matching_device_ids(&mut self, value: String) {
        self.prevent_installation_of_matching_device_ids = Some(value);
    }

    /// Gets the value of PreventInstallationOfMatchingDeviceIDs
    pub fn get_prevent_installation_of_matching_device_ids(&self) -> Option<&String> {
        self.prevent_installation_of_matching_device_ids.as_ref()
    }

    /// Sets the value of PreventInstallationOfMatchingDeviceInstanceIDs
    pub fn set_prevent_installation_of_matching_device_instance_ids(&mut self, value: String) {
        self.prevent_installation_of_matching_device_instance_ids = Some(value);
    }

    /// Gets the value of PreventInstallationOfMatchingDeviceInstanceIDs
    pub fn get_prevent_installation_of_matching_device_instance_ids(&self) -> Option<&String> {
        self.prevent_installation_of_matching_device_instance_ids.as_ref()
    }

    /// Sets the value of PreventInstallationOfMatchingDeviceSetupClasses
    pub fn set_prevent_installation_of_matching_device_setup_classes(&mut self, value: String) {
        self.prevent_installation_of_matching_device_setup_classes = Some(value);
    }

    /// Gets the value of PreventInstallationOfMatchingDeviceSetupClasses
    pub fn get_prevent_installation_of_matching_device_setup_classes(&self) -> Option<&String> {
        self.prevent_installation_of_matching_device_setup_classes.as_ref()
    }
}


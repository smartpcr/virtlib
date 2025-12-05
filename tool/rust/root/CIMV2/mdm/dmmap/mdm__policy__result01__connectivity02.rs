// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_Connectivity02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_Connectivity02 {

/// 
    #[serde(rename = "AllowBluetooth")]
    pub allow_bluetooth: Option<i32>,

/// 
    #[serde(rename = "AllowCellularData")]
    pub allow_cellular_data: Option<i32>,

/// 
    #[serde(rename = "AllowCellularDataRoaming")]
    pub allow_cellular_data_roaming: Option<i32>,

/// 
    #[serde(rename = "AllowConnectedDevices")]
    pub allow_connected_devices: Option<i32>,

/// 
    #[serde(rename = "AllowPhonePCLinking")]
    pub allow_phone_pclinking: Option<i32>,

/// 
    #[serde(rename = "AllowVPNOverCellular")]
    pub allow_vpnover_cellular: Option<i32>,

/// 
    #[serde(rename = "AllowVPNRoamingOverCellular")]
    pub allow_vpnroaming_over_cellular: Option<i32>,

/// 
    #[serde(rename = "DiablePrintingOverHTTP")]
    pub diable_printing_over_http: Option<String>,

/// 
    #[serde(rename = "DisableDownloadingOfPrintDriversOverHTTP")]
    pub disable_downloading_of_print_drivers_over_http: Option<String>,

/// 
    #[serde(rename = "DisableInternetDownloadForWebPublishingAndOnlineOrderingWizards")]
    pub disable_internet_download_for_web_publishing_and_online_ordering_wizards: Option<String>,

/// 
    #[serde(rename = "DisallowNetworkConnectivityActiveTests")]
    pub disallow_network_connectivity_active_tests: Option<i32>,

/// 
    #[serde(rename = "HardenedUNCPaths")]
    pub hardened_uncpaths: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "ProhibitInstallationAndConfigurationOfNetworkBridge")]
    pub prohibit_installation_and_configuration_of_network_bridge: Option<String>,
}

impl MDM_Policy_Result01_Connectivity02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_bluetooth: None,
            allow_cellular_data: None,
            allow_cellular_data_roaming: None,
            allow_connected_devices: None,
            allow_phone_pclinking: None,
            allow_vpnover_cellular: None,
            allow_vpnroaming_over_cellular: None,
            diable_printing_over_http: None,
            disable_downloading_of_print_drivers_over_http: None,
            disable_internet_download_for_web_publishing_and_online_ordering_wizards: None,
            disallow_network_connectivity_active_tests: None,
            hardened_uncpaths: None,
            instance_id: None,
            parent_id: None,
            prohibit_installation_and_configuration_of_network_bridge: None,
        }
    }


    /// Sets the value of AllowBluetooth
    pub fn set_allow_bluetooth(&mut self, value: i32) {
        self.allow_bluetooth = Some(value);
    }

    /// Gets the value of AllowBluetooth
    pub fn get_allow_bluetooth(&self) -> Option<&i32> {
        self.allow_bluetooth.as_ref()
    }

    /// Sets the value of AllowCellularData
    pub fn set_allow_cellular_data(&mut self, value: i32) {
        self.allow_cellular_data = Some(value);
    }

    /// Gets the value of AllowCellularData
    pub fn get_allow_cellular_data(&self) -> Option<&i32> {
        self.allow_cellular_data.as_ref()
    }

    /// Sets the value of AllowCellularDataRoaming
    pub fn set_allow_cellular_data_roaming(&mut self, value: i32) {
        self.allow_cellular_data_roaming = Some(value);
    }

    /// Gets the value of AllowCellularDataRoaming
    pub fn get_allow_cellular_data_roaming(&self) -> Option<&i32> {
        self.allow_cellular_data_roaming.as_ref()
    }

    /// Sets the value of AllowConnectedDevices
    pub fn set_allow_connected_devices(&mut self, value: i32) {
        self.allow_connected_devices = Some(value);
    }

    /// Gets the value of AllowConnectedDevices
    pub fn get_allow_connected_devices(&self) -> Option<&i32> {
        self.allow_connected_devices.as_ref()
    }

    /// Sets the value of AllowPhonePCLinking
    pub fn set_allow_phone_pclinking(&mut self, value: i32) {
        self.allow_phone_pclinking = Some(value);
    }

    /// Gets the value of AllowPhonePCLinking
    pub fn get_allow_phone_pclinking(&self) -> Option<&i32> {
        self.allow_phone_pclinking.as_ref()
    }

    /// Sets the value of AllowVPNOverCellular
    pub fn set_allow_vpnover_cellular(&mut self, value: i32) {
        self.allow_vpnover_cellular = Some(value);
    }

    /// Gets the value of AllowVPNOverCellular
    pub fn get_allow_vpnover_cellular(&self) -> Option<&i32> {
        self.allow_vpnover_cellular.as_ref()
    }

    /// Sets the value of AllowVPNRoamingOverCellular
    pub fn set_allow_vpnroaming_over_cellular(&mut self, value: i32) {
        self.allow_vpnroaming_over_cellular = Some(value);
    }

    /// Gets the value of AllowVPNRoamingOverCellular
    pub fn get_allow_vpnroaming_over_cellular(&self) -> Option<&i32> {
        self.allow_vpnroaming_over_cellular.as_ref()
    }

    /// Sets the value of DiablePrintingOverHTTP
    pub fn set_diable_printing_over_http(&mut self, value: String) {
        self.diable_printing_over_http = Some(value);
    }

    /// Gets the value of DiablePrintingOverHTTP
    pub fn get_diable_printing_over_http(&self) -> Option<&String> {
        self.diable_printing_over_http.as_ref()
    }

    /// Sets the value of DisableDownloadingOfPrintDriversOverHTTP
    pub fn set_disable_downloading_of_print_drivers_over_http(&mut self, value: String) {
        self.disable_downloading_of_print_drivers_over_http = Some(value);
    }

    /// Gets the value of DisableDownloadingOfPrintDriversOverHTTP
    pub fn get_disable_downloading_of_print_drivers_over_http(&self) -> Option<&String> {
        self.disable_downloading_of_print_drivers_over_http.as_ref()
    }

    /// Sets the value of DisableInternetDownloadForWebPublishingAndOnlineOrderingWizards
    pub fn set_disable_internet_download_for_web_publishing_and_online_ordering_wizards(&mut self, value: String) {
        self.disable_internet_download_for_web_publishing_and_online_ordering_wizards = Some(value);
    }

    /// Gets the value of DisableInternetDownloadForWebPublishingAndOnlineOrderingWizards
    pub fn get_disable_internet_download_for_web_publishing_and_online_ordering_wizards(&self) -> Option<&String> {
        self.disable_internet_download_for_web_publishing_and_online_ordering_wizards.as_ref()
    }

    /// Sets the value of DisallowNetworkConnectivityActiveTests
    pub fn set_disallow_network_connectivity_active_tests(&mut self, value: i32) {
        self.disallow_network_connectivity_active_tests = Some(value);
    }

    /// Gets the value of DisallowNetworkConnectivityActiveTests
    pub fn get_disallow_network_connectivity_active_tests(&self) -> Option<&i32> {
        self.disallow_network_connectivity_active_tests.as_ref()
    }

    /// Sets the value of HardenedUNCPaths
    pub fn set_hardened_uncpaths(&mut self, value: String) {
        self.hardened_uncpaths = Some(value);
    }

    /// Gets the value of HardenedUNCPaths
    pub fn get_hardened_uncpaths(&self) -> Option<&String> {
        self.hardened_uncpaths.as_ref()
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

    /// Sets the value of ProhibitInstallationAndConfigurationOfNetworkBridge
    pub fn set_prohibit_installation_and_configuration_of_network_bridge(&mut self, value: String) {
        self.prohibit_installation_and_configuration_of_network_bridge = Some(value);
    }

    /// Gets the value of ProhibitInstallationAndConfigurationOfNetworkBridge
    pub fn get_prohibit_installation_and_configuration_of_network_bridge(&self) -> Option<&String> {
        self.prohibit_installation_and_configuration_of_network_bridge.as_ref()
    }
}


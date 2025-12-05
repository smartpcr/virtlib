// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ThirdPartyVpnConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThirdPartyVpnConnection {
    #[serde(flatten)]
    pub base: VpnCommonConfig,

/// 
    #[serde(rename = "CustomConfiguration")]
    pub custom_configuration: Option<String>,

/// 
    #[serde(rename = "PlugInApplicationID")]
    pub plug_in_application_id: Option<String>,

/// 
    #[serde(rename = "VpnConfigurationXml")]
    pub vpn_configuration_xml: Option<String>,
}

impl ThirdPartyVpnConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: VpnCommonConfig::new(),
            custom_configuration: None,
            plug_in_application_id: None,
            vpn_configuration_xml: None,
        }
    }


    /// Sets the value of CustomConfiguration
    pub fn set_custom_configuration(&mut self, value: String) {
        self.custom_configuration = Some(value);
    }

    /// Gets the value of CustomConfiguration
    pub fn get_custom_configuration(&self) -> Option<&String> {
        self.custom_configuration.as_ref()
    }

    /// Sets the value of PlugInApplicationID
    pub fn set_plug_in_application_id(&mut self, value: String) {
        self.plug_in_application_id = Some(value);
    }

    /// Gets the value of PlugInApplicationID
    pub fn get_plug_in_application_id(&self) -> Option<&String> {
        self.plug_in_application_id.as_ref()
    }

    /// Sets the value of VpnConfigurationXml
    pub fn set_vpn_configuration_xml(&mut self, value: String) {
        self.vpn_configuration_xml = Some(value);
    }

    /// Gets the value of VpnConfigurationXml
    pub fn get_vpn_configuration_xml(&self) -> Option<&String> {
        self.vpn_configuration_xml.as_ref()
    }
}


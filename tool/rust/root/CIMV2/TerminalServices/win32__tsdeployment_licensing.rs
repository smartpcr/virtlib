// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSDeploymentLicensing struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSDeploymentLicensing {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// License Servers to use
    #[serde(rename = "LicenseServers")]
    pub license_servers: Vec<String>,

///  Licensing Mode
    #[serde(rename = "LicensingType")]
    pub licensing_type: Option<TSDeploymentLicensing_LicensingType>,

/// Use deployment-wide licensing settings, as opposed to setting them per-server.  If this is set to false, all other licensing settings are ignored.
    #[serde(rename = "UseCentralLicensingSettings")]
    pub use_central_licensing_settings: Option<bool>,
}

impl Win32_TSDeploymentLicensing {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            license_servers: Vec::new(),
            licensing_type: None,
            use_central_licensing_settings: None,
        }
    }


    /// Sets the value of LicenseServers
    pub fn set_license_servers(&mut self, value: Vec<String>) {
        self.license_servers = value;
    }

    /// Gets the value of LicenseServers
    pub fn get_license_servers(&self) -> &Vec<String> {
        &self.license_servers
    }

    /// Sets the value of LicensingType
    pub fn set_licensing_type(&mut self, value: TSDeploymentLicensing_LicensingType) {
        self.licensing_type = Some(value);
    }

    /// Gets the value of LicensingType
    pub fn get_licensing_type(&self) -> Option<&TSDeploymentLicensing_LicensingType> {
        self.licensing_type.as_ref()
    }

    /// Sets the value of UseCentralLicensingSettings
    pub fn set_use_central_licensing_settings(&mut self, value: bool) {
        self.use_central_licensing_settings = Some(value);
    }

    /// Gets the value of UseCentralLicensingSettings
    pub fn get_use_central_licensing_settings(&self) -> Option<&bool> {
        self.use_central_licensing_settings.as_ref()
    }
}


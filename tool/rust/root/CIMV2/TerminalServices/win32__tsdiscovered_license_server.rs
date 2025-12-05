// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSDiscoveredLicenseServer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSDiscoveredLicenseServer {

/// 
    #[serde(rename = "HowDiscovered")]
    pub how_discovered: Option<u32>,

/// 
    #[serde(rename = "IsAdminOnLS")]
    pub is_admin_on_ls: Option<u32>,

/// 
    #[serde(rename = "IsLSAvailable")]
    pub is_lsavailable: Option<u32>,

/// 
    #[serde(rename = "IssuingCALs")]
    pub issuing_cals: Option<u32>,

/// 
    #[serde(rename = "LicenseServer")]
    pub license_server: Option<String>,
}

impl Win32_TSDiscoveredLicenseServer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            how_discovered: None,
            is_admin_on_ls: None,
            is_lsavailable: None,
            issuing_cals: None,
            license_server: None,
        }
    }


    /// Sets the value of HowDiscovered
    pub fn set_how_discovered(&mut self, value: u32) {
        self.how_discovered = Some(value);
    }

    /// Gets the value of HowDiscovered
    pub fn get_how_discovered(&self) -> Option<&u32> {
        self.how_discovered.as_ref()
    }

    /// Sets the value of IsAdminOnLS
    pub fn set_is_admin_on_ls(&mut self, value: u32) {
        self.is_admin_on_ls = Some(value);
    }

    /// Gets the value of IsAdminOnLS
    pub fn get_is_admin_on_ls(&self) -> Option<&u32> {
        self.is_admin_on_ls.as_ref()
    }

    /// Sets the value of IsLSAvailable
    pub fn set_is_lsavailable(&mut self, value: u32) {
        self.is_lsavailable = Some(value);
    }

    /// Gets the value of IsLSAvailable
    pub fn get_is_lsavailable(&self) -> Option<&u32> {
        self.is_lsavailable.as_ref()
    }

    /// Sets the value of IssuingCALs
    pub fn set_issuing_cals(&mut self, value: u32) {
        self.issuing_cals = Some(value);
    }

    /// Gets the value of IssuingCALs
    pub fn get_issuing_cals(&self) -> Option<&u32> {
        self.issuing_cals.as_ref()
    }

    /// Sets the value of LicenseServer
    pub fn set_license_server(&mut self, value: String) {
        self.license_server = Some(value);
    }

    /// Gets the value of LicenseServer
    pub fn get_license_server(&self) -> Option<&String> {
        self.license_server.as_ref()
    }
}


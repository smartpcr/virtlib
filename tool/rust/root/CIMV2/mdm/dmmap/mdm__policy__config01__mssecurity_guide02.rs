// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_MSSecurityGuide02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_MSSecurityGuide02 {

/// 
    #[serde(rename = "ApplyUACRestrictionsToLocalAccountsOnNetworkLogon")]
    pub apply_uacrestrictions_to_local_accounts_on_network_logon: Option<String>,

/// 
    #[serde(rename = "ConfigureSMBV1ClientDriver")]
    pub configure_smbv1_client_driver: Option<String>,

/// 
    #[serde(rename = "ConfigureSMBV1Server")]
    pub configure_smbv1_server: Option<String>,

/// 
    #[serde(rename = "EnableStructuredExceptionHandlingOverwriteProtection")]
    pub enable_structured_exception_handling_overwrite_protection: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "NetBTNodeTypeConfiguration")]
    pub net_btnode_type_configuration: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "TurnOnWindowsDefenderProtectionAgainstPotentiallyUnwantedApplications")]
    pub turn_on_windows_defender_protection_against_potentially_unwanted_applications: Option<String>,

/// 
    #[serde(rename = "WDigestAuthentication")]
    pub wdigest_authentication: Option<String>,
}

impl MDM_Policy_Config01_MSSecurityGuide02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            apply_uacrestrictions_to_local_accounts_on_network_logon: None,
            configure_smbv1_client_driver: None,
            configure_smbv1_server: None,
            enable_structured_exception_handling_overwrite_protection: None,
            instance_id: None,
            net_btnode_type_configuration: None,
            parent_id: None,
            turn_on_windows_defender_protection_against_potentially_unwanted_applications: None,
            wdigest_authentication: None,
        }
    }


    /// Sets the value of ApplyUACRestrictionsToLocalAccountsOnNetworkLogon
    pub fn set_apply_uacrestrictions_to_local_accounts_on_network_logon(&mut self, value: String) {
        self.apply_uacrestrictions_to_local_accounts_on_network_logon = Some(value);
    }

    /// Gets the value of ApplyUACRestrictionsToLocalAccountsOnNetworkLogon
    pub fn get_apply_uacrestrictions_to_local_accounts_on_network_logon(&self) -> Option<&String> {
        self.apply_uacrestrictions_to_local_accounts_on_network_logon.as_ref()
    }

    /// Sets the value of ConfigureSMBV1ClientDriver
    pub fn set_configure_smbv1_client_driver(&mut self, value: String) {
        self.configure_smbv1_client_driver = Some(value);
    }

    /// Gets the value of ConfigureSMBV1ClientDriver
    pub fn get_configure_smbv1_client_driver(&self) -> Option<&String> {
        self.configure_smbv1_client_driver.as_ref()
    }

    /// Sets the value of ConfigureSMBV1Server
    pub fn set_configure_smbv1_server(&mut self, value: String) {
        self.configure_smbv1_server = Some(value);
    }

    /// Gets the value of ConfigureSMBV1Server
    pub fn get_configure_smbv1_server(&self) -> Option<&String> {
        self.configure_smbv1_server.as_ref()
    }

    /// Sets the value of EnableStructuredExceptionHandlingOverwriteProtection
    pub fn set_enable_structured_exception_handling_overwrite_protection(&mut self, value: String) {
        self.enable_structured_exception_handling_overwrite_protection = Some(value);
    }

    /// Gets the value of EnableStructuredExceptionHandlingOverwriteProtection
    pub fn get_enable_structured_exception_handling_overwrite_protection(&self) -> Option<&String> {
        self.enable_structured_exception_handling_overwrite_protection.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of NetBTNodeTypeConfiguration
    pub fn set_net_btnode_type_configuration(&mut self, value: String) {
        self.net_btnode_type_configuration = Some(value);
    }

    /// Gets the value of NetBTNodeTypeConfiguration
    pub fn get_net_btnode_type_configuration(&self) -> Option<&String> {
        self.net_btnode_type_configuration.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of TurnOnWindowsDefenderProtectionAgainstPotentiallyUnwantedApplications
    pub fn set_turn_on_windows_defender_protection_against_potentially_unwanted_applications(&mut self, value: String) {
        self.turn_on_windows_defender_protection_against_potentially_unwanted_applications = Some(value);
    }

    /// Gets the value of TurnOnWindowsDefenderProtectionAgainstPotentiallyUnwantedApplications
    pub fn get_turn_on_windows_defender_protection_against_potentially_unwanted_applications(&self) -> Option<&String> {
        self.turn_on_windows_defender_protection_against_potentially_unwanted_applications.as_ref()
    }

    /// Sets the value of WDigestAuthentication
    pub fn set_wdigest_authentication(&mut self, value: String) {
        self.wdigest_authentication = Some(value);
    }

    /// Gets the value of WDigestAuthentication
    pub fn get_wdigest_authentication(&self) -> Option<&String> {
        self.wdigest_authentication.as_ref()
    }
}


// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_MgmtAuthority struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_MgmtAuthority {

/// 
    #[serde(rename = "AuthorityName")]
    pub authority_name: Option<String>,

/// 
    #[serde(rename = "ClientSearchCriteria")]
    pub client_search_criteria: Option<String>,

/// 
    #[serde(rename = "ProvisionedCertThumbprint")]
    pub provisioned_cert_thumbprint: Option<String>,

/// 
    #[serde(rename = "RootThumbprint")]
    pub root_thumbprint: Option<String>,

/// 
    #[serde(rename = "ServerList")]
    pub server_list: Option<String>,
}

impl MDM_MgmtAuthority {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            authority_name: None,
            client_search_criteria: None,
            provisioned_cert_thumbprint: None,
            root_thumbprint: None,
            server_list: None,
        }
    }


    /// Sets the value of AuthorityName
    pub fn set_authority_name(&mut self, value: String) {
        self.authority_name = Some(value);
    }

    /// Gets the value of AuthorityName
    pub fn get_authority_name(&self) -> Option<&String> {
        self.authority_name.as_ref()
    }

    /// Sets the value of ClientSearchCriteria
    pub fn set_client_search_criteria(&mut self, value: String) {
        self.client_search_criteria = Some(value);
    }

    /// Gets the value of ClientSearchCriteria
    pub fn get_client_search_criteria(&self) -> Option<&String> {
        self.client_search_criteria.as_ref()
    }

    /// Sets the value of ProvisionedCertThumbprint
    pub fn set_provisioned_cert_thumbprint(&mut self, value: String) {
        self.provisioned_cert_thumbprint = Some(value);
    }

    /// Gets the value of ProvisionedCertThumbprint
    pub fn get_provisioned_cert_thumbprint(&self) -> Option<&String> {
        self.provisioned_cert_thumbprint.as_ref()
    }

    /// Sets the value of RootThumbprint
    pub fn set_root_thumbprint(&mut self, value: String) {
        self.root_thumbprint = Some(value);
    }

    /// Gets the value of RootThumbprint
    pub fn get_root_thumbprint(&self) -> Option<&String> {
        self.root_thumbprint.as_ref()
    }

    /// Sets the value of ServerList
    pub fn set_server_list(&mut self, value: String) {
        self.server_list = Some(value);
    }

    /// Gets the value of ServerList
    pub fn get_server_list(&self) -> Option<&String> {
        self.server_list.as_ref()
    }

/// 

    /// * `authority_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn create_new_authority(&self, authority_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AuthorityName".to_string(), value: authority_name.into() });
        self.invoke_method("CreateNewAuthority", &args)

    }

}


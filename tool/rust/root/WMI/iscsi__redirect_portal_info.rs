// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ISCSI_RedirectPortalInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ISCSI_RedirectPortalInfo {

/// Original Target IP Address given in the login
    #[serde(rename = "OriginalIPAddr")]
    pub original_ipaddr: Option<ISCSI_IP_Address>,

/// Original Target portal's socket number given in the login
    #[serde(rename = "OriginalPort")]
    pub original_port: Option<u32>,

/// TRUE if login was redirected. RedirectedIPAddr and RedirectedPort are valid then.
    #[serde(rename = "Redirected")]
    pub redirected: Option<u8>,

/// Redirected Target IP Address
    #[serde(rename = "RedirectedIPAddr")]
    pub redirected_ipaddr: Option<ISCSI_IP_Address>,

/// Redirected Target portal's socket number
    #[serde(rename = "RedirectedPort")]
    pub redirected_port: Option<u32>,

/// TRUE if the redirection is temporary. FALSE otherwise
    #[serde(rename = "TemporaryRedirect")]
    pub temporary_redirect: Option<u8>,

/// A uniquely generated connection ID. Do not confuse this with CID.
    #[serde(rename = "UniqueConnectionId")]
    pub unique_connection_id: Option<u64>,
}

impl ISCSI_RedirectPortalInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            original_ipaddr: None,
            original_port: None,
            redirected: None,
            redirected_ipaddr: None,
            redirected_port: None,
            temporary_redirect: None,
            unique_connection_id: None,
        }
    }


    /// Sets the value of OriginalIPAddr
    pub fn set_original_ipaddr(&mut self, value: ISCSI_IP_Address) {
        self.original_ipaddr = Some(value);
    }

    /// Gets the value of OriginalIPAddr
    pub fn get_original_ipaddr(&self) -> Option<&ISCSI_IP_Address> {
        self.original_ipaddr.as_ref()
    }

    /// Sets the value of OriginalPort
    pub fn set_original_port(&mut self, value: u32) {
        self.original_port = Some(value);
    }

    /// Gets the value of OriginalPort
    pub fn get_original_port(&self) -> Option<&u32> {
        self.original_port.as_ref()
    }

    /// Sets the value of Redirected
    pub fn set_redirected(&mut self, value: u8) {
        self.redirected = Some(value);
    }

    /// Gets the value of Redirected
    pub fn get_redirected(&self) -> Option<&u8> {
        self.redirected.as_ref()
    }

    /// Sets the value of RedirectedIPAddr
    pub fn set_redirected_ipaddr(&mut self, value: ISCSI_IP_Address) {
        self.redirected_ipaddr = Some(value);
    }

    /// Gets the value of RedirectedIPAddr
    pub fn get_redirected_ipaddr(&self) -> Option<&ISCSI_IP_Address> {
        self.redirected_ipaddr.as_ref()
    }

    /// Sets the value of RedirectedPort
    pub fn set_redirected_port(&mut self, value: u32) {
        self.redirected_port = Some(value);
    }

    /// Gets the value of RedirectedPort
    pub fn get_redirected_port(&self) -> Option<&u32> {
        self.redirected_port.as_ref()
    }

    /// Sets the value of TemporaryRedirect
    pub fn set_temporary_redirect(&mut self, value: u8) {
        self.temporary_redirect = Some(value);
    }

    /// Gets the value of TemporaryRedirect
    pub fn get_temporary_redirect(&self) -> Option<&u8> {
        self.temporary_redirect.as_ref()
    }

    /// Sets the value of UniqueConnectionId
    pub fn set_unique_connection_id(&mut self, value: u64) {
        self.unique_connection_id = Some(value);
    }

    /// Gets the value of UniqueConnectionId
    pub fn get_unique_connection_id(&self) -> Option<&u64> {
        self.unique_connection_id.as_ref()
    }
}


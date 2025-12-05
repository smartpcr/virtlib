// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.msdtc
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DtcInstance struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DtcInstance {

/// 
    #[serde(rename = "DtcName")]
    pub dtc_name: Option<String>,

/// 
    #[serde(rename = "KtmRmEndpointCid")]
    pub ktm_rm_endpoint_cid: Option<String>,

/// 
    #[serde(rename = "OleTxEndpointCid")]
    pub ole_tx_endpoint_cid: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<String>,

/// 
    #[serde(rename = "UisEndpointCid")]
    pub uis_endpoint_cid: Option<String>,

/// 
    #[serde(rename = "VirtualServerName")]
    pub virtual_server_name: Option<String>,

/// 
    #[serde(rename = "XAEndpointCid")]
    pub xaendpoint_cid: Option<String>,
}

impl DtcInstance {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            dtc_name: None,
            ktm_rm_endpoint_cid: None,
            ole_tx_endpoint_cid: None,
            status: None,
            uis_endpoint_cid: None,
            virtual_server_name: None,
            xaendpoint_cid: None,
        }
    }


    /// Sets the value of DtcName
    pub fn set_dtc_name(&mut self, value: String) {
        self.dtc_name = Some(value);
    }

    /// Gets the value of DtcName
    pub fn get_dtc_name(&self) -> Option<&String> {
        self.dtc_name.as_ref()
    }

    /// Sets the value of KtmRmEndpointCid
    pub fn set_ktm_rm_endpoint_cid(&mut self, value: String) {
        self.ktm_rm_endpoint_cid = Some(value);
    }

    /// Gets the value of KtmRmEndpointCid
    pub fn get_ktm_rm_endpoint_cid(&self) -> Option<&String> {
        self.ktm_rm_endpoint_cid.as_ref()
    }

    /// Sets the value of OleTxEndpointCid
    pub fn set_ole_tx_endpoint_cid(&mut self, value: String) {
        self.ole_tx_endpoint_cid = Some(value);
    }

    /// Gets the value of OleTxEndpointCid
    pub fn get_ole_tx_endpoint_cid(&self) -> Option<&String> {
        self.ole_tx_endpoint_cid.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: String) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&String> {
        self.status.as_ref()
    }

    /// Sets the value of UisEndpointCid
    pub fn set_uis_endpoint_cid(&mut self, value: String) {
        self.uis_endpoint_cid = Some(value);
    }

    /// Gets the value of UisEndpointCid
    pub fn get_uis_endpoint_cid(&self) -> Option<&String> {
        self.uis_endpoint_cid.as_ref()
    }

    /// Sets the value of VirtualServerName
    pub fn set_virtual_server_name(&mut self, value: String) {
        self.virtual_server_name = Some(value);
    }

    /// Gets the value of VirtualServerName
    pub fn get_virtual_server_name(&self) -> Option<&String> {
        self.virtual_server_name.as_ref()
    }

    /// Sets the value of XAEndpointCid
    pub fn set_xaendpoint_cid(&mut self, value: String) {
        self.xaendpoint_cid = Some(value);
    }

    /// Gets the value of XAEndpointCid
    pub fn get_xaendpoint_cid(&self) -> Option<&String> {
        self.xaendpoint_cid.as_ref()
    }
}


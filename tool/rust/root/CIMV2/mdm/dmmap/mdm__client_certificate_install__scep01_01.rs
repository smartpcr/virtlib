// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_ClientCertificateInstall_SCEP01_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_ClientCertificateInstall_SCEP01_01 {

/// 
    #[serde(rename = "CertThumbprint")]
    pub cert_thumbprint: Option<String>,

/// 
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RespondentServerUrl")]
    pub respondent_server_url: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<i32>,
}

impl MDM_ClientCertificateInstall_SCEP01_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cert_thumbprint: None,
            error_code: None,
            instance_id: None,
            parent_id: None,
            respondent_server_url: None,
            status: None,
        }
    }


    /// Sets the value of CertThumbprint
    pub fn set_cert_thumbprint(&mut self, value: String) {
        self.cert_thumbprint = Some(value);
    }

    /// Gets the value of CertThumbprint
    pub fn get_cert_thumbprint(&self) -> Option<&String> {
        self.cert_thumbprint.as_ref()
    }

    /// Sets the value of ErrorCode
    pub fn set_error_code(&mut self, value: i32) {
        self.error_code = Some(value);
    }

    /// Gets the value of ErrorCode
    pub fn get_error_code(&self) -> Option<&i32> {
        self.error_code.as_ref()
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

    /// Sets the value of RespondentServerUrl
    pub fn set_respondent_server_url(&mut self, value: String) {
        self.respondent_server_url = Some(value);
    }

    /// Gets the value of RespondentServerUrl
    pub fn get_respondent_server_url(&self) -> Option<&String> {
        self.respondent_server_url.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: i32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&i32> {
        self.status.as_ref()
    }
}


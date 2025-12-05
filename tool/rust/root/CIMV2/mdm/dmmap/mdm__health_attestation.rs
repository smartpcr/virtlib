// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_HealthAttestation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_HealthAttestation {

/// 
    #[serde(rename = "Certificate")]
    pub certificate: Option<String>,

/// 
    #[serde(rename = "CorrelationID")]
    pub correlation_id: Option<String>,

/// 
    #[serde(rename = "CurrentProtocolVersion")]
    pub current_protocol_version: Option<i32>,

/// 
    #[serde(rename = "ForceRetrieve")]
    pub force_retrieve: Option<bool>,

/// 
    #[serde(rename = "HASEndpoint")]
    pub hasendpoint: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "MaxSupportedProtocolVersion")]
    pub max_supported_protocol_version: Option<i32>,

/// 
    #[serde(rename = "Nonce")]
    pub nonce: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PreferredMaxProtocolVersion")]
    pub preferred_max_protocol_version: Option<i32>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<i32>,

/// 
    #[serde(rename = "TpmReadyStatus")]
    pub tpm_ready_status: Option<i32>,
}

impl MDM_HealthAttestation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            certificate: None,
            correlation_id: None,
            current_protocol_version: None,
            force_retrieve: None,
            hasendpoint: None,
            instance_id: None,
            max_supported_protocol_version: None,
            nonce: None,
            parent_id: None,
            preferred_max_protocol_version: None,
            status: None,
            tpm_ready_status: None,
        }
    }


    /// Sets the value of Certificate
    pub fn set_certificate(&mut self, value: String) {
        self.certificate = Some(value);
    }

    /// Gets the value of Certificate
    pub fn get_certificate(&self) -> Option<&String> {
        self.certificate.as_ref()
    }

    /// Sets the value of CorrelationID
    pub fn set_correlation_id(&mut self, value: String) {
        self.correlation_id = Some(value);
    }

    /// Gets the value of CorrelationID
    pub fn get_correlation_id(&self) -> Option<&String> {
        self.correlation_id.as_ref()
    }

    /// Sets the value of CurrentProtocolVersion
    pub fn set_current_protocol_version(&mut self, value: i32) {
        self.current_protocol_version = Some(value);
    }

    /// Gets the value of CurrentProtocolVersion
    pub fn get_current_protocol_version(&self) -> Option<&i32> {
        self.current_protocol_version.as_ref()
    }

    /// Sets the value of ForceRetrieve
    pub fn set_force_retrieve(&mut self, value: bool) {
        self.force_retrieve = Some(value);
    }

    /// Gets the value of ForceRetrieve
    pub fn get_force_retrieve(&self) -> Option<&bool> {
        self.force_retrieve.as_ref()
    }

    /// Sets the value of HASEndpoint
    pub fn set_hasendpoint(&mut self, value: String) {
        self.hasendpoint = Some(value);
    }

    /// Gets the value of HASEndpoint
    pub fn get_hasendpoint(&self) -> Option<&String> {
        self.hasendpoint.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of MaxSupportedProtocolVersion
    pub fn set_max_supported_protocol_version(&mut self, value: i32) {
        self.max_supported_protocol_version = Some(value);
    }

    /// Gets the value of MaxSupportedProtocolVersion
    pub fn get_max_supported_protocol_version(&self) -> Option<&i32> {
        self.max_supported_protocol_version.as_ref()
    }

    /// Sets the value of Nonce
    pub fn set_nonce(&mut self, value: String) {
        self.nonce = Some(value);
    }

    /// Gets the value of Nonce
    pub fn get_nonce(&self) -> Option<&String> {
        self.nonce.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PreferredMaxProtocolVersion
    pub fn set_preferred_max_protocol_version(&mut self, value: i32) {
        self.preferred_max_protocol_version = Some(value);
    }

    /// Gets the value of PreferredMaxProtocolVersion
    pub fn get_preferred_max_protocol_version(&self) -> Option<&i32> {
        self.preferred_max_protocol_version.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: i32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&i32> {
        self.status.as_ref()
    }

    /// Sets the value of TpmReadyStatus
    pub fn set_tpm_ready_status(&mut self, value: i32) {
        self.tpm_ready_status = Some(value);
    }

    /// Gets the value of TpmReadyStatus
    pub fn get_tpm_ready_status(&self) -> Option<&i32> {
        self.tpm_ready_status.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn verify_health_method(&self) -> Result<(), WmiError> {
        self.invoke_method("VerifyHealthMethod", &[])

    }

}


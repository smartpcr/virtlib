// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Firewall_Global02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Firewall_Global02 {

/// 
    #[serde(rename = "BinaryVersionSupported")]
    pub binary_version_supported: Option<String>,

/// 
    #[serde(rename = "CRLcheck")]
    pub crlcheck: Option<i32>,

/// 
    #[serde(rename = "CurrentProfiles")]
    pub current_profiles: Option<i32>,

/// 
    #[serde(rename = "DisableStatefulFtp")]
    pub disable_stateful_ftp: Option<bool>,

/// 
    #[serde(rename = "EnableAuditMode")]
    pub enable_audit_mode: Option<bool>,

/// 
    #[serde(rename = "EnablePacketQueue")]
    pub enable_packet_queue: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "IPsecExempt")]
    pub ipsec_exempt: Option<i32>,

/// 
    #[serde(rename = "OpportunisticallyMatchAuthSetPerKM")]
    pub opportunistically_match_auth_set_per_km: Option<bool>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PolicyVersion")]
    pub policy_version: Option<String>,

/// 
    #[serde(rename = "PolicyVersionSupported")]
    pub policy_version_supported: Option<i32>,

/// 
    #[serde(rename = "PresharedKeyEncoding")]
    pub preshared_key_encoding: Option<i32>,

/// 
    #[serde(rename = "SaIdleTime")]
    pub sa_idle_time: Option<i32>,
}

impl MDM_Firewall_Global02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            binary_version_supported: None,
            crlcheck: None,
            current_profiles: None,
            disable_stateful_ftp: None,
            enable_audit_mode: None,
            enable_packet_queue: None,
            instance_id: None,
            ipsec_exempt: None,
            opportunistically_match_auth_set_per_km: None,
            parent_id: None,
            policy_version: None,
            policy_version_supported: None,
            preshared_key_encoding: None,
            sa_idle_time: None,
        }
    }


    /// Sets the value of BinaryVersionSupported
    pub fn set_binary_version_supported(&mut self, value: String) {
        self.binary_version_supported = Some(value);
    }

    /// Gets the value of BinaryVersionSupported
    pub fn get_binary_version_supported(&self) -> Option<&String> {
        self.binary_version_supported.as_ref()
    }

    /// Sets the value of CRLcheck
    pub fn set_crlcheck(&mut self, value: i32) {
        self.crlcheck = Some(value);
    }

    /// Gets the value of CRLcheck
    pub fn get_crlcheck(&self) -> Option<&i32> {
        self.crlcheck.as_ref()
    }

    /// Sets the value of CurrentProfiles
    pub fn set_current_profiles(&mut self, value: i32) {
        self.current_profiles = Some(value);
    }

    /// Gets the value of CurrentProfiles
    pub fn get_current_profiles(&self) -> Option<&i32> {
        self.current_profiles.as_ref()
    }

    /// Sets the value of DisableStatefulFtp
    pub fn set_disable_stateful_ftp(&mut self, value: bool) {
        self.disable_stateful_ftp = Some(value);
    }

    /// Gets the value of DisableStatefulFtp
    pub fn get_disable_stateful_ftp(&self) -> Option<&bool> {
        self.disable_stateful_ftp.as_ref()
    }

    /// Sets the value of EnableAuditMode
    pub fn set_enable_audit_mode(&mut self, value: bool) {
        self.enable_audit_mode = Some(value);
    }

    /// Gets the value of EnableAuditMode
    pub fn get_enable_audit_mode(&self) -> Option<&bool> {
        self.enable_audit_mode.as_ref()
    }

    /// Sets the value of EnablePacketQueue
    pub fn set_enable_packet_queue(&mut self, value: i32) {
        self.enable_packet_queue = Some(value);
    }

    /// Gets the value of EnablePacketQueue
    pub fn get_enable_packet_queue(&self) -> Option<&i32> {
        self.enable_packet_queue.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of IPsecExempt
    pub fn set_ipsec_exempt(&mut self, value: i32) {
        self.ipsec_exempt = Some(value);
    }

    /// Gets the value of IPsecExempt
    pub fn get_ipsec_exempt(&self) -> Option<&i32> {
        self.ipsec_exempt.as_ref()
    }

    /// Sets the value of OpportunisticallyMatchAuthSetPerKM
    pub fn set_opportunistically_match_auth_set_per_km(&mut self, value: bool) {
        self.opportunistically_match_auth_set_per_km = Some(value);
    }

    /// Gets the value of OpportunisticallyMatchAuthSetPerKM
    pub fn get_opportunistically_match_auth_set_per_km(&self) -> Option<&bool> {
        self.opportunistically_match_auth_set_per_km.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PolicyVersion
    pub fn set_policy_version(&mut self, value: String) {
        self.policy_version = Some(value);
    }

    /// Gets the value of PolicyVersion
    pub fn get_policy_version(&self) -> Option<&String> {
        self.policy_version.as_ref()
    }

    /// Sets the value of PolicyVersionSupported
    pub fn set_policy_version_supported(&mut self, value: i32) {
        self.policy_version_supported = Some(value);
    }

    /// Gets the value of PolicyVersionSupported
    pub fn get_policy_version_supported(&self) -> Option<&i32> {
        self.policy_version_supported.as_ref()
    }

    /// Sets the value of PresharedKeyEncoding
    pub fn set_preshared_key_encoding(&mut self, value: i32) {
        self.preshared_key_encoding = Some(value);
    }

    /// Gets the value of PresharedKeyEncoding
    pub fn get_preshared_key_encoding(&self) -> Option<&i32> {
        self.preshared_key_encoding.as_ref()
    }

    /// Sets the value of SaIdleTime
    pub fn set_sa_idle_time(&mut self, value: i32) {
        self.sa_idle_time = Some(value);
    }

    /// Gets the value of SaIdleTime
    pub fn get_sa_idle_time(&self) -> Option<&i32> {
        self.sa_idle_time.as_ref()
    }
}


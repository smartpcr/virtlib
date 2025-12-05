// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Defender
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MpThreatDetection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MpThreatDetection {
    #[serde(flatten)]
    pub base: BaseStatus,

/// 
    #[serde(rename = "ActionSuccess")]
    pub action_success: Option<bool>,

/// 
    #[serde(rename = "AdditionalActionsBitMask")]
    pub additional_actions_bit_mask: Option<u32>,

/// 
    #[serde(rename = "AMProductVersion")]
    pub amproduct_version: Option<String>,

/// 
    #[serde(rename = "CleaningActionID")]
    pub cleaning_action_id: Option<u8>,

/// 
    #[serde(rename = "CurrentThreatExecutionStatusID")]
    pub current_threat_execution_status_id: Option<u8>,

/// 
    #[serde(rename = "DetectionID")]
    pub detection_id: Option<String>,

/// 
    #[serde(rename = "DetectionSourceTypeID")]
    pub detection_source_type_id: Option<u8>,

/// 
    #[serde(rename = "DomainUser")]
    pub domain_user: Option<String>,

/// 
    #[serde(rename = "InitialDetectionTime")]
    pub initial_detection_time: Option<String>,

/// 
    #[serde(rename = "LastThreatStatusChangeTime")]
    pub last_threat_status_change_time: Option<String>,

/// 
    #[serde(rename = "ProcessName")]
    pub process_name: Option<String>,

/// 
    #[serde(rename = "RemediationTime")]
    pub remediation_time: Option<String>,

/// 
    #[serde(rename = "Resources")]
    pub resources: Vec<String>,

/// 
    #[serde(rename = "ThreatID")]
    pub threat_id: Option<i64>,

/// 
    #[serde(rename = "ThreatStatusErrorCode")]
    pub threat_status_error_code: Option<i32>,

/// 
    #[serde(rename = "ThreatStatusID")]
    pub threat_status_id: Option<u8>,
}

impl MSFT_MpThreatDetection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BaseStatus::new(),
            action_success: None,
            additional_actions_bit_mask: None,
            amproduct_version: None,
            cleaning_action_id: None,
            current_threat_execution_status_id: None,
            detection_id: None,
            detection_source_type_id: None,
            domain_user: None,
            initial_detection_time: None,
            last_threat_status_change_time: None,
            process_name: None,
            remediation_time: None,
            resources: Vec::new(),
            threat_id: None,
            threat_status_error_code: None,
            threat_status_id: None,
        }
    }


    /// Sets the value of ActionSuccess
    pub fn set_action_success(&mut self, value: bool) {
        self.action_success = Some(value);
    }

    /// Gets the value of ActionSuccess
    pub fn get_action_success(&self) -> Option<&bool> {
        self.action_success.as_ref()
    }

    /// Sets the value of AdditionalActionsBitMask
    pub fn set_additional_actions_bit_mask(&mut self, value: u32) {
        self.additional_actions_bit_mask = Some(value);
    }

    /// Gets the value of AdditionalActionsBitMask
    pub fn get_additional_actions_bit_mask(&self) -> Option<&u32> {
        self.additional_actions_bit_mask.as_ref()
    }

    /// Sets the value of AMProductVersion
    pub fn set_amproduct_version(&mut self, value: String) {
        self.amproduct_version = Some(value);
    }

    /// Gets the value of AMProductVersion
    pub fn get_amproduct_version(&self) -> Option<&String> {
        self.amproduct_version.as_ref()
    }

    /// Sets the value of CleaningActionID
    pub fn set_cleaning_action_id(&mut self, value: u8) {
        self.cleaning_action_id = Some(value);
    }

    /// Gets the value of CleaningActionID
    pub fn get_cleaning_action_id(&self) -> Option<&u8> {
        self.cleaning_action_id.as_ref()
    }

    /// Sets the value of CurrentThreatExecutionStatusID
    pub fn set_current_threat_execution_status_id(&mut self, value: u8) {
        self.current_threat_execution_status_id = Some(value);
    }

    /// Gets the value of CurrentThreatExecutionStatusID
    pub fn get_current_threat_execution_status_id(&self) -> Option<&u8> {
        self.current_threat_execution_status_id.as_ref()
    }

    /// Sets the value of DetectionID
    pub fn set_detection_id(&mut self, value: String) {
        self.detection_id = Some(value);
    }

    /// Gets the value of DetectionID
    pub fn get_detection_id(&self) -> Option<&String> {
        self.detection_id.as_ref()
    }

    /// Sets the value of DetectionSourceTypeID
    pub fn set_detection_source_type_id(&mut self, value: u8) {
        self.detection_source_type_id = Some(value);
    }

    /// Gets the value of DetectionSourceTypeID
    pub fn get_detection_source_type_id(&self) -> Option<&u8> {
        self.detection_source_type_id.as_ref()
    }

    /// Sets the value of DomainUser
    pub fn set_domain_user(&mut self, value: String) {
        self.domain_user = Some(value);
    }

    /// Gets the value of DomainUser
    pub fn get_domain_user(&self) -> Option<&String> {
        self.domain_user.as_ref()
    }

    /// Sets the value of InitialDetectionTime
    pub fn set_initial_detection_time(&mut self, value: String) {
        self.initial_detection_time = Some(value);
    }

    /// Gets the value of InitialDetectionTime
    pub fn get_initial_detection_time(&self) -> Option<&String> {
        self.initial_detection_time.as_ref()
    }

    /// Sets the value of LastThreatStatusChangeTime
    pub fn set_last_threat_status_change_time(&mut self, value: String) {
        self.last_threat_status_change_time = Some(value);
    }

    /// Gets the value of LastThreatStatusChangeTime
    pub fn get_last_threat_status_change_time(&self) -> Option<&String> {
        self.last_threat_status_change_time.as_ref()
    }

    /// Sets the value of ProcessName
    pub fn set_process_name(&mut self, value: String) {
        self.process_name = Some(value);
    }

    /// Gets the value of ProcessName
    pub fn get_process_name(&self) -> Option<&String> {
        self.process_name.as_ref()
    }

    /// Sets the value of RemediationTime
    pub fn set_remediation_time(&mut self, value: String) {
        self.remediation_time = Some(value);
    }

    /// Gets the value of RemediationTime
    pub fn get_remediation_time(&self) -> Option<&String> {
        self.remediation_time.as_ref()
    }

    /// Sets the value of Resources
    pub fn set_resources(&mut self, value: Vec<String>) {
        self.resources = value;
    }

    /// Gets the value of Resources
    pub fn get_resources(&self) -> &Vec<String> {
        &self.resources
    }

    /// Sets the value of ThreatID
    pub fn set_threat_id(&mut self, value: i64) {
        self.threat_id = Some(value);
    }

    /// Gets the value of ThreatID
    pub fn get_threat_id(&self) -> Option<&i64> {
        self.threat_id.as_ref()
    }

    /// Sets the value of ThreatStatusErrorCode
    pub fn set_threat_status_error_code(&mut self, value: i32) {
        self.threat_status_error_code = Some(value);
    }

    /// Gets the value of ThreatStatusErrorCode
    pub fn get_threat_status_error_code(&self) -> Option<&i32> {
        self.threat_status_error_code.as_ref()
    }

    /// Sets the value of ThreatStatusID
    pub fn set_threat_status_id(&mut self, value: u8) {
        self.threat_status_id = Some(value);
    }

    /// Gets the value of ThreatStatusID
    pub fn get_threat_status_id(&self) -> Option<&u8> {
        self.threat_status_id.as_ref()
    }
}


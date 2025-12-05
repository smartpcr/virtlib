// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Defender
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MpEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MpEvent {

/// 
    #[serde(rename = "AdditionalData")]
    pub additional_data: Option<i64>,

/// 
    #[serde(rename = "CategoryDiscriminant")]
    pub category_discriminant: Option<u32>,

/// 
    #[serde(rename = "ComputerNotificationsValue")]
    pub computer_notifications_value: Option<u32>,

/// 
    #[serde(rename = "NotificationTime")]
    pub notification_time: Option<String>,

/// 
    #[serde(rename = "ScanNotificationsValue")]
    pub scan_notifications_value: Option<u32>,

/// 
    #[serde(rename = "SignatureNotificationsValue")]
    pub signature_notifications_value: Option<u32>,

/// 
    #[serde(rename = "ThreatNotificationsValue")]
    pub threat_notifications_value: Option<u32>,
}

impl MSFT_MpEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            additional_data: None,
            category_discriminant: None,
            computer_notifications_value: None,
            notification_time: None,
            scan_notifications_value: None,
            signature_notifications_value: None,
            threat_notifications_value: None,
        }
    }


    /// Sets the value of AdditionalData
    pub fn set_additional_data(&mut self, value: i64) {
        self.additional_data = Some(value);
    }

    /// Gets the value of AdditionalData
    pub fn get_additional_data(&self) -> Option<&i64> {
        self.additional_data.as_ref()
    }

    /// Sets the value of CategoryDiscriminant
    pub fn set_category_discriminant(&mut self, value: u32) {
        self.category_discriminant = Some(value);
    }

    /// Gets the value of CategoryDiscriminant
    pub fn get_category_discriminant(&self) -> Option<&u32> {
        self.category_discriminant.as_ref()
    }

    /// Sets the value of ComputerNotificationsValue
    pub fn set_computer_notifications_value(&mut self, value: u32) {
        self.computer_notifications_value = Some(value);
    }

    /// Gets the value of ComputerNotificationsValue
    pub fn get_computer_notifications_value(&self) -> Option<&u32> {
        self.computer_notifications_value.as_ref()
    }

    /// Sets the value of NotificationTime
    pub fn set_notification_time(&mut self, value: String) {
        self.notification_time = Some(value);
    }

    /// Gets the value of NotificationTime
    pub fn get_notification_time(&self) -> Option<&String> {
        self.notification_time.as_ref()
    }

    /// Sets the value of ScanNotificationsValue
    pub fn set_scan_notifications_value(&mut self, value: u32) {
        self.scan_notifications_value = Some(value);
    }

    /// Gets the value of ScanNotificationsValue
    pub fn get_scan_notifications_value(&self) -> Option<&u32> {
        self.scan_notifications_value.as_ref()
    }

    /// Sets the value of SignatureNotificationsValue
    pub fn set_signature_notifications_value(&mut self, value: u32) {
        self.signature_notifications_value = Some(value);
    }

    /// Gets the value of SignatureNotificationsValue
    pub fn get_signature_notifications_value(&self) -> Option<&u32> {
        self.signature_notifications_value.as_ref()
    }

    /// Sets the value of ThreatNotificationsValue
    pub fn set_threat_notifications_value(&mut self, value: u32) {
        self.threat_notifications_value = Some(value);
    }

    /// Gets the value of ThreatNotificationsValue
    pub fn get_threat_notifications_value(&self) -> Option<&u32> {
        self.threat_notifications_value.as_ref()
    }
}


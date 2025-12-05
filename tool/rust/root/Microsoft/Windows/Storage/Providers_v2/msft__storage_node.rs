// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageNode struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageNode {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// This field is a string representation of the node's firmware version.
    #[serde(rename = "FirmwareVersion")]
    pub firmware_version: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// Name is a human-readable string used to identify a storage node.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// NameFormat describes the format of the Name identifier.
    #[serde(rename = "NameFormat")]
    pub name_format: Option<StorageNode_NameFormat>,

/// Indicates the current status of the node.
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Option<StorageNode_OperationalStatus>,

/// This field is an array of custom identifier for the node. If this field is set, the OtherIdentifyingInfoDescription field must also be set.
    #[serde(rename = "OtherIdentifyingInfo")]
    pub other_identifying_info: Vec<String>,

/// An array of string description of the format used in the custom identifiers defined in the OtherIdentifyingInfo field. There must be a 1:1 mapping between this array and OtherIdentifyingInfo.
    #[serde(rename = "OtherIdentifyingInfoDescription")]
    pub other_identifying_info_description: Vec<String>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,
}

impl MSFT_StorageNode {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            firmware_version: None,
            manufacturer: None,
            model: None,
            name: None,
            name_format: None,
            operational_status: None,
            other_identifying_info: Vec::new(),
            other_identifying_info_description: Vec::new(),
            serial_number: None,
        }
    }


    /// Sets the value of FirmwareVersion
    pub fn set_firmware_version(&mut self, value: String) {
        self.firmware_version = Some(value);
    }

    /// Gets the value of FirmwareVersion
    pub fn get_firmware_version(&self) -> Option<&String> {
        self.firmware_version.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of Model
    pub fn set_model(&mut self, value: String) {
        self.model = Some(value);
    }

    /// Gets the value of Model
    pub fn get_model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NameFormat
    pub fn set_name_format(&mut self, value: StorageNode_NameFormat) {
        self.name_format = Some(value);
    }

    /// Gets the value of NameFormat
    pub fn get_name_format(&self) -> Option<&StorageNode_NameFormat> {
        self.name_format.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: StorageNode_OperationalStatus) {
        self.operational_status = Some(value);
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> Option<&StorageNode_OperationalStatus> {
        self.operational_status.as_ref()
    }

    /// Sets the value of OtherIdentifyingInfo
    pub fn set_other_identifying_info(&mut self, value: Vec<String>) {
        self.other_identifying_info = value;
    }

    /// Gets the value of OtherIdentifyingInfo
    pub fn get_other_identifying_info(&self) -> &Vec<String> {
        &self.other_identifying_info
    }

    /// Sets the value of OtherIdentifyingInfoDescription
    pub fn set_other_identifying_info_description(&mut self, value: Vec<String>) {
        self.other_identifying_info_description = value;
    }

    /// Gets the value of OtherIdentifyingInfoDescription
    pub fn get_other_identifying_info_description(&self) -> &Vec<String> {
        &self.other_identifying_info_description
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }
}


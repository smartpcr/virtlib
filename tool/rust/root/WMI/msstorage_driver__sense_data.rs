// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSStorageDriver_SenseData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSStorageDriver_SenseData {

/// Additional Sense Code
    #[serde(rename = "additionalSenseCode")]
    pub additional_sense_code: Option<u8>,

/// Additional Sense Code Qualifier
    #[serde(rename = "additionalSenseCodeQualifier")]
    pub additional_sense_code_qualifier: Option<u8>,

/// Additional Sense Length
    #[serde(rename = "additionalSenseLength")]
    pub additional_sense_length: Option<u8>,

/// Command Specific Information
    #[serde(rename = "commandSpecificInformation")]
    pub command_specific_information: Vec<u8>,

/// End Of Media
    #[serde(rename = "endOfMedia")]
    pub end_of_media: Option<bool>,

/// Error Code
    #[serde(rename = "errorCode")]
    pub error_code: Option<u8>,

/// Field Replaceable Unit Code
    #[serde(rename = "fieldReplaceableUnitCode")]
    pub field_replaceable_unit_code: Option<u8>,

/// File Mark
    #[serde(rename = "fileMark")]
    pub file_mark: Option<bool>,

/// Incorrect Length
    #[serde(rename = "incorrectLength")]
    pub incorrect_length: Option<bool>,

/// Information
    #[serde(rename = "information")]
    pub information: Vec<u8>,

/// Reserved
    #[serde(rename = "reserved")]
    pub reserved: Option<bool>,

/// Segment Number
    #[serde(rename = "segmentNumber")]
    pub segment_number: Option<u8>,

/// Sense Key
    #[serde(rename = "senseKey")]
    pub sense_key: Option<u8>,

/// Sense Key Specific
    #[serde(rename = "senseKeySpecific")]
    pub sense_key_specific: Vec<u8>,

/// Valid
    #[serde(rename = "valid")]
    pub valid: Option<bool>,
}

impl MSStorageDriver_SenseData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            additional_sense_code: None,
            additional_sense_code_qualifier: None,
            additional_sense_length: None,
            command_specific_information: Vec::new(),
            end_of_media: None,
            error_code: None,
            field_replaceable_unit_code: None,
            file_mark: None,
            incorrect_length: None,
            information: Vec::new(),
            reserved: None,
            segment_number: None,
            sense_key: None,
            sense_key_specific: Vec::new(),
            valid: None,
        }
    }


    /// Sets the value of additionalSenseCode
    pub fn set_additional_sense_code(&mut self, value: u8) {
        self.additional_sense_code = Some(value);
    }

    /// Gets the value of additionalSenseCode
    pub fn get_additional_sense_code(&self) -> Option<&u8> {
        self.additional_sense_code.as_ref()
    }

    /// Sets the value of additionalSenseCodeQualifier
    pub fn set_additional_sense_code_qualifier(&mut self, value: u8) {
        self.additional_sense_code_qualifier = Some(value);
    }

    /// Gets the value of additionalSenseCodeQualifier
    pub fn get_additional_sense_code_qualifier(&self) -> Option<&u8> {
        self.additional_sense_code_qualifier.as_ref()
    }

    /// Sets the value of additionalSenseLength
    pub fn set_additional_sense_length(&mut self, value: u8) {
        self.additional_sense_length = Some(value);
    }

    /// Gets the value of additionalSenseLength
    pub fn get_additional_sense_length(&self) -> Option<&u8> {
        self.additional_sense_length.as_ref()
    }

    /// Sets the value of commandSpecificInformation
    pub fn set_command_specific_information(&mut self, value: Vec<u8>) {
        self.command_specific_information = value;
    }

    /// Gets the value of commandSpecificInformation
    pub fn get_command_specific_information(&self) -> &Vec<u8> {
        &self.command_specific_information
    }

    /// Sets the value of endOfMedia
    pub fn set_end_of_media(&mut self, value: bool) {
        self.end_of_media = Some(value);
    }

    /// Gets the value of endOfMedia
    pub fn get_end_of_media(&self) -> Option<&bool> {
        self.end_of_media.as_ref()
    }

    /// Sets the value of errorCode
    pub fn set_error_code(&mut self, value: u8) {
        self.error_code = Some(value);
    }

    /// Gets the value of errorCode
    pub fn get_error_code(&self) -> Option<&u8> {
        self.error_code.as_ref()
    }

    /// Sets the value of fieldReplaceableUnitCode
    pub fn set_field_replaceable_unit_code(&mut self, value: u8) {
        self.field_replaceable_unit_code = Some(value);
    }

    /// Gets the value of fieldReplaceableUnitCode
    pub fn get_field_replaceable_unit_code(&self) -> Option<&u8> {
        self.field_replaceable_unit_code.as_ref()
    }

    /// Sets the value of fileMark
    pub fn set_file_mark(&mut self, value: bool) {
        self.file_mark = Some(value);
    }

    /// Gets the value of fileMark
    pub fn get_file_mark(&self) -> Option<&bool> {
        self.file_mark.as_ref()
    }

    /// Sets the value of incorrectLength
    pub fn set_incorrect_length(&mut self, value: bool) {
        self.incorrect_length = Some(value);
    }

    /// Gets the value of incorrectLength
    pub fn get_incorrect_length(&self) -> Option<&bool> {
        self.incorrect_length.as_ref()
    }

    /// Sets the value of information
    pub fn set_information(&mut self, value: Vec<u8>) {
        self.information = value;
    }

    /// Gets the value of information
    pub fn get_information(&self) -> &Vec<u8> {
        &self.information
    }

    /// Sets the value of reserved
    pub fn set_reserved(&mut self, value: bool) {
        self.reserved = Some(value);
    }

    /// Gets the value of reserved
    pub fn get_reserved(&self) -> Option<&bool> {
        self.reserved.as_ref()
    }

    /// Sets the value of segmentNumber
    pub fn set_segment_number(&mut self, value: u8) {
        self.segment_number = Some(value);
    }

    /// Gets the value of segmentNumber
    pub fn get_segment_number(&self) -> Option<&u8> {
        self.segment_number.as_ref()
    }

    /// Sets the value of senseKey
    pub fn set_sense_key(&mut self, value: u8) {
        self.sense_key = Some(value);
    }

    /// Gets the value of senseKey
    pub fn get_sense_key(&self) -> Option<&u8> {
        self.sense_key.as_ref()
    }

    /// Sets the value of senseKeySpecific
    pub fn set_sense_key_specific(&mut self, value: Vec<u8>) {
        self.sense_key_specific = value;
    }

    /// Gets the value of senseKeySpecific
    pub fn get_sense_key_specific(&self) -> &Vec<u8> {
        &self.sense_key_specific
    }

    /// Sets the value of valid
    pub fn set_valid(&mut self, value: bool) {
        self.valid = Some(value);
    }

    /// Gets the value of valid
    pub fn get_valid(&self) -> Option<&bool> {
        self.valid.as_ref()
    }
}


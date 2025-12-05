// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_PassportForWork_PINComplexity03 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_PassportForWork_PINComplexity03 {

/// 
    #[serde(rename = "Digits")]
    pub digits: Option<i32>,

/// 
    #[serde(rename = "Expiration")]
    pub expiration: Option<i32>,

/// 
    #[serde(rename = "History")]
    pub history: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LowercaseLetters")]
    pub lowercase_letters: Option<i32>,

/// 
    #[serde(rename = "MaximumPINLength")]
    pub maximum_pinlength: Option<i32>,

/// 
    #[serde(rename = "MinimumPINLength")]
    pub minimum_pinlength: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "SpecialCharacters")]
    pub special_characters: Option<i32>,

/// 
    #[serde(rename = "UppercaseLetters")]
    pub uppercase_letters: Option<i32>,
}

impl MDM_PassportForWork_PINComplexity03 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            digits: None,
            expiration: None,
            history: None,
            instance_id: None,
            lowercase_letters: None,
            maximum_pinlength: None,
            minimum_pinlength: None,
            parent_id: None,
            special_characters: None,
            uppercase_letters: None,
        }
    }


    /// Sets the value of Digits
    pub fn set_digits(&mut self, value: i32) {
        self.digits = Some(value);
    }

    /// Gets the value of Digits
    pub fn get_digits(&self) -> Option<&i32> {
        self.digits.as_ref()
    }

    /// Sets the value of Expiration
    pub fn set_expiration(&mut self, value: i32) {
        self.expiration = Some(value);
    }

    /// Gets the value of Expiration
    pub fn get_expiration(&self) -> Option<&i32> {
        self.expiration.as_ref()
    }

    /// Sets the value of History
    pub fn set_history(&mut self, value: i32) {
        self.history = Some(value);
    }

    /// Gets the value of History
    pub fn get_history(&self) -> Option<&i32> {
        self.history.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LowercaseLetters
    pub fn set_lowercase_letters(&mut self, value: i32) {
        self.lowercase_letters = Some(value);
    }

    /// Gets the value of LowercaseLetters
    pub fn get_lowercase_letters(&self) -> Option<&i32> {
        self.lowercase_letters.as_ref()
    }

    /// Sets the value of MaximumPINLength
    pub fn set_maximum_pinlength(&mut self, value: i32) {
        self.maximum_pinlength = Some(value);
    }

    /// Gets the value of MaximumPINLength
    pub fn get_maximum_pinlength(&self) -> Option<&i32> {
        self.maximum_pinlength.as_ref()
    }

    /// Sets the value of MinimumPINLength
    pub fn set_minimum_pinlength(&mut self, value: i32) {
        self.minimum_pinlength = Some(value);
    }

    /// Gets the value of MinimumPINLength
    pub fn get_minimum_pinlength(&self) -> Option<&i32> {
        self.minimum_pinlength.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of SpecialCharacters
    pub fn set_special_characters(&mut self, value: i32) {
        self.special_characters = Some(value);
    }

    /// Gets the value of SpecialCharacters
    pub fn get_special_characters(&self) -> Option<&i32> {
        self.special_characters.as_ref()
    }

    /// Sets the value of UppercaseLetters
    pub fn set_uppercase_letters(&mut self, value: i32) {
        self.uppercase_letters = Some(value);
    }

    /// Gets the value of UppercaseLetters
    pub fn get_uppercase_letters(&self) -> Option<&i32> {
        self.uppercase_letters.as_ref()
    }
}


// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_DeviceStatus_CellularIdentities01_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_DeviceStatus_CellularIdentities01_01 {

/// 
    #[serde(rename = "CommercializationOperator")]
    pub commercialization_operator: Option<String>,

/// 
    #[serde(rename = "ICCID")]
    pub iccid: Option<String>,

/// 
    #[serde(rename = "IMSI")]
    pub imsi: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PhoneNumber")]
    pub phone_number: Option<String>,

/// 
    #[serde(rename = "RoamingCompliance")]
    pub roaming_compliance: Option<bool>,

/// 
    #[serde(rename = "RoamingStatus")]
    pub roaming_status: Option<bool>,
}

impl MDM_DeviceStatus_CellularIdentities01_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            commercialization_operator: None,
            iccid: None,
            imsi: None,
            instance_id: None,
            parent_id: None,
            phone_number: None,
            roaming_compliance: None,
            roaming_status: None,
        }
    }


    /// Sets the value of CommercializationOperator
    pub fn set_commercialization_operator(&mut self, value: String) {
        self.commercialization_operator = Some(value);
    }

    /// Gets the value of CommercializationOperator
    pub fn get_commercialization_operator(&self) -> Option<&String> {
        self.commercialization_operator.as_ref()
    }

    /// Sets the value of ICCID
    pub fn set_iccid(&mut self, value: String) {
        self.iccid = Some(value);
    }

    /// Gets the value of ICCID
    pub fn get_iccid(&self) -> Option<&String> {
        self.iccid.as_ref()
    }

    /// Sets the value of IMSI
    pub fn set_imsi(&mut self, value: String) {
        self.imsi = Some(value);
    }

    /// Gets the value of IMSI
    pub fn get_imsi(&self) -> Option<&String> {
        self.imsi.as_ref()
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

    /// Sets the value of PhoneNumber
    pub fn set_phone_number(&mut self, value: String) {
        self.phone_number = Some(value);
    }

    /// Gets the value of PhoneNumber
    pub fn get_phone_number(&self) -> Option<&String> {
        self.phone_number.as_ref()
    }

    /// Sets the value of RoamingCompliance
    pub fn set_roaming_compliance(&mut self, value: bool) {
        self.roaming_compliance = Some(value);
    }

    /// Gets the value of RoamingCompliance
    pub fn get_roaming_compliance(&self) -> Option<&bool> {
        self.roaming_compliance.as_ref()
    }

    /// Sets the value of RoamingStatus
    pub fn set_roaming_status(&mut self, value: bool) {
        self.roaming_status = Some(value);
    }

    /// Gets the value of RoamingStatus
    pub fn get_roaming_status(&self) -> Option<&bool> {
        self.roaming_status.as_ref()
    }
}


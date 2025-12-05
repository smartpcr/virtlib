// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WHEAErrorSourceMethods struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WHEAErrorSourceMethods {
    #[serde(flatten)]
    pub base: WHEA,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl WHEAErrorSourceMethods {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WHEA::new(),
            active: None,
            instance_name: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

/// 

    /// * `count` -  (u32)
    /// * `error_source_array` -  (u8[])
    /// * `length` -  (u32)
    /// * `status` -  (u32)
    pub fn get_all_error_sources_rtn(&self, status: &mut u32, count: &mut u32, length: &mut u32, error_source_array: &mut Vec<u8>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetAllErrorSourcesRtn", &[])?;
        let count = result.get_value("Count")?;
        let error_source_array = result.get_value("ErrorSourceArray")?;
        let length = result.get_value("Length")?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `error_source_id` -  (u32)

    /// * `error_source_info` -  (u8[])
    /// * `length` -  (u32)
    /// * `status` -  (u32)
    pub fn get_error_source_info_rtn(&self, status: &mut u32, error_source_id: Option<u32>, length: &mut Option<u32>, error_source_info: &mut Option<Vec<u8>>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = error_source_id {
            args.push(MethodParameter { name: "ErrorSourceId".to_string(), value: val.into() });
        }

        let result = self.invoke_method("GetErrorSourceInfoRtn", &args)?;
        let error_source_info = result.get_value("ErrorSourceInfo")?;
        let length = result.get_value("Length")?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `error_source_info` -  (u8[])
    /// * `length` -  (u32)

    /// * `status` -  (u32)
    pub fn set_error_source_info_rtn(&self, status: &mut u32, length: Option<u32>, error_source_info: &Option<Vec<u8>>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = length {
            args.push(MethodParameter { name: "Length".to_string(), value: val.into() });
        }
        if let Some(val) = error_source_info {
            args.push(MethodParameter { name: "ErrorSourceInfo".to_string(), value: val.into() });
        }

        let result = self.invoke_method("SetErrorSourceInfoRtn", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `error_source_id` -  (u32)

    /// * `status` -  (u32)
    pub fn enable_error_source_rtn(&self, status: &mut u32, error_source_id: Option<u32>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = error_source_id {
            args.push(MethodParameter { name: "ErrorSourceId".to_string(), value: val.into() });
        }

        let result = self.invoke_method("EnableErrorSourceRtn", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `error_source_id` -  (u32)

    /// * `status` -  (u32)
    pub fn disable_error_source_rtn(&self, status: &mut u32, error_source_id: Option<u32>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = error_source_id {
            args.push(MethodParameter { name: "ErrorSourceId".to_string(), value: val.into() });
        }

        let result = self.invoke_method("DisableErrorSourceRtn", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }

}


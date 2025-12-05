// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.InventoryLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MsftSil_ManagementTasks struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MsftSil_ManagementTasks {
}

impl MsftSil_ManagementTasks {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `certificate_thumbprint` -  (String)
    /// * `return_value` -  (u32)
    /// * `uri` -  (String)
    pub fn get_target_uri(&self, uri: &mut String, certificate_thumbprint: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("GetTargetUri", &[])?;
        let certificate_thumbprint = result.get_value("certificateThumbprint")?;
        let uri = result.get_value("uri")?;
        Ok(result.return_value)

    }


/// 

    /// * `certificate_thumbprint` -  (String)
    /// * `uri` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_target_uri(&self, uri: &String, certificate_thumbprint: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "uri".to_string(), value: uri.into() });
        args.push(MethodParameter { name: "certificateThumbprint".to_string(), value: certificate_thumbprint.into() });
        self.invoke_method("SetTargetUri", &args)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `state` -  (u8)
    pub fn get_logging_state(&self, state: &mut u8) -> Result<(), WmiError> {

        let result = self.invoke_method("GetLoggingState", &[])?;
        let state = result.get_value("state")?;
        Ok(result.return_value)

    }


/// 

    /// * `state` -  (u8)

    /// * `return_value` -  (u32)
    pub fn set_logging_state(&self, state: u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "state".to_string(), value: state.into() });
        self.invoke_method("SetLoggingState", &args)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `time` -  (String)
    pub fn get_logging_time(&self, time: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("GetLoggingTime", &[])?;
        let time = result.get_value("time")?;
        Ok(result.return_value)

    }


/// 

    /// * `time` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_logging_time(&self, time: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "time".to_string(), value: time.into() });
        self.invoke_method("SetLoggingTime", &args)

    }

}


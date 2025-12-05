// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DCOMApplicationSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DCOMApplicationSetting {
    #[serde(flatten)]
    pub base: Win32_COMSetting,

/// 
    #[serde(rename = "AppID")]
    pub app_id: Option<String>,

/// 
    #[serde(rename = "AuthenticationLevel")]
    pub authentication_level: Option<u32>,

/// 
    #[serde(rename = "CustomSurrogate")]
    pub custom_surrogate: Option<String>,

/// 
    #[serde(rename = "EnableAtStorageActivation")]
    pub enable_at_storage_activation: Option<bool>,

/// 
    #[serde(rename = "LocalService")]
    pub local_service: Option<String>,

/// 
    #[serde(rename = "RemoteServerName")]
    pub remote_server_name: Option<String>,

/// 
    #[serde(rename = "RunAsUser")]
    pub run_as_user: Option<String>,

/// 
    #[serde(rename = "ServiceParameters")]
    pub service_parameters: Option<String>,

/// 
    #[serde(rename = "UseSurrogate")]
    pub use_surrogate: Option<bool>,
}

impl Win32_DCOMApplicationSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_COMSetting::new(),
            app_id: None,
            authentication_level: None,
            custom_surrogate: None,
            enable_at_storage_activation: None,
            local_service: None,
            remote_server_name: None,
            run_as_user: None,
            service_parameters: None,
            use_surrogate: None,
        }
    }


    /// Sets the value of AppID
    pub fn set_app_id(&mut self, value: String) {
        self.app_id = Some(value);
    }

    /// Gets the value of AppID
    pub fn get_app_id(&self) -> Option<&String> {
        self.app_id.as_ref()
    }

    /// Sets the value of AuthenticationLevel
    pub fn set_authentication_level(&mut self, value: u32) {
        self.authentication_level = Some(value);
    }

    /// Gets the value of AuthenticationLevel
    pub fn get_authentication_level(&self) -> Option<&u32> {
        self.authentication_level.as_ref()
    }

    /// Sets the value of CustomSurrogate
    pub fn set_custom_surrogate(&mut self, value: String) {
        self.custom_surrogate = Some(value);
    }

    /// Gets the value of CustomSurrogate
    pub fn get_custom_surrogate(&self) -> Option<&String> {
        self.custom_surrogate.as_ref()
    }

    /// Sets the value of EnableAtStorageActivation
    pub fn set_enable_at_storage_activation(&mut self, value: bool) {
        self.enable_at_storage_activation = Some(value);
    }

    /// Gets the value of EnableAtStorageActivation
    pub fn get_enable_at_storage_activation(&self) -> Option<&bool> {
        self.enable_at_storage_activation.as_ref()
    }

    /// Sets the value of LocalService
    pub fn set_local_service(&mut self, value: String) {
        self.local_service = Some(value);
    }

    /// Gets the value of LocalService
    pub fn get_local_service(&self) -> Option<&String> {
        self.local_service.as_ref()
    }

    /// Sets the value of RemoteServerName
    pub fn set_remote_server_name(&mut self, value: String) {
        self.remote_server_name = Some(value);
    }

    /// Gets the value of RemoteServerName
    pub fn get_remote_server_name(&self) -> Option<&String> {
        self.remote_server_name.as_ref()
    }

    /// Sets the value of RunAsUser
    pub fn set_run_as_user(&mut self, value: String) {
        self.run_as_user = Some(value);
    }

    /// Gets the value of RunAsUser
    pub fn get_run_as_user(&self) -> Option<&String> {
        self.run_as_user.as_ref()
    }

    /// Sets the value of ServiceParameters
    pub fn set_service_parameters(&mut self, value: String) {
        self.service_parameters = Some(value);
    }

    /// Gets the value of ServiceParameters
    pub fn get_service_parameters(&self) -> Option<&String> {
        self.service_parameters.as_ref()
    }

    /// Sets the value of UseSurrogate
    pub fn set_use_surrogate(&mut self, value: bool) {
        self.use_surrogate = Some(value);
    }

    /// Gets the value of UseSurrogate
    pub fn get_use_surrogate(&self) -> Option<&bool> {
        self.use_surrogate.as_ref()
    }

/// 

    /// * `descriptor` -  (Win32_SecurityDescriptor)
    /// * `return_value` -  (u32)
    pub fn get_launch_security_descriptor(&self, descriptor: &mut Win32_SecurityDescriptor) -> Result<(), WmiError> {

        let result = self.invoke_method("GetLaunchSecurityDescriptor", &[])?;
        let descriptor = result.get_value("Descriptor")?;
        Ok(result.return_value)

    }


/// 

    /// * `descriptor` -  (Win32_SecurityDescriptor)

    /// * `return_value` -  (u32)
    pub fn set_launch_security_descriptor(&self, descriptor: Win32_SecurityDescriptor) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Descriptor".to_string(), value: descriptor.into() });
        self.invoke_method("SetLaunchSecurityDescriptor", &args)

    }


/// 

    /// * `descriptor` -  (Win32_SecurityDescriptor)
    /// * `return_value` -  (u32)
    pub fn get_access_security_descriptor(&self, descriptor: &mut Win32_SecurityDescriptor) -> Result<(), WmiError> {

        let result = self.invoke_method("GetAccessSecurityDescriptor", &[])?;
        let descriptor = result.get_value("Descriptor")?;
        Ok(result.return_value)

    }


/// 

    /// * `descriptor` -  (Win32_SecurityDescriptor)

    /// * `return_value` -  (u32)
    pub fn set_access_security_descriptor(&self, descriptor: Win32_SecurityDescriptor) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Descriptor".to_string(), value: descriptor.into() });
        self.invoke_method("SetAccessSecurityDescriptor", &args)

    }


/// 

    /// * `descriptor` -  (Win32_SecurityDescriptor)
    /// * `return_value` -  (u32)
    pub fn get_configuration_security_descriptor(&self, descriptor: &mut Win32_SecurityDescriptor) -> Result<(), WmiError> {

        let result = self.invoke_method("GetConfigurationSecurityDescriptor", &[])?;
        let descriptor = result.get_value("Descriptor")?;
        Ok(result.return_value)

    }


/// 

    /// * `descriptor` -  (Win32_SecurityDescriptor)

    /// * `return_value` -  (u32)
    pub fn set_configuration_security_descriptor(&self, descriptor: Win32_SecurityDescriptor) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Descriptor".to_string(), value: descriptor.into() });
        self.invoke_method("SetConfigurationSecurityDescriptor", &args)

    }

}


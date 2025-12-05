// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_BaseService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_BaseService {
    #[serde(flatten)]
    pub base: CIM_Service,

/// 
    #[serde(rename = "AcceptPause")]
    pub accept_pause: Option<bool>,

/// 
    #[serde(rename = "AcceptStop")]
    pub accept_stop: Option<bool>,

/// 
    #[serde(rename = "DesktopInteract")]
    pub desktop_interact: Option<bool>,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "ErrorControl")]
    pub error_control: Option<String>,

/// 
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<u32>,

/// 
    #[serde(rename = "PathName")]
    pub path_name: Option<String>,

/// 
    #[serde(rename = "ServiceSpecificExitCode")]
    pub service_specific_exit_code: Option<u32>,

/// 
    #[serde(rename = "ServiceType")]
    pub service_type: Option<String>,

/// 
    #[serde(rename = "StartName")]
    pub start_name: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<String>,

/// 
    #[serde(rename = "TagId")]
    pub tag_id: Option<u32>,
}

impl Win32_BaseService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
            accept_pause: None,
            accept_stop: None,
            desktop_interact: None,
            display_name: None,
            error_control: None,
            exit_code: None,
            path_name: None,
            service_specific_exit_code: None,
            service_type: None,
            start_name: None,
            state: None,
            tag_id: None,
        }
    }


    /// Sets the value of AcceptPause
    pub fn set_accept_pause(&mut self, value: bool) {
        self.accept_pause = Some(value);
    }

    /// Gets the value of AcceptPause
    pub fn get_accept_pause(&self) -> Option<&bool> {
        self.accept_pause.as_ref()
    }

    /// Sets the value of AcceptStop
    pub fn set_accept_stop(&mut self, value: bool) {
        self.accept_stop = Some(value);
    }

    /// Gets the value of AcceptStop
    pub fn get_accept_stop(&self) -> Option<&bool> {
        self.accept_stop.as_ref()
    }

    /// Sets the value of DesktopInteract
    pub fn set_desktop_interact(&mut self, value: bool) {
        self.desktop_interact = Some(value);
    }

    /// Gets the value of DesktopInteract
    pub fn get_desktop_interact(&self) -> Option<&bool> {
        self.desktop_interact.as_ref()
    }

    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of ErrorControl
    pub fn set_error_control(&mut self, value: String) {
        self.error_control = Some(value);
    }

    /// Gets the value of ErrorControl
    pub fn get_error_control(&self) -> Option<&String> {
        self.error_control.as_ref()
    }

    /// Sets the value of ExitCode
    pub fn set_exit_code(&mut self, value: u32) {
        self.exit_code = Some(value);
    }

    /// Gets the value of ExitCode
    pub fn get_exit_code(&self) -> Option<&u32> {
        self.exit_code.as_ref()
    }

    /// Sets the value of PathName
    pub fn set_path_name(&mut self, value: String) {
        self.path_name = Some(value);
    }

    /// Gets the value of PathName
    pub fn get_path_name(&self) -> Option<&String> {
        self.path_name.as_ref()
    }

    /// Sets the value of ServiceSpecificExitCode
    pub fn set_service_specific_exit_code(&mut self, value: u32) {
        self.service_specific_exit_code = Some(value);
    }

    /// Gets the value of ServiceSpecificExitCode
    pub fn get_service_specific_exit_code(&self) -> Option<&u32> {
        self.service_specific_exit_code.as_ref()
    }

    /// Sets the value of ServiceType
    pub fn set_service_type(&mut self, value: String) {
        self.service_type = Some(value);
    }

    /// Gets the value of ServiceType
    pub fn get_service_type(&self) -> Option<&String> {
        self.service_type.as_ref()
    }

    /// Sets the value of StartName
    pub fn set_start_name(&mut self, value: String) {
        self.start_name = Some(value);
    }

    /// Gets the value of StartName
    pub fn get_start_name(&self) -> Option<&String> {
        self.start_name.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: String) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&String> {
        self.state.as_ref()
    }

    /// Sets the value of TagId
    pub fn set_tag_id(&mut self, value: u32) {
        self.tag_id = Some(value);
    }

    /// Gets the value of TagId
    pub fn get_tag_id(&self) -> Option<&u32> {
        self.tag_id.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn pause_service(&self) -> Result<(), WmiError> {
        self.invoke_method("PauseService", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn resume_service(&self) -> Result<(), WmiError> {
        self.invoke_method("ResumeService", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn interrogate_service(&self) -> Result<(), WmiError> {
        self.invoke_method("InterrogateService", &[])

    }


/// 

    /// * `control_code` -  (u8)

    /// * `return_value` -  (u32)
    pub fn user_control_service(&self, control_code: u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ControlCode".to_string(), value: control_code.into() });
        self.invoke_method("UserControlService", &args)

    }


/// 

    /// * `desktop_interact` -  (bool)
    /// * `display_name` -  (String)
    /// * `error_control` -  (u8)
    /// * `load_order_group` -  (String)
    /// * `load_order_group_dependencies` -  (String[])
    /// * `name` -  (String)
    /// * `path_name` -  (String)
    /// * `service_dependencies` -  (String[])
    /// * `service_type` -  (u8)
    /// * `start_mode` -  (String)
    /// * `start_name` -  (String)
    /// * `start_password` -  (String)

    /// * `return_value` -  (u32)
    pub fn create(&self, name: &String, display_name: &String, path_name: &String, service_type: u8, error_control: u8, start_mode: &String, desktop_interact: bool, start_name: &String, start_password: &String, load_order_group: &String, load_order_group_dependencies: &Vec<String>, service_dependencies: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "DisplayName".to_string(), value: display_name.into() });
        args.push(MethodParameter { name: "PathName".to_string(), value: path_name.into() });
        args.push(MethodParameter { name: "ServiceType".to_string(), value: service_type.into() });
        args.push(MethodParameter { name: "ErrorControl".to_string(), value: error_control.into() });
        args.push(MethodParameter { name: "StartMode".to_string(), value: start_mode.into() });
        args.push(MethodParameter { name: "DesktopInteract".to_string(), value: desktop_interact.into() });
        args.push(MethodParameter { name: "StartName".to_string(), value: start_name.into() });
        args.push(MethodParameter { name: "StartPassword".to_string(), value: start_password.into() });
        args.push(MethodParameter { name: "LoadOrderGroup".to_string(), value: load_order_group.into() });
        args.push(MethodParameter { name: "LoadOrderGroupDependencies".to_string(), value: load_order_group_dependencies.into() });
        args.push(MethodParameter { name: "ServiceDependencies".to_string(), value: service_dependencies.into() });
        self.invoke_method("Create", &args)

    }


/// 

    /// * `desktop_interact` -  (bool)
    /// * `display_name` -  (String)
    /// * `error_control` -  (u8)
    /// * `load_order_group` -  (String)
    /// * `load_order_group_dependencies` -  (String[])
    /// * `path_name` -  (String)
    /// * `service_dependencies` -  (String[])
    /// * `service_type` -  (u8)
    /// * `start_mode` -  (String)
    /// * `start_name` -  (String)
    /// * `start_password` -  (String)

    /// * `return_value` -  (u32)
    pub fn change(&self, display_name: &String, path_name: &String, service_type: u8, error_control: u8, start_mode: &String, desktop_interact: bool, start_name: &String, start_password: &String, load_order_group: &String, load_order_group_dependencies: &Vec<String>, service_dependencies: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DisplayName".to_string(), value: display_name.into() });
        args.push(MethodParameter { name: "PathName".to_string(), value: path_name.into() });
        args.push(MethodParameter { name: "ServiceType".to_string(), value: service_type.into() });
        args.push(MethodParameter { name: "ErrorControl".to_string(), value: error_control.into() });
        args.push(MethodParameter { name: "StartMode".to_string(), value: start_mode.into() });
        args.push(MethodParameter { name: "DesktopInteract".to_string(), value: desktop_interact.into() });
        args.push(MethodParameter { name: "StartName".to_string(), value: start_name.into() });
        args.push(MethodParameter { name: "StartPassword".to_string(), value: start_password.into() });
        args.push(MethodParameter { name: "LoadOrderGroup".to_string(), value: load_order_group.into() });
        args.push(MethodParameter { name: "LoadOrderGroupDependencies".to_string(), value: load_order_group_dependencies.into() });
        args.push(MethodParameter { name: "ServiceDependencies".to_string(), value: service_dependencies.into() });
        self.invoke_method("Change", &args)

    }


/// 

    /// * `start_mode` -  (String)

    /// * `return_value` -  (u32)
    pub fn change_start_mode(&self, start_mode: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StartMode".to_string(), value: start_mode.into() });
        self.invoke_method("ChangeStartMode", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn delete(&self) -> Result<(), WmiError> {
        self.invoke_method("Delete", &[])

    }

}


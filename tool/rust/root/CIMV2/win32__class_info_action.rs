// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ClassInfoAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ClassInfoAction {
    #[serde(flatten)]
    pub base: CIM_Action,

/// 
    #[serde(rename = "AppID")]
    pub app_id: Option<String>,

/// 
    #[serde(rename = "Argument")]
    pub argument: Option<String>,

/// 
    #[serde(rename = "CLSID")]
    pub clsid: Option<String>,

/// 
    #[serde(rename = "Context")]
    pub context: Option<String>,

/// 
    #[serde(rename = "DefInprocHandler")]
    pub def_inproc_handler: Option<String>,

/// 
    #[serde(rename = "FileTypeMask")]
    pub file_type_mask: Option<String>,

/// 
    #[serde(rename = "Insertable")]
    pub insertable: Option<u16>,

/// 
    #[serde(rename = "ProgID")]
    pub prog_id: Option<String>,

/// 
    #[serde(rename = "RemoteName")]
    pub remote_name: Option<String>,

/// 
    #[serde(rename = "VIProgID")]
    pub viprog_id: Option<String>,
}

impl Win32_ClassInfoAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Action::new(),
            app_id: None,
            argument: None,
            clsid: None,
            context: None,
            def_inproc_handler: None,
            file_type_mask: None,
            insertable: None,
            prog_id: None,
            remote_name: None,
            viprog_id: None,
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

    /// Sets the value of Argument
    pub fn set_argument(&mut self, value: String) {
        self.argument = Some(value);
    }

    /// Gets the value of Argument
    pub fn get_argument(&self) -> Option<&String> {
        self.argument.as_ref()
    }

    /// Sets the value of CLSID
    pub fn set_clsid(&mut self, value: String) {
        self.clsid = Some(value);
    }

    /// Gets the value of CLSID
    pub fn get_clsid(&self) -> Option<&String> {
        self.clsid.as_ref()
    }

    /// Sets the value of Context
    pub fn set_context(&mut self, value: String) {
        self.context = Some(value);
    }

    /// Gets the value of Context
    pub fn get_context(&self) -> Option<&String> {
        self.context.as_ref()
    }

    /// Sets the value of DefInprocHandler
    pub fn set_def_inproc_handler(&mut self, value: String) {
        self.def_inproc_handler = Some(value);
    }

    /// Gets the value of DefInprocHandler
    pub fn get_def_inproc_handler(&self) -> Option<&String> {
        self.def_inproc_handler.as_ref()
    }

    /// Sets the value of FileTypeMask
    pub fn set_file_type_mask(&mut self, value: String) {
        self.file_type_mask = Some(value);
    }

    /// Gets the value of FileTypeMask
    pub fn get_file_type_mask(&self) -> Option<&String> {
        self.file_type_mask.as_ref()
    }

    /// Sets the value of Insertable
    pub fn set_insertable(&mut self, value: u16) {
        self.insertable = Some(value);
    }

    /// Gets the value of Insertable
    pub fn get_insertable(&self) -> Option<&u16> {
        self.insertable.as_ref()
    }

    /// Sets the value of ProgID
    pub fn set_prog_id(&mut self, value: String) {
        self.prog_id = Some(value);
    }

    /// Gets the value of ProgID
    pub fn get_prog_id(&self) -> Option<&String> {
        self.prog_id.as_ref()
    }

    /// Sets the value of RemoteName
    pub fn set_remote_name(&mut self, value: String) {
        self.remote_name = Some(value);
    }

    /// Gets the value of RemoteName
    pub fn get_remote_name(&self) -> Option<&String> {
        self.remote_name.as_ref()
    }

    /// Sets the value of VIProgID
    pub fn set_viprog_id(&mut self, value: String) {
        self.viprog_id = Some(value);
    }

    /// Gets the value of VIProgID
    pub fn get_viprog_id(&self) -> Option<&String> {
        self.viprog_id.as_ref()
    }
}


// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ClassicCOMClassSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ClassicCOMClassSetting {
    #[serde(flatten)]
    pub base: Win32_COMSetting,

/// 
    #[serde(rename = "AppID")]
    pub app_id: Option<String>,

/// 
    #[serde(rename = "AutoConvertToClsid")]
    pub auto_convert_to_clsid: Option<String>,

/// 
    #[serde(rename = "AutoTreatAsClsid")]
    pub auto_treat_as_clsid: Option<String>,

/// 
    #[serde(rename = "ComponentId")]
    pub component_id: Option<String>,

/// 
    #[serde(rename = "Control")]
    pub control: Option<bool>,

/// 
    #[serde(rename = "DefaultIcon")]
    pub default_icon: Option<String>,

/// 
    #[serde(rename = "InprocHandler")]
    pub inproc_handler: Option<String>,

/// 
    #[serde(rename = "InprocHandler32")]
    pub inproc_handler32: Option<String>,

/// 
    #[serde(rename = "InprocServer")]
    pub inproc_server: Option<String>,

/// 
    #[serde(rename = "InprocServer32")]
    pub inproc_server32: Option<String>,

/// 
    #[serde(rename = "Insertable")]
    pub insertable: Option<bool>,

/// 
    #[serde(rename = "JavaClass")]
    pub java_class: Option<bool>,

/// 
    #[serde(rename = "LocalServer")]
    pub local_server: Option<String>,

/// 
    #[serde(rename = "LocalServer32")]
    pub local_server32: Option<String>,

/// 
    #[serde(rename = "LongDisplayName")]
    pub long_display_name: Option<String>,

/// 
    #[serde(rename = "ProgId")]
    pub prog_id: Option<String>,

/// 
    #[serde(rename = "ShortDisplayName")]
    pub short_display_name: Option<String>,

/// 
    #[serde(rename = "ThreadingModel")]
    pub threading_model: Option<String>,

/// 
    #[serde(rename = "ToolBoxBitmap32")]
    pub tool_box_bitmap32: Option<String>,

/// 
    #[serde(rename = "TreatAsClsid")]
    pub treat_as_clsid: Option<String>,

/// 
    #[serde(rename = "TypeLibraryId")]
    pub type_library_id: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,

/// 
    #[serde(rename = "VersionIndependentProgId")]
    pub version_independent_prog_id: Option<String>,
}

impl Win32_ClassicCOMClassSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_COMSetting::new(),
            app_id: None,
            auto_convert_to_clsid: None,
            auto_treat_as_clsid: None,
            component_id: None,
            control: None,
            default_icon: None,
            inproc_handler: None,
            inproc_handler32: None,
            inproc_server: None,
            inproc_server32: None,
            insertable: None,
            java_class: None,
            local_server: None,
            local_server32: None,
            long_display_name: None,
            prog_id: None,
            short_display_name: None,
            threading_model: None,
            tool_box_bitmap32: None,
            treat_as_clsid: None,
            type_library_id: None,
            version: None,
            version_independent_prog_id: None,
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

    /// Sets the value of AutoConvertToClsid
    pub fn set_auto_convert_to_clsid(&mut self, value: String) {
        self.auto_convert_to_clsid = Some(value);
    }

    /// Gets the value of AutoConvertToClsid
    pub fn get_auto_convert_to_clsid(&self) -> Option<&String> {
        self.auto_convert_to_clsid.as_ref()
    }

    /// Sets the value of AutoTreatAsClsid
    pub fn set_auto_treat_as_clsid(&mut self, value: String) {
        self.auto_treat_as_clsid = Some(value);
    }

    /// Gets the value of AutoTreatAsClsid
    pub fn get_auto_treat_as_clsid(&self) -> Option<&String> {
        self.auto_treat_as_clsid.as_ref()
    }

    /// Sets the value of ComponentId
    pub fn set_component_id(&mut self, value: String) {
        self.component_id = Some(value);
    }

    /// Gets the value of ComponentId
    pub fn get_component_id(&self) -> Option<&String> {
        self.component_id.as_ref()
    }

    /// Sets the value of Control
    pub fn set_control(&mut self, value: bool) {
        self.control = Some(value);
    }

    /// Gets the value of Control
    pub fn get_control(&self) -> Option<&bool> {
        self.control.as_ref()
    }

    /// Sets the value of DefaultIcon
    pub fn set_default_icon(&mut self, value: String) {
        self.default_icon = Some(value);
    }

    /// Gets the value of DefaultIcon
    pub fn get_default_icon(&self) -> Option<&String> {
        self.default_icon.as_ref()
    }

    /// Sets the value of InprocHandler
    pub fn set_inproc_handler(&mut self, value: String) {
        self.inproc_handler = Some(value);
    }

    /// Gets the value of InprocHandler
    pub fn get_inproc_handler(&self) -> Option<&String> {
        self.inproc_handler.as_ref()
    }

    /// Sets the value of InprocHandler32
    pub fn set_inproc_handler32(&mut self, value: String) {
        self.inproc_handler32 = Some(value);
    }

    /// Gets the value of InprocHandler32
    pub fn get_inproc_handler32(&self) -> Option<&String> {
        self.inproc_handler32.as_ref()
    }

    /// Sets the value of InprocServer
    pub fn set_inproc_server(&mut self, value: String) {
        self.inproc_server = Some(value);
    }

    /// Gets the value of InprocServer
    pub fn get_inproc_server(&self) -> Option<&String> {
        self.inproc_server.as_ref()
    }

    /// Sets the value of InprocServer32
    pub fn set_inproc_server32(&mut self, value: String) {
        self.inproc_server32 = Some(value);
    }

    /// Gets the value of InprocServer32
    pub fn get_inproc_server32(&self) -> Option<&String> {
        self.inproc_server32.as_ref()
    }

    /// Sets the value of Insertable
    pub fn set_insertable(&mut self, value: bool) {
        self.insertable = Some(value);
    }

    /// Gets the value of Insertable
    pub fn get_insertable(&self) -> Option<&bool> {
        self.insertable.as_ref()
    }

    /// Sets the value of JavaClass
    pub fn set_java_class(&mut self, value: bool) {
        self.java_class = Some(value);
    }

    /// Gets the value of JavaClass
    pub fn get_java_class(&self) -> Option<&bool> {
        self.java_class.as_ref()
    }

    /// Sets the value of LocalServer
    pub fn set_local_server(&mut self, value: String) {
        self.local_server = Some(value);
    }

    /// Gets the value of LocalServer
    pub fn get_local_server(&self) -> Option<&String> {
        self.local_server.as_ref()
    }

    /// Sets the value of LocalServer32
    pub fn set_local_server32(&mut self, value: String) {
        self.local_server32 = Some(value);
    }

    /// Gets the value of LocalServer32
    pub fn get_local_server32(&self) -> Option<&String> {
        self.local_server32.as_ref()
    }

    /// Sets the value of LongDisplayName
    pub fn set_long_display_name(&mut self, value: String) {
        self.long_display_name = Some(value);
    }

    /// Gets the value of LongDisplayName
    pub fn get_long_display_name(&self) -> Option<&String> {
        self.long_display_name.as_ref()
    }

    /// Sets the value of ProgId
    pub fn set_prog_id(&mut self, value: String) {
        self.prog_id = Some(value);
    }

    /// Gets the value of ProgId
    pub fn get_prog_id(&self) -> Option<&String> {
        self.prog_id.as_ref()
    }

    /// Sets the value of ShortDisplayName
    pub fn set_short_display_name(&mut self, value: String) {
        self.short_display_name = Some(value);
    }

    /// Gets the value of ShortDisplayName
    pub fn get_short_display_name(&self) -> Option<&String> {
        self.short_display_name.as_ref()
    }

    /// Sets the value of ThreadingModel
    pub fn set_threading_model(&mut self, value: String) {
        self.threading_model = Some(value);
    }

    /// Gets the value of ThreadingModel
    pub fn get_threading_model(&self) -> Option<&String> {
        self.threading_model.as_ref()
    }

    /// Sets the value of ToolBoxBitmap32
    pub fn set_tool_box_bitmap32(&mut self, value: String) {
        self.tool_box_bitmap32 = Some(value);
    }

    /// Gets the value of ToolBoxBitmap32
    pub fn get_tool_box_bitmap32(&self) -> Option<&String> {
        self.tool_box_bitmap32.as_ref()
    }

    /// Sets the value of TreatAsClsid
    pub fn set_treat_as_clsid(&mut self, value: String) {
        self.treat_as_clsid = Some(value);
    }

    /// Gets the value of TreatAsClsid
    pub fn get_treat_as_clsid(&self) -> Option<&String> {
        self.treat_as_clsid.as_ref()
    }

    /// Sets the value of TypeLibraryId
    pub fn set_type_library_id(&mut self, value: String) {
        self.type_library_id = Some(value);
    }

    /// Gets the value of TypeLibraryId
    pub fn get_type_library_id(&self) -> Option<&String> {
        self.type_library_id.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    /// Sets the value of VersionIndependentProgId
    pub fn set_version_independent_prog_id(&mut self, value: String) {
        self.version_independent_prog_id = Some(value);
    }

    /// Gets the value of VersionIndependentProgId
    pub fn get_version_independent_prog_id(&self) -> Option<&String> {
        self.version_independent_prog_id.as_ref()
    }
}


// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// VssTracingEventClass_VssTracingEventTypeClass struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VssTracingEventClass_VssTracingEventTypeClass {
    #[serde(flatten)]
    pub base: VssTracingEventClass,

/// 
    #[serde(rename = "FunctionName")]
    pub function_name: Option<String>,

/// 
    #[serde(rename = "Indent")]
    pub indent: Option<u32>,

/// 
    #[serde(rename = "LineNumber")]
    pub line_number: Option<u32>,

/// 
    #[serde(rename = "MessageDescription")]
    pub message_description: Option<String>,

/// 
    #[serde(rename = "ModuleIndex")]
    pub module_index: Option<u32>,

/// 
    #[serde(rename = "SourceFileAlias")]
    pub source_file_alias: Option<String>,

/// 
    #[serde(rename = "SourceFileName")]
    pub source_file_name: Option<String>,

/// 
    #[serde(rename = "Text")]
    pub text: Option<String>,
}

impl VssTracingEventClass_VssTracingEventTypeClass {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: VssTracingEventClass::new(),
            function_name: None,
            indent: None,
            line_number: None,
            message_description: None,
            module_index: None,
            source_file_alias: None,
            source_file_name: None,
            text: None,
        }
    }


    /// Sets the value of FunctionName
    pub fn set_function_name(&mut self, value: String) {
        self.function_name = Some(value);
    }

    /// Gets the value of FunctionName
    pub fn get_function_name(&self) -> Option<&String> {
        self.function_name.as_ref()
    }

    /// Sets the value of Indent
    pub fn set_indent(&mut self, value: u32) {
        self.indent = Some(value);
    }

    /// Gets the value of Indent
    pub fn get_indent(&self) -> Option<&u32> {
        self.indent.as_ref()
    }

    /// Sets the value of LineNumber
    pub fn set_line_number(&mut self, value: u32) {
        self.line_number = Some(value);
    }

    /// Gets the value of LineNumber
    pub fn get_line_number(&self) -> Option<&u32> {
        self.line_number.as_ref()
    }

    /// Sets the value of MessageDescription
    pub fn set_message_description(&mut self, value: String) {
        self.message_description = Some(value);
    }

    /// Gets the value of MessageDescription
    pub fn get_message_description(&self) -> Option<&String> {
        self.message_description.as_ref()
    }

    /// Sets the value of ModuleIndex
    pub fn set_module_index(&mut self, value: u32) {
        self.module_index = Some(value);
    }

    /// Gets the value of ModuleIndex
    pub fn get_module_index(&self) -> Option<&u32> {
        self.module_index.as_ref()
    }

    /// Sets the value of SourceFileAlias
    pub fn set_source_file_alias(&mut self, value: String) {
        self.source_file_alias = Some(value);
    }

    /// Gets the value of SourceFileAlias
    pub fn get_source_file_alias(&self) -> Option<&String> {
        self.source_file_alias.as_ref()
    }

    /// Sets the value of SourceFileName
    pub fn set_source_file_name(&mut self, value: String) {
        self.source_file_name = Some(value);
    }

    /// Gets the value of SourceFileName
    pub fn get_source_file_name(&self) -> Option<&String> {
        self.source_file_name.as_ref()
    }

    /// Sets the value of Text
    pub fn set_text(&mut self, value: String) {
        self.text = Some(value);
    }

    /// Gets the value of Text
    pub fn get_text(&self) -> Option<&String> {
        self.text.as_ref()
    }
}


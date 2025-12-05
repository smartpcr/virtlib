// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_User_Result01_Education02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_User_Result01_Education02 {

/// 
    #[serde(rename = "AllowGraphingCalculator")]
    pub allow_graphing_calculator: Option<i32>,

/// 
    #[serde(rename = "DefaultPrinterName")]
    pub default_printer_name: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PreventAddingNewPrinters")]
    pub prevent_adding_new_printers: Option<i32>,

/// 
    #[serde(rename = "PrinterNames")]
    pub printer_names: Option<String>,
}

impl MDM_Policy_User_Result01_Education02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_graphing_calculator: None,
            default_printer_name: None,
            instance_id: None,
            parent_id: None,
            prevent_adding_new_printers: None,
            printer_names: None,
        }
    }


    /// Sets the value of AllowGraphingCalculator
    pub fn set_allow_graphing_calculator(&mut self, value: i32) {
        self.allow_graphing_calculator = Some(value);
    }

    /// Gets the value of AllowGraphingCalculator
    pub fn get_allow_graphing_calculator(&self) -> Option<&i32> {
        self.allow_graphing_calculator.as_ref()
    }

    /// Sets the value of DefaultPrinterName
    pub fn set_default_printer_name(&mut self, value: String) {
        self.default_printer_name = Some(value);
    }

    /// Gets the value of DefaultPrinterName
    pub fn get_default_printer_name(&self) -> Option<&String> {
        self.default_printer_name.as_ref()
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

    /// Sets the value of PreventAddingNewPrinters
    pub fn set_prevent_adding_new_printers(&mut self, value: i32) {
        self.prevent_adding_new_printers = Some(value);
    }

    /// Gets the value of PreventAddingNewPrinters
    pub fn get_prevent_adding_new_printers(&self) -> Option<&i32> {
        self.prevent_adding_new_printers.as_ref()
    }

    /// Sets the value of PrinterNames
    pub fn set_printer_names(&mut self, value: String) {
        self.printer_names = Some(value);
    }

    /// Gets the value of PrinterNames
    pub fn get_printer_names(&self) -> Option<&String> {
        self.printer_names.as_ref()
    }
}


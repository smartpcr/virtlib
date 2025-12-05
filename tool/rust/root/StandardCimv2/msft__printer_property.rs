// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_PrinterProperty struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_PrinterProperty {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "ComputerName")]
    pub computer_name: Option<String>,

/// 
    #[serde(rename = "PrinterName")]
    pub printer_name: Option<String>,

/// 
    #[serde(rename = "PropertyName")]
    pub property_name: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,

/// 
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl MSFT_PrinterProperty {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            computer_name: None,
            printer_name: None,
            property_name: None,
            type: None,
            value: None,
        }
    }


    /// Sets the value of ComputerName
    pub fn set_computer_name(&mut self, value: String) {
        self.computer_name = Some(value);
    }

    /// Gets the value of ComputerName
    pub fn get_computer_name(&self) -> Option<&String> {
        self.computer_name.as_ref()
    }

    /// Sets the value of PrinterName
    pub fn set_printer_name(&mut self, value: String) {
        self.printer_name = Some(value);
    }

    /// Gets the value of PrinterName
    pub fn get_printer_name(&self) -> Option<&String> {
        self.printer_name.as_ref()
    }

    /// Sets the value of PropertyName
    pub fn set_property_name(&mut self, value: String) {
        self.property_name = Some(value);
    }

    /// Gets the value of PropertyName
    pub fn get_property_name(&self) -> Option<&String> {
        self.property_name.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }

    /// Sets the value of Value
    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    /// Gets the value of Value
    pub fn get_value(&self) -> Option<&String> {
        self.value.as_ref()
    }

/// 

    /// * `computer_name` -  (String)
    /// * `printer_name` -  (String)
    /// * `property_name` -  (String)
    /// * `value` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_by_printer_name(&self, computer_name: &String, printer_name: &String, property_name: &String, value: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        args.push(MethodParameter { name: "PrinterName".to_string(), value: printer_name.into() });
        args.push(MethodParameter { name: "PropertyName".to_string(), value: property_name.into() });
        args.push(MethodParameter { name: "Value".to_string(), value: value.into() });
        self.invoke_method("SetByPrinterName", &args)

    }


/// 

    /// * `input_object` -  (MSFT_PrinterProperty)

    /// * `return_value` -  (u32)
    pub fn set_by_printer_property_object(&self, input_object: MSFT_PrinterProperty) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        self.invoke_method("SetByPrinterPropertyObject", &args)

    }


/// 

    /// * `printer_object` -  (MSFT_Printer)
    /// * `property_name` -  (String)
    /// * `value` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_by_printer_object(&self, printer_object: MSFT_Printer, property_name: &String, value: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PrinterObject".to_string(), value: printer_object.into() });
        args.push(MethodParameter { name: "PropertyName".to_string(), value: property_name.into() });
        args.push(MethodParameter { name: "Value".to_string(), value: value.into() });
        self.invoke_method("SetByPrinterObject", &args)

    }

}


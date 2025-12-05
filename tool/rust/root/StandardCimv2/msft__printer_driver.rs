// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_PrinterDriver struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_PrinterDriver {
    #[serde(flatten)]
    pub base: CIM_ManagedSystemElement,

/// 
    #[serde(rename = "ColorProfiles")]
    pub color_profiles: Vec<String>,

/// 
    #[serde(rename = "ComputerName")]
    pub computer_name: Option<String>,

/// 
    #[serde(rename = "ConfigFile")]
    pub config_file: Option<String>,

/// 
    #[serde(rename = "CoreDriverDependencies")]
    pub core_driver_dependencies: Vec<String>,

/// 
    #[serde(rename = "DataFile")]
    pub data_file: Option<String>,

/// 
    #[serde(rename = "Date")]
    pub date: Option<String>,

/// 
    #[serde(rename = "DefaultDatatype")]
    pub default_datatype: Option<String>,

/// 
    #[serde(rename = "DependentFiles")]
    pub dependent_files: Vec<String>,

/// 
    #[serde(rename = "DriverVersion")]
    pub driver_version: Option<u64>,

/// 
    #[serde(rename = "HardwareID")]
    pub hardware_id: Option<String>,

/// 
    #[serde(rename = "HelpFile")]
    pub help_file: Option<String>,

/// 
    #[serde(rename = "InfPath")]
    pub inf_path: Option<String>,

/// 
    #[serde(rename = "IsPackageAware")]
    pub is_package_aware: Option<bool>,

/// 
    #[serde(rename = "MajorVersion")]
    pub major_version: Option<u32>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "Monitor")]
    pub monitor: Option<String>,

/// 
    #[serde(rename = "OEMUrl")]
    pub oemurl: Option<String>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// 
    #[serde(rename = "PreviousCompatibleNames")]
    pub previous_compatible_names: Vec<String>,

/// 
    #[serde(rename = "PrinterEnvironment")]
    pub printer_environment: Option<String>,

/// 
    #[serde(rename = "PrintProcessor")]
    pub print_processor: Option<String>,

/// 
    #[serde(rename = "provider")]
    pub provider: Option<String>,

/// 
    #[serde(rename = "VendorSetup")]
    pub vendor_setup: Option<String>,
}

impl MSFT_PrinterDriver {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedSystemElement::new(),
            color_profiles: Vec::new(),
            computer_name: None,
            config_file: None,
            core_driver_dependencies: Vec::new(),
            data_file: None,
            date: None,
            default_datatype: None,
            dependent_files: Vec::new(),
            driver_version: None,
            hardware_id: None,
            help_file: None,
            inf_path: None,
            is_package_aware: None,
            major_version: None,
            manufacturer: None,
            monitor: None,
            oemurl: None,
            path: None,
            previous_compatible_names: Vec::new(),
            printer_environment: None,
            print_processor: None,
            provider: None,
            vendor_setup: None,
        }
    }


    /// Sets the value of ColorProfiles
    pub fn set_color_profiles(&mut self, value: Vec<String>) {
        self.color_profiles = value;
    }

    /// Gets the value of ColorProfiles
    pub fn get_color_profiles(&self) -> &Vec<String> {
        &self.color_profiles
    }

    /// Sets the value of ComputerName
    pub fn set_computer_name(&mut self, value: String) {
        self.computer_name = Some(value);
    }

    /// Gets the value of ComputerName
    pub fn get_computer_name(&self) -> Option<&String> {
        self.computer_name.as_ref()
    }

    /// Sets the value of ConfigFile
    pub fn set_config_file(&mut self, value: String) {
        self.config_file = Some(value);
    }

    /// Gets the value of ConfigFile
    pub fn get_config_file(&self) -> Option<&String> {
        self.config_file.as_ref()
    }

    /// Sets the value of CoreDriverDependencies
    pub fn set_core_driver_dependencies(&mut self, value: Vec<String>) {
        self.core_driver_dependencies = value;
    }

    /// Gets the value of CoreDriverDependencies
    pub fn get_core_driver_dependencies(&self) -> &Vec<String> {
        &self.core_driver_dependencies
    }

    /// Sets the value of DataFile
    pub fn set_data_file(&mut self, value: String) {
        self.data_file = Some(value);
    }

    /// Gets the value of DataFile
    pub fn get_data_file(&self) -> Option<&String> {
        self.data_file.as_ref()
    }

    /// Sets the value of Date
    pub fn set_date(&mut self, value: String) {
        self.date = Some(value);
    }

    /// Gets the value of Date
    pub fn get_date(&self) -> Option<&String> {
        self.date.as_ref()
    }

    /// Sets the value of DefaultDatatype
    pub fn set_default_datatype(&mut self, value: String) {
        self.default_datatype = Some(value);
    }

    /// Gets the value of DefaultDatatype
    pub fn get_default_datatype(&self) -> Option<&String> {
        self.default_datatype.as_ref()
    }

    /// Sets the value of DependentFiles
    pub fn set_dependent_files(&mut self, value: Vec<String>) {
        self.dependent_files = value;
    }

    /// Gets the value of DependentFiles
    pub fn get_dependent_files(&self) -> &Vec<String> {
        &self.dependent_files
    }

    /// Sets the value of DriverVersion
    pub fn set_driver_version(&mut self, value: u64) {
        self.driver_version = Some(value);
    }

    /// Gets the value of DriverVersion
    pub fn get_driver_version(&self) -> Option<&u64> {
        self.driver_version.as_ref()
    }

    /// Sets the value of HardwareID
    pub fn set_hardware_id(&mut self, value: String) {
        self.hardware_id = Some(value);
    }

    /// Gets the value of HardwareID
    pub fn get_hardware_id(&self) -> Option<&String> {
        self.hardware_id.as_ref()
    }

    /// Sets the value of HelpFile
    pub fn set_help_file(&mut self, value: String) {
        self.help_file = Some(value);
    }

    /// Gets the value of HelpFile
    pub fn get_help_file(&self) -> Option<&String> {
        self.help_file.as_ref()
    }

    /// Sets the value of InfPath
    pub fn set_inf_path(&mut self, value: String) {
        self.inf_path = Some(value);
    }

    /// Gets the value of InfPath
    pub fn get_inf_path(&self) -> Option<&String> {
        self.inf_path.as_ref()
    }

    /// Sets the value of IsPackageAware
    pub fn set_is_package_aware(&mut self, value: bool) {
        self.is_package_aware = Some(value);
    }

    /// Gets the value of IsPackageAware
    pub fn get_is_package_aware(&self) -> Option<&bool> {
        self.is_package_aware.as_ref()
    }

    /// Sets the value of MajorVersion
    pub fn set_major_version(&mut self, value: u32) {
        self.major_version = Some(value);
    }

    /// Gets the value of MajorVersion
    pub fn get_major_version(&self) -> Option<&u32> {
        self.major_version.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of Monitor
    pub fn set_monitor(&mut self, value: String) {
        self.monitor = Some(value);
    }

    /// Gets the value of Monitor
    pub fn get_monitor(&self) -> Option<&String> {
        self.monitor.as_ref()
    }

    /// Sets the value of OEMUrl
    pub fn set_oemurl(&mut self, value: String) {
        self.oemurl = Some(value);
    }

    /// Gets the value of OEMUrl
    pub fn get_oemurl(&self) -> Option<&String> {
        self.oemurl.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Sets the value of PreviousCompatibleNames
    pub fn set_previous_compatible_names(&mut self, value: Vec<String>) {
        self.previous_compatible_names = value;
    }

    /// Gets the value of PreviousCompatibleNames
    pub fn get_previous_compatible_names(&self) -> &Vec<String> {
        &self.previous_compatible_names
    }

    /// Sets the value of PrinterEnvironment
    pub fn set_printer_environment(&mut self, value: String) {
        self.printer_environment = Some(value);
    }

    /// Gets the value of PrinterEnvironment
    pub fn get_printer_environment(&self) -> Option<&String> {
        self.printer_environment.as_ref()
    }

    /// Sets the value of PrintProcessor
    pub fn set_print_processor(&mut self, value: String) {
        self.print_processor = Some(value);
    }

    /// Gets the value of PrintProcessor
    pub fn get_print_processor(&self) -> Option<&String> {
        self.print_processor.as_ref()
    }

    /// Sets the value of provider
    pub fn set_provider(&mut self, value: String) {
        self.provider = Some(value);
    }

    /// Gets the value of provider
    pub fn get_provider(&self) -> Option<&String> {
        self.provider.as_ref()
    }

    /// Sets the value of VendorSetup
    pub fn set_vendor_setup(&mut self, value: String) {
        self.vendor_setup = Some(value);
    }

    /// Gets the value of VendorSetup
    pub fn get_vendor_setup(&self) -> Option<&String> {
        self.vendor_setup.as_ref()
    }

/// 

    /// * `computer_name` -  (String)
    /// * `inf_path` -  (String)
    /// * `name` -  (String)
    /// * `printer_environment` -  (String)

    /// * `return_value` -  (u32)
    pub fn add(&self, name: &String, inf_path: &String, printer_environment: &String, computer_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "InfPath".to_string(), value: inf_path.into() });
        args.push(MethodParameter { name: "PrinterEnvironment".to_string(), value: printer_environment.into() });
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        self.invoke_method("Add", &args)

    }

}


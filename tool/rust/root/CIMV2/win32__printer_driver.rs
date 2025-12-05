// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PrinterDriver struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PrinterDriver {
    #[serde(flatten)]
    pub base: CIM_Service,

/// The ConfigFile property contains the configuration file for this printer driver, (example: pscrptui.dll).
    #[serde(rename = "ConfigFile")]
    pub config_file: Option<String>,

/// The DataFile property contains the data file for this printer driver, (example: qms810.ppd).
    #[serde(rename = "DataFile")]
    pub data_file: Option<String>,

/// The DefaultDataType property indicates the default data type for this printer driver, (example: EMF).
    #[serde(rename = "DefaultDataType")]
    pub default_data_type: Option<String>,

/// The DependentFiles property contains a list of dependent files for this printer driver.
    #[serde(rename = "DependentFiles")]
    pub dependent_files: Vec<String>,

/// The DriverPath property contains the path for this printer driver, (example: C:\drivers\pscript.dll).
    #[serde(rename = "DriverPath")]
    pub driver_path: Option<String>,

/// The FilePath property contains the path to the INF file being used, (Example: c:\temp\driver).
    #[serde(rename = "FilePath")]
    pub file_path: Option<String>,

/// The HelpFile property contains the help file for this printer driver, (example: pscrptui.hlp).
    #[serde(rename = "HelpFile")]
    pub help_file: Option<String>,

/// The InfName property contains the name of the INF file being used. The default is 'ntprint.INF'.  This will only be different if the drivers are provided directly by the manufacturer of the printer and not the operating system.
    #[serde(rename = "InfName")]
    pub inf_name: Option<String>,

/// The MonitorName property contains the name of the of the monitor for this printer driver, (example: PJL monitor).
    #[serde(rename = "MonitorName")]
    pub monitor_name: Option<String>,

/// The OEMUrl property provides a world wide web link to the printer manufacturer's web site.  Note that this property is not populated when the Win32.INF file is used and is only applicable for drivers provided directly from the manufacturer.
    #[serde(rename = "OEMUrl")]
    pub oemurl: Option<String>,

/// The SupportedPlatform property indicates the operating environments that the driver is intended for.  Examples are 'Windows NT x86' or 'Windows IA64'.
    #[serde(rename = "SupportedPlatform")]
    pub supported_platform: Option<String>,

/// The Version property indicates the operating system version that the driver is intended for.
    #[serde(rename = "Version")]
    pub version: Option<PrinterDriver_Version>,
}

impl Win32_PrinterDriver {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
            config_file: None,
            data_file: None,
            default_data_type: None,
            dependent_files: Vec::new(),
            driver_path: None,
            file_path: None,
            help_file: None,
            inf_name: None,
            monitor_name: None,
            oemurl: None,
            supported_platform: None,
            version: None,
        }
    }


    /// Sets the value of ConfigFile
    pub fn set_config_file(&mut self, value: String) {
        self.config_file = Some(value);
    }

    /// Gets the value of ConfigFile
    pub fn get_config_file(&self) -> Option<&String> {
        self.config_file.as_ref()
    }

    /// Sets the value of DataFile
    pub fn set_data_file(&mut self, value: String) {
        self.data_file = Some(value);
    }

    /// Gets the value of DataFile
    pub fn get_data_file(&self) -> Option<&String> {
        self.data_file.as_ref()
    }

    /// Sets the value of DefaultDataType
    pub fn set_default_data_type(&mut self, value: String) {
        self.default_data_type = Some(value);
    }

    /// Gets the value of DefaultDataType
    pub fn get_default_data_type(&self) -> Option<&String> {
        self.default_data_type.as_ref()
    }

    /// Sets the value of DependentFiles
    pub fn set_dependent_files(&mut self, value: Vec<String>) {
        self.dependent_files = value;
    }

    /// Gets the value of DependentFiles
    pub fn get_dependent_files(&self) -> &Vec<String> {
        &self.dependent_files
    }

    /// Sets the value of DriverPath
    pub fn set_driver_path(&mut self, value: String) {
        self.driver_path = Some(value);
    }

    /// Gets the value of DriverPath
    pub fn get_driver_path(&self) -> Option<&String> {
        self.driver_path.as_ref()
    }

    /// Sets the value of FilePath
    pub fn set_file_path(&mut self, value: String) {
        self.file_path = Some(value);
    }

    /// Gets the value of FilePath
    pub fn get_file_path(&self) -> Option<&String> {
        self.file_path.as_ref()
    }

    /// Sets the value of HelpFile
    pub fn set_help_file(&mut self, value: String) {
        self.help_file = Some(value);
    }

    /// Gets the value of HelpFile
    pub fn get_help_file(&self) -> Option<&String> {
        self.help_file.as_ref()
    }

    /// Sets the value of InfName
    pub fn set_inf_name(&mut self, value: String) {
        self.inf_name = Some(value);
    }

    /// Gets the value of InfName
    pub fn get_inf_name(&self) -> Option<&String> {
        self.inf_name.as_ref()
    }

    /// Sets the value of MonitorName
    pub fn set_monitor_name(&mut self, value: String) {
        self.monitor_name = Some(value);
    }

    /// Gets the value of MonitorName
    pub fn get_monitor_name(&self) -> Option<&String> {
        self.monitor_name.as_ref()
    }

    /// Sets the value of OEMUrl
    pub fn set_oemurl(&mut self, value: String) {
        self.oemurl = Some(value);
    }

    /// Gets the value of OEMUrl
    pub fn get_oemurl(&self) -> Option<&String> {
        self.oemurl.as_ref()
    }

    /// Sets the value of SupportedPlatform
    pub fn set_supported_platform(&mut self, value: String) {
        self.supported_platform = Some(value);
    }

    /// Gets the value of SupportedPlatform
    pub fn get_supported_platform(&self) -> Option<&String> {
        self.supported_platform.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: PrinterDriver_Version) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&PrinterDriver_Version> {
        self.version.as_ref()
    }

/// The AddPrinterDriver method installs a printer driver. The method can return the following values:
/// 0 - Success.
/// 5 - Access denied.
/// 1797 - The printer driver is unknown.
/// Other - For integer values other than those listed above, refer to the documentation on the Win32 error codes.

    /// * `driver_info` - The DriverInfo parameter specifies the neccessary inforation needed in order to create the printer driver. (Win32_PrinterDriver)

    /// * `return_value` -  (u32)
    pub fn add_printer_driver(&self, driver_info: Win32_PrinterDriver) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DriverInfo".to_string(), value: driver_info.into() });
        self.invoke_method("AddPrinterDriver", &args)

    }

}


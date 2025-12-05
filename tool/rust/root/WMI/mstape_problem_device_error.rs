// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSTapeProblemDeviceError struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSTapeProblemDeviceError {
    #[serde(flatten)]
    pub base: MSTapeDriver,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "DriveHardwareError")]
    pub drive_hardware_error: Option<bool>,

/// 
    #[serde(rename = "DriveRequiresCleaning")]
    pub drive_requires_cleaning: Option<bool>,

/// 
    #[serde(rename = "HardError")]
    pub hard_error: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "MediaLife")]
    pub media_life: Option<bool>,

/// 
    #[serde(rename = "ReadFailure")]
    pub read_failure: Option<bool>,

/// 
    #[serde(rename = "ReadWarning")]
    pub read_warning: Option<bool>,

/// 
    #[serde(rename = "ScsiInterfaceError")]
    pub scsi_interface_error: Option<bool>,

/// 
    #[serde(rename = "TapeSnapped")]
    pub tape_snapped: Option<bool>,

/// 
    #[serde(rename = "TimetoCleanDrive")]
    pub timeto_clean_drive: Option<bool>,

/// 
    #[serde(rename = "UnsupportedFormat")]
    pub unsupported_format: Option<bool>,

/// 
    #[serde(rename = "WriteFailure")]
    pub write_failure: Option<bool>,

/// 
    #[serde(rename = "WriteWarning")]
    pub write_warning: Option<bool>,
}

impl MSTapeProblemDeviceError {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSTapeDriver::new(),
            active: None,
            drive_hardware_error: None,
            drive_requires_cleaning: None,
            hard_error: None,
            instance_name: None,
            media_life: None,
            read_failure: None,
            read_warning: None,
            scsi_interface_error: None,
            tape_snapped: None,
            timeto_clean_drive: None,
            unsupported_format: None,
            write_failure: None,
            write_warning: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of DriveHardwareError
    pub fn set_drive_hardware_error(&mut self, value: bool) {
        self.drive_hardware_error = Some(value);
    }

    /// Gets the value of DriveHardwareError
    pub fn get_drive_hardware_error(&self) -> Option<&bool> {
        self.drive_hardware_error.as_ref()
    }

    /// Sets the value of DriveRequiresCleaning
    pub fn set_drive_requires_cleaning(&mut self, value: bool) {
        self.drive_requires_cleaning = Some(value);
    }

    /// Gets the value of DriveRequiresCleaning
    pub fn get_drive_requires_cleaning(&self) -> Option<&bool> {
        self.drive_requires_cleaning.as_ref()
    }

    /// Sets the value of HardError
    pub fn set_hard_error(&mut self, value: bool) {
        self.hard_error = Some(value);
    }

    /// Gets the value of HardError
    pub fn get_hard_error(&self) -> Option<&bool> {
        self.hard_error.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of MediaLife
    pub fn set_media_life(&mut self, value: bool) {
        self.media_life = Some(value);
    }

    /// Gets the value of MediaLife
    pub fn get_media_life(&self) -> Option<&bool> {
        self.media_life.as_ref()
    }

    /// Sets the value of ReadFailure
    pub fn set_read_failure(&mut self, value: bool) {
        self.read_failure = Some(value);
    }

    /// Gets the value of ReadFailure
    pub fn get_read_failure(&self) -> Option<&bool> {
        self.read_failure.as_ref()
    }

    /// Sets the value of ReadWarning
    pub fn set_read_warning(&mut self, value: bool) {
        self.read_warning = Some(value);
    }

    /// Gets the value of ReadWarning
    pub fn get_read_warning(&self) -> Option<&bool> {
        self.read_warning.as_ref()
    }

    /// Sets the value of ScsiInterfaceError
    pub fn set_scsi_interface_error(&mut self, value: bool) {
        self.scsi_interface_error = Some(value);
    }

    /// Gets the value of ScsiInterfaceError
    pub fn get_scsi_interface_error(&self) -> Option<&bool> {
        self.scsi_interface_error.as_ref()
    }

    /// Sets the value of TapeSnapped
    pub fn set_tape_snapped(&mut self, value: bool) {
        self.tape_snapped = Some(value);
    }

    /// Gets the value of TapeSnapped
    pub fn get_tape_snapped(&self) -> Option<&bool> {
        self.tape_snapped.as_ref()
    }

    /// Sets the value of TimetoCleanDrive
    pub fn set_timeto_clean_drive(&mut self, value: bool) {
        self.timeto_clean_drive = Some(value);
    }

    /// Gets the value of TimetoCleanDrive
    pub fn get_timeto_clean_drive(&self) -> Option<&bool> {
        self.timeto_clean_drive.as_ref()
    }

    /// Sets the value of UnsupportedFormat
    pub fn set_unsupported_format(&mut self, value: bool) {
        self.unsupported_format = Some(value);
    }

    /// Gets the value of UnsupportedFormat
    pub fn get_unsupported_format(&self) -> Option<&bool> {
        self.unsupported_format.as_ref()
    }

    /// Sets the value of WriteFailure
    pub fn set_write_failure(&mut self, value: bool) {
        self.write_failure = Some(value);
    }

    /// Gets the value of WriteFailure
    pub fn get_write_failure(&self) -> Option<&bool> {
        self.write_failure.as_ref()
    }

    /// Sets the value of WriteWarning
    pub fn set_write_warning(&mut self, value: bool) {
        self.write_warning = Some(value);
    }

    /// Gets the value of WriteWarning
    pub fn get_write_warning(&self) -> Option<&bool> {
        self.write_warning.as_ref()
    }
}


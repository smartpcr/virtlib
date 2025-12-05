// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// FileOperation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileOperation {
    #[serde(flatten)]
    pub base: FileTrace,

/// 
    #[serde(rename = "AccessToken")]
    pub access_token: Option<u32>,

/// 
    #[serde(rename = "CreateOnExisting")]
    pub create_on_existing: Option<FileOperation_CreateOnExisting>,

/// 
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,

/// 
    #[serde(rename = "FileObject")]
    pub file_object: Option<u64>,

/// 
    #[serde(rename = "IsDirectory")]
    pub is_directory: Option<FileOperation_IsDirectory>,

/// 
    #[serde(rename = "IsFastIO")]
    pub is_fast_io: Option<u8>,

/// 
    #[serde(rename = "IsPagingIO")]
    pub is_paging_io: Option<u8>,

/// 
    #[serde(rename = "LastAccessTime")]
    pub last_access_time: Option<i64>,

/// 
    #[serde(rename = "MinorOperation")]
    pub minor_operation: Option<u8>,

/// 
    #[serde(rename = "Operation")]
    pub operation: Option<FileOperation_Operation>,

/// 
    #[serde(rename = "OperationalParameters")]
    pub operational_parameters: Vec<u8>,

/// 
    #[serde(rename = "ParametersLength")]
    pub parameters_length: Option<u32>,

/// 
    #[serde(rename = "PreviousValue")]
    pub previous_value: Vec<u8>,

/// 
    #[serde(rename = "PreviousValueLength")]
    pub previous_value_length: Option<u32>,

/// 
    #[serde(rename = "ProcessCreateTime")]
    pub process_create_time: Option<i64>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "ResultData")]
    pub result_data: Vec<u8>,

/// 
    #[serde(rename = "ResultLength")]
    pub result_length: Option<u32>,

/// 
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: Option<u32>,

/// 
    #[serde(rename = "SessionId")]
    pub session_id: Option<u32>,

/// 
    #[serde(rename = "SidLength")]
    pub sid_length: Option<u32>,

/// 
    #[serde(rename = "StartTime")]
    pub start_time: Option<i64>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u32>,

/// 
    #[serde(rename = "UserSID")]
    pub user_sid: Option<serde_json::Value>,

/// 
    #[serde(rename = "VolumeDosName")]
    pub volume_dos_name: Option<String>,

/// 
    #[serde(rename = "VolumeGuidName")]
    pub volume_guid_name: Option<String>,

/// 
    #[serde(rename = "VolumeName")]
    pub volume_name: Option<String>,

/// 
    #[serde(rename = "WindowStation")]
    pub window_station: Option<u64>,
}

impl FileOperation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: FileTrace::new(),
            access_token: None,
            create_on_existing: None,
            file_name: None,
            file_object: None,
            is_directory: None,
            is_fast_io: None,
            is_paging_io: None,
            last_access_time: None,
            minor_operation: None,
            operation: None,
            operational_parameters: Vec::new(),
            parameters_length: None,
            previous_value: Vec::new(),
            previous_value_length: None,
            process_create_time: None,
            process_id: None,
            result_data: Vec::new(),
            result_length: None,
            sequence_number: None,
            session_id: None,
            sid_length: None,
            start_time: None,
            status: None,
            user_sid: None,
            volume_dos_name: None,
            volume_guid_name: None,
            volume_name: None,
            window_station: None,
        }
    }


    /// Sets the value of AccessToken
    pub fn set_access_token(&mut self, value: u32) {
        self.access_token = Some(value);
    }

    /// Gets the value of AccessToken
    pub fn get_access_token(&self) -> Option<&u32> {
        self.access_token.as_ref()
    }

    /// Sets the value of CreateOnExisting
    pub fn set_create_on_existing(&mut self, value: FileOperation_CreateOnExisting) {
        self.create_on_existing = Some(value);
    }

    /// Gets the value of CreateOnExisting
    pub fn get_create_on_existing(&self) -> Option<&FileOperation_CreateOnExisting> {
        self.create_on_existing.as_ref()
    }

    /// Sets the value of FileName
    pub fn set_file_name(&mut self, value: String) {
        self.file_name = Some(value);
    }

    /// Gets the value of FileName
    pub fn get_file_name(&self) -> Option<&String> {
        self.file_name.as_ref()
    }

    /// Sets the value of FileObject
    pub fn set_file_object(&mut self, value: u64) {
        self.file_object = Some(value);
    }

    /// Gets the value of FileObject
    pub fn get_file_object(&self) -> Option<&u64> {
        self.file_object.as_ref()
    }

    /// Sets the value of IsDirectory
    pub fn set_is_directory(&mut self, value: FileOperation_IsDirectory) {
        self.is_directory = Some(value);
    }

    /// Gets the value of IsDirectory
    pub fn get_is_directory(&self) -> Option<&FileOperation_IsDirectory> {
        self.is_directory.as_ref()
    }

    /// Sets the value of IsFastIO
    pub fn set_is_fast_io(&mut self, value: u8) {
        self.is_fast_io = Some(value);
    }

    /// Gets the value of IsFastIO
    pub fn get_is_fast_io(&self) -> Option<&u8> {
        self.is_fast_io.as_ref()
    }

    /// Sets the value of IsPagingIO
    pub fn set_is_paging_io(&mut self, value: u8) {
        self.is_paging_io = Some(value);
    }

    /// Gets the value of IsPagingIO
    pub fn get_is_paging_io(&self) -> Option<&u8> {
        self.is_paging_io.as_ref()
    }

    /// Sets the value of LastAccessTime
    pub fn set_last_access_time(&mut self, value: i64) {
        self.last_access_time = Some(value);
    }

    /// Gets the value of LastAccessTime
    pub fn get_last_access_time(&self) -> Option<&i64> {
        self.last_access_time.as_ref()
    }

    /// Sets the value of MinorOperation
    pub fn set_minor_operation(&mut self, value: u8) {
        self.minor_operation = Some(value);
    }

    /// Gets the value of MinorOperation
    pub fn get_minor_operation(&self) -> Option<&u8> {
        self.minor_operation.as_ref()
    }

    /// Sets the value of Operation
    pub fn set_operation(&mut self, value: FileOperation_Operation) {
        self.operation = Some(value);
    }

    /// Gets the value of Operation
    pub fn get_operation(&self) -> Option<&FileOperation_Operation> {
        self.operation.as_ref()
    }

    /// Sets the value of OperationalParameters
    pub fn set_operational_parameters(&mut self, value: Vec<u8>) {
        self.operational_parameters = value;
    }

    /// Gets the value of OperationalParameters
    pub fn get_operational_parameters(&self) -> &Vec<u8> {
        &self.operational_parameters
    }

    /// Sets the value of ParametersLength
    pub fn set_parameters_length(&mut self, value: u32) {
        self.parameters_length = Some(value);
    }

    /// Gets the value of ParametersLength
    pub fn get_parameters_length(&self) -> Option<&u32> {
        self.parameters_length.as_ref()
    }

    /// Sets the value of PreviousValue
    pub fn set_previous_value(&mut self, value: Vec<u8>) {
        self.previous_value = value;
    }

    /// Gets the value of PreviousValue
    pub fn get_previous_value(&self) -> &Vec<u8> {
        &self.previous_value
    }

    /// Sets the value of PreviousValueLength
    pub fn set_previous_value_length(&mut self, value: u32) {
        self.previous_value_length = Some(value);
    }

    /// Gets the value of PreviousValueLength
    pub fn get_previous_value_length(&self) -> Option<&u32> {
        self.previous_value_length.as_ref()
    }

    /// Sets the value of ProcessCreateTime
    pub fn set_process_create_time(&mut self, value: i64) {
        self.process_create_time = Some(value);
    }

    /// Gets the value of ProcessCreateTime
    pub fn get_process_create_time(&self) -> Option<&i64> {
        self.process_create_time.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of ResultData
    pub fn set_result_data(&mut self, value: Vec<u8>) {
        self.result_data = value;
    }

    /// Gets the value of ResultData
    pub fn get_result_data(&self) -> &Vec<u8> {
        &self.result_data
    }

    /// Sets the value of ResultLength
    pub fn set_result_length(&mut self, value: u32) {
        self.result_length = Some(value);
    }

    /// Gets the value of ResultLength
    pub fn get_result_length(&self) -> Option<&u32> {
        self.result_length.as_ref()
    }

    /// Sets the value of SequenceNumber
    pub fn set_sequence_number(&mut self, value: u32) {
        self.sequence_number = Some(value);
    }

    /// Gets the value of SequenceNumber
    pub fn get_sequence_number(&self) -> Option<&u32> {
        self.sequence_number.as_ref()
    }

    /// Sets the value of SessionId
    pub fn set_session_id(&mut self, value: u32) {
        self.session_id = Some(value);
    }

    /// Gets the value of SessionId
    pub fn get_session_id(&self) -> Option<&u32> {
        self.session_id.as_ref()
    }

    /// Sets the value of SidLength
    pub fn set_sid_length(&mut self, value: u32) {
        self.sid_length = Some(value);
    }

    /// Gets the value of SidLength
    pub fn get_sid_length(&self) -> Option<&u32> {
        self.sid_length.as_ref()
    }

    /// Sets the value of StartTime
    pub fn set_start_time(&mut self, value: i64) {
        self.start_time = Some(value);
    }

    /// Gets the value of StartTime
    pub fn get_start_time(&self) -> Option<&i64> {
        self.start_time.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }

    /// Sets the value of UserSID
    pub fn set_user_sid(&mut self, value: serde_json::Value) {
        self.user_sid = Some(value);
    }

    /// Gets the value of UserSID
    pub fn get_user_sid(&self) -> Option<&serde_json::Value> {
        self.user_sid.as_ref()
    }

    /// Sets the value of VolumeDosName
    pub fn set_volume_dos_name(&mut self, value: String) {
        self.volume_dos_name = Some(value);
    }

    /// Gets the value of VolumeDosName
    pub fn get_volume_dos_name(&self) -> Option<&String> {
        self.volume_dos_name.as_ref()
    }

    /// Sets the value of VolumeGuidName
    pub fn set_volume_guid_name(&mut self, value: String) {
        self.volume_guid_name = Some(value);
    }

    /// Gets the value of VolumeGuidName
    pub fn get_volume_guid_name(&self) -> Option<&String> {
        self.volume_guid_name.as_ref()
    }

    /// Sets the value of VolumeName
    pub fn set_volume_name(&mut self, value: String) {
        self.volume_name = Some(value);
    }

    /// Gets the value of VolumeName
    pub fn get_volume_name(&self) -> Option<&String> {
        self.volume_name.as_ref()
    }

    /// Sets the value of WindowStation
    pub fn set_window_station(&mut self, value: u64) {
        self.window_station = Some(value);
    }

    /// Gets the value of WindowStation
    pub fn get_window_station(&self) -> Option<&u64> {
        self.window_station.as_ref()
    }
}


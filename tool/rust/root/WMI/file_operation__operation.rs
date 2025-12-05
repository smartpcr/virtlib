// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FileOperation_Operation
//////////////////////////////////////////////

/// FileOperation_Operation enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FileOperation_Operation {
    /// NORMALIZE_NAME_COMPONENT
    #[serde(rename = "NORMALIZE_NAME_COMPONENT")]
    NORMALIZENAMECOMPONENT = 0,
    /// GENERATE_FILE_NAME
    #[serde(rename = "GENERATE_FILE_NAME")]
    GENERATEFILENAME = 1,
    /// VOLUME_DISMOUNT
    #[serde(rename = "VOLUME_DISMOUNT")]
    VOLUMEDISMOUNT = 2,
    /// VOLUME_MOUNT
    #[serde(rename = "VOLUME_MOUNT")]
    VOLUMEMOUNT = 3,
    /// MDL_WRITE_COMPLETE
    #[serde(rename = "MDL_WRITE_COMPLETE")]
    MDLWRITECOMPLETE = 4,
    /// PREPARE_MDL_WRITE
    #[serde(rename = "PREPARE_MDL_WRITE")]
    PREPAREMDLWRITE = 5,
    /// MDL_READ_COMPLETE
    #[serde(rename = "MDL_READ_COMPLETE")]
    MDLREADCOMPLETE = 6,
    /// MDL_READ
    #[serde(rename = "MDL_READ")]
    MDLREAD = 7,
    /// NETWORK_QUERY_OPEN
    #[serde(rename = "NETWORK_QUERY_OPEN")]
    NETWORKQUERYOPEN = 8,
    /// FAST_IO_CHECK_IF_POSSIBLE
    #[serde(rename = "FAST_IO_CHECK_IF_POSSIBLE")]
    FASTIOCHECKIFPOSSIBLE = 9,
    /// NOTIFY_STREAM_FILE_OBJECT
    #[serde(rename = "NOTIFY_STREAM_FILE_OBJECT")]
    NOTIFYSTREAMFILEOBJECT = 15,
    /// RELEASE_FOR_CC_FLUSH
    #[serde(rename = "RELEASE_FOR_CC_FLUSH")]
    RELEASEFORCCFLUSH = 16,
    /// ACQUIRE_FOR_CC_FLUSH
    #[serde(rename = "ACQUIRE_FOR_CC_FLUSH")]
    ACQUIREFORCCFLUSH = 17,
    /// RELEASE_FOR_MOD_WRITE
    #[serde(rename = "RELEASE_FOR_MOD_WRITE")]
    RELEASEFORMODWRITE = 18,
    /// ACQUIRE_FOR_MOD_WRITE
    #[serde(rename = "ACQUIRE_FOR_MOD_WRITE")]
    ACQUIREFORMODWRITE = 19,
    /// RELEASE_FOR_SECTION_SYNCHRONIZATION
    #[serde(rename = "RELEASE_FOR_SECTION_SYNCHRONIZATION")]
    RELEASEFORSECTIONSYNCHRONIZATION = 20,
    /// ACQUIRE_FOR_SECTION_SYNCHRONIZATION
    #[serde(rename = "ACQUIRE_FOR_SECTION_SYNCHRONIZATION")]
    ACQUIREFORSECTIONSYNCHRONIZATION = 21,
    /// CREATE
    #[serde(rename = "CREATE")]
    CREATE = 22,
    /// CREATE_NAMED_PIPE
    #[serde(rename = "CREATE_NAMED_PIPE")]
    CREATENAMEDPIPE = 23,
    /// CLOSE
    #[serde(rename = "CLOSE")]
    CLOSE = 24,
    /// READ
    #[serde(rename = "READ")]
    READ = 25,
    /// WRITE
    #[serde(rename = "WRITE")]
    WRITE = 26,
    /// QUERY_INFORMATION
    #[serde(rename = "QUERY_INFORMATION")]
    QUERYINFORMATION = 27,
    /// SET_INFORMATION
    #[serde(rename = "SET_INFORMATION")]
    SETINFORMATION = 28,
    /// QUERY_EA
    #[serde(rename = "QUERY_EA")]
    QUERYEA = 29,
    /// SET_EA
    #[serde(rename = "SET_EA")]
    SETEA = 30,
    /// FLUSH_BUFFERS
    #[serde(rename = "FLUSH_BUFFERS")]
    FLUSHBUFFERS = 31,
    /// QUERY_VOLUME_INFORMATION
    #[serde(rename = "QUERY_VOLUME_INFORMATION")]
    QUERYVOLUMEINFORMATION = 32,
    /// SET_VOLUME_INFORMATION
    #[serde(rename = "SET_VOLUME_INFORMATION")]
    SETVOLUMEINFORMATION = 33,
    /// DIRECTORY_CONTROL
    #[serde(rename = "DIRECTORY_CONTROL")]
    DIRECTORYCONTROL = 34,
    /// FILE_SYSTEM_CONTROL
    #[serde(rename = "FILE_SYSTEM_CONTROL")]
    FILESYSTEMCONTROL = 35,
    /// DEVICE_CONTROL
    #[serde(rename = "DEVICE_CONTROL")]
    DEVICECONTROL = 36,
    /// INTERNAL_DEVICE_CONTROL
    #[serde(rename = "INTERNAL_DEVICE_CONTROL")]
    INTERNALDEVICECONTROL = 37,
    /// SHUTDOWN
    #[serde(rename = "SHUTDOWN")]
    SHUTDOWN = 38,
    /// LOCK_CONTROL
    #[serde(rename = "LOCK_CONTROL")]
    LOCKCONTROL = 39,
    /// CLEANUP
    #[serde(rename = "CLEANUP")]
    CLEANUP = 40,
    /// CREATE_MAILSLOT
    #[serde(rename = "CREATE_MAILSLOT")]
    CREATEMAILSLOT = 41,
    /// QUERY_SECURITY
    #[serde(rename = "QUERY_SECURITY")]
    QUERYSECURITY = 42,
    /// SET_SECURITY
    #[serde(rename = "SET_SECURITY")]
    SETSECURITY = 43,
    /// POWER
    #[serde(rename = "POWER")]
    POWER = 44,
    /// SYSTEM_CONTROL
    #[serde(rename = "SYSTEM_CONTROL")]
    SYSTEMCONTROL = 45,
    /// DEVICE_CHANGE
    #[serde(rename = "DEVICE_CHANGE")]
    DEVICECHANGE = 46,
    /// QUERY_QUOTA
    #[serde(rename = "QUERY_QUOTA")]
    QUERYQUOTA = 47,
    /// SET_QUOTA
    #[serde(rename = "SET_QUOTA")]
    SETQUOTA = 48,
    /// PNP
    #[serde(rename = "PNP")]
    PNP = 49,
}

impl Default for FileOperation_Operation {
    fn default() -> Self {
        Self::NORMALIZENAMECOMPONENT
    }
}


// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_ClusBfltPerfProvider_ClusterStorageHybridDisksIOProfile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_ClusBfltPerfProvider_ClusterStorageHybridDisksIOProfile {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "Reads0K4K")]
    pub reads0_k4_k: Option<u64>,

/// 
    #[serde(rename = "Reads1024K2048K")]
    pub reads1024_k2048_k: Option<u64>,

/// 
    #[serde(rename = "Reads128K256K")]
    pub reads128_k256_k: Option<u64>,

/// 
    #[serde(rename = "Reads16K32K")]
    pub reads16_k32_k: Option<u64>,

/// 
    #[serde(rename = "Reads2048K4096K")]
    pub reads2048_k4096_k: Option<u64>,

/// 
    #[serde(rename = "Reads256K512K")]
    pub reads256_k512_k: Option<u64>,

/// 
    #[serde(rename = "Reads32K64K")]
    pub reads32_k64_k: Option<u64>,

/// 
    #[serde(rename = "Reads4096Koo")]
    pub reads4096_koo: Option<u64>,

/// 
    #[serde(rename = "Reads4K8K")]
    pub reads4_k8_k: Option<u64>,

/// 
    #[serde(rename = "Reads512K1024K")]
    pub reads512_k1024_k: Option<u64>,

/// 
    #[serde(rename = "Reads64K128K")]
    pub reads64_k128_k: Option<u64>,

/// 
    #[serde(rename = "Reads8K16K")]
    pub reads8_k16_k: Option<u64>,

/// 
    #[serde(rename = "Readsnotaligned")]
    pub readsnotaligned: Option<u64>,

/// 
    #[serde(rename = "ReadsPagingIO")]
    pub reads_paging_io: Option<u64>,

/// 
    #[serde(rename = "ReadsPersec0K4K")]
    pub reads_persec0_k4_k: Option<u64>,

/// 
    #[serde(rename = "ReadsPersec1024K2048K")]
    pub reads_persec1024_k2048_k: Option<u64>,

/// 
    #[serde(rename = "ReadsPersec128K256K")]
    pub reads_persec128_k256_k: Option<u64>,

/// 
    #[serde(rename = "ReadsPersec16K32K")]
    pub reads_persec16_k32_k: Option<u64>,

/// 
    #[serde(rename = "ReadsPersec2048K4096K")]
    pub reads_persec2048_k4096_k: Option<u64>,

/// 
    #[serde(rename = "ReadsPersec256K512K")]
    pub reads_persec256_k512_k: Option<u64>,

/// 
    #[serde(rename = "ReadsPersec32K64K")]
    pub reads_persec32_k64_k: Option<u64>,

/// 
    #[serde(rename = "ReadsPersec4096Koo")]
    pub reads_persec4096_koo: Option<u64>,

/// 
    #[serde(rename = "ReadsPersec4K8K")]
    pub reads_persec4_k8_k: Option<u64>,

/// 
    #[serde(rename = "ReadsPersec512K1024K")]
    pub reads_persec512_k1024_k: Option<u64>,

/// 
    #[serde(rename = "ReadsPersec64K128K")]
    pub reads_persec64_k128_k: Option<u64>,

/// 
    #[serde(rename = "ReadsPersec8K16K")]
    pub reads_persec8_k16_k: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecnotaligned")]
    pub reads_persecnotaligned: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecPagingIO")]
    pub reads_persec_paging_io: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecTotal")]
    pub reads_persec_total: Option<u64>,

/// 
    #[serde(rename = "ReadsTotal")]
    pub reads_total: Option<u64>,

/// 
    #[serde(rename = "Writes0K4K")]
    pub writes0_k4_k: Option<u64>,

/// 
    #[serde(rename = "Writes1024K2048K")]
    pub writes1024_k2048_k: Option<u64>,

/// 
    #[serde(rename = "Writes128K256K")]
    pub writes128_k256_k: Option<u64>,

/// 
    #[serde(rename = "Writes16K32K")]
    pub writes16_k32_k: Option<u64>,

/// 
    #[serde(rename = "Writes2048K4096K")]
    pub writes2048_k4096_k: Option<u64>,

/// 
    #[serde(rename = "Writes256K512K")]
    pub writes256_k512_k: Option<u64>,

/// 
    #[serde(rename = "Writes32K64K")]
    pub writes32_k64_k: Option<u64>,

/// 
    #[serde(rename = "Writes4096Koo")]
    pub writes4096_koo: Option<u64>,

/// 
    #[serde(rename = "Writes4K8K")]
    pub writes4_k8_k: Option<u64>,

/// 
    #[serde(rename = "Writes512K1024K")]
    pub writes512_k1024_k: Option<u64>,

/// 
    #[serde(rename = "Writes64K128K")]
    pub writes64_k128_k: Option<u64>,

/// 
    #[serde(rename = "Writes8K16K")]
    pub writes8_k16_k: Option<u64>,

/// 
    #[serde(rename = "Writesnotaligned")]
    pub writesnotaligned: Option<u64>,

/// 
    #[serde(rename = "WritesPagingIO")]
    pub writes_paging_io: Option<u64>,

/// 
    #[serde(rename = "WritesPersec0K4K")]
    pub writes_persec0_k4_k: Option<u64>,

/// 
    #[serde(rename = "WritesPersec1024K2048K")]
    pub writes_persec1024_k2048_k: Option<u64>,

/// 
    #[serde(rename = "WritesPersec128K256K")]
    pub writes_persec128_k256_k: Option<u64>,

/// 
    #[serde(rename = "WritesPersec16K32K")]
    pub writes_persec16_k32_k: Option<u64>,

/// 
    #[serde(rename = "WritesPersec2048K4096K")]
    pub writes_persec2048_k4096_k: Option<u64>,

/// 
    #[serde(rename = "WritesPersec256K512K")]
    pub writes_persec256_k512_k: Option<u64>,

/// 
    #[serde(rename = "WritesPersec32K64K")]
    pub writes_persec32_k64_k: Option<u64>,

/// 
    #[serde(rename = "WritesPersec4096Koo")]
    pub writes_persec4096_koo: Option<u64>,

/// 
    #[serde(rename = "WritesPersec4K8K")]
    pub writes_persec4_k8_k: Option<u64>,

/// 
    #[serde(rename = "WritesPersec512K1024K")]
    pub writes_persec512_k1024_k: Option<u64>,

/// 
    #[serde(rename = "WritesPersec64K128K")]
    pub writes_persec64_k128_k: Option<u64>,

/// 
    #[serde(rename = "WritesPersec8K16K")]
    pub writes_persec8_k16_k: Option<u64>,

/// 
    #[serde(rename = "WritesPersecnotaligned")]
    pub writes_persecnotaligned: Option<u64>,

/// 
    #[serde(rename = "WritesPersecPagingIO")]
    pub writes_persec_paging_io: Option<u64>,

/// 
    #[serde(rename = "WritesPersecTotal")]
    pub writes_persec_total: Option<u64>,

/// 
    #[serde(rename = "WritesTotal")]
    pub writes_total: Option<u64>,
}

impl Win32_PerfFormattedData_ClusBfltPerfProvider_ClusterStorageHybridDisksIOProfile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            reads0_k4_k: None,
            reads1024_k2048_k: None,
            reads128_k256_k: None,
            reads16_k32_k: None,
            reads2048_k4096_k: None,
            reads256_k512_k: None,
            reads32_k64_k: None,
            reads4096_koo: None,
            reads4_k8_k: None,
            reads512_k1024_k: None,
            reads64_k128_k: None,
            reads8_k16_k: None,
            readsnotaligned: None,
            reads_paging_io: None,
            reads_persec0_k4_k: None,
            reads_persec1024_k2048_k: None,
            reads_persec128_k256_k: None,
            reads_persec16_k32_k: None,
            reads_persec2048_k4096_k: None,
            reads_persec256_k512_k: None,
            reads_persec32_k64_k: None,
            reads_persec4096_koo: None,
            reads_persec4_k8_k: None,
            reads_persec512_k1024_k: None,
            reads_persec64_k128_k: None,
            reads_persec8_k16_k: None,
            reads_persecnotaligned: None,
            reads_persec_paging_io: None,
            reads_persec_total: None,
            reads_total: None,
            writes0_k4_k: None,
            writes1024_k2048_k: None,
            writes128_k256_k: None,
            writes16_k32_k: None,
            writes2048_k4096_k: None,
            writes256_k512_k: None,
            writes32_k64_k: None,
            writes4096_koo: None,
            writes4_k8_k: None,
            writes512_k1024_k: None,
            writes64_k128_k: None,
            writes8_k16_k: None,
            writesnotaligned: None,
            writes_paging_io: None,
            writes_persec0_k4_k: None,
            writes_persec1024_k2048_k: None,
            writes_persec128_k256_k: None,
            writes_persec16_k32_k: None,
            writes_persec2048_k4096_k: None,
            writes_persec256_k512_k: None,
            writes_persec32_k64_k: None,
            writes_persec4096_koo: None,
            writes_persec4_k8_k: None,
            writes_persec512_k1024_k: None,
            writes_persec64_k128_k: None,
            writes_persec8_k16_k: None,
            writes_persecnotaligned: None,
            writes_persec_paging_io: None,
            writes_persec_total: None,
            writes_total: None,
        }
    }


    /// Sets the value of Reads0K4K
    pub fn set_reads0_k4_k(&mut self, value: u64) {
        self.reads0_k4_k = Some(value);
    }

    /// Gets the value of Reads0K4K
    pub fn get_reads0_k4_k(&self) -> Option<&u64> {
        self.reads0_k4_k.as_ref()
    }

    /// Sets the value of Reads1024K2048K
    pub fn set_reads1024_k2048_k(&mut self, value: u64) {
        self.reads1024_k2048_k = Some(value);
    }

    /// Gets the value of Reads1024K2048K
    pub fn get_reads1024_k2048_k(&self) -> Option<&u64> {
        self.reads1024_k2048_k.as_ref()
    }

    /// Sets the value of Reads128K256K
    pub fn set_reads128_k256_k(&mut self, value: u64) {
        self.reads128_k256_k = Some(value);
    }

    /// Gets the value of Reads128K256K
    pub fn get_reads128_k256_k(&self) -> Option<&u64> {
        self.reads128_k256_k.as_ref()
    }

    /// Sets the value of Reads16K32K
    pub fn set_reads16_k32_k(&mut self, value: u64) {
        self.reads16_k32_k = Some(value);
    }

    /// Gets the value of Reads16K32K
    pub fn get_reads16_k32_k(&self) -> Option<&u64> {
        self.reads16_k32_k.as_ref()
    }

    /// Sets the value of Reads2048K4096K
    pub fn set_reads2048_k4096_k(&mut self, value: u64) {
        self.reads2048_k4096_k = Some(value);
    }

    /// Gets the value of Reads2048K4096K
    pub fn get_reads2048_k4096_k(&self) -> Option<&u64> {
        self.reads2048_k4096_k.as_ref()
    }

    /// Sets the value of Reads256K512K
    pub fn set_reads256_k512_k(&mut self, value: u64) {
        self.reads256_k512_k = Some(value);
    }

    /// Gets the value of Reads256K512K
    pub fn get_reads256_k512_k(&self) -> Option<&u64> {
        self.reads256_k512_k.as_ref()
    }

    /// Sets the value of Reads32K64K
    pub fn set_reads32_k64_k(&mut self, value: u64) {
        self.reads32_k64_k = Some(value);
    }

    /// Gets the value of Reads32K64K
    pub fn get_reads32_k64_k(&self) -> Option<&u64> {
        self.reads32_k64_k.as_ref()
    }

    /// Sets the value of Reads4096Koo
    pub fn set_reads4096_koo(&mut self, value: u64) {
        self.reads4096_koo = Some(value);
    }

    /// Gets the value of Reads4096Koo
    pub fn get_reads4096_koo(&self) -> Option<&u64> {
        self.reads4096_koo.as_ref()
    }

    /// Sets the value of Reads4K8K
    pub fn set_reads4_k8_k(&mut self, value: u64) {
        self.reads4_k8_k = Some(value);
    }

    /// Gets the value of Reads4K8K
    pub fn get_reads4_k8_k(&self) -> Option<&u64> {
        self.reads4_k8_k.as_ref()
    }

    /// Sets the value of Reads512K1024K
    pub fn set_reads512_k1024_k(&mut self, value: u64) {
        self.reads512_k1024_k = Some(value);
    }

    /// Gets the value of Reads512K1024K
    pub fn get_reads512_k1024_k(&self) -> Option<&u64> {
        self.reads512_k1024_k.as_ref()
    }

    /// Sets the value of Reads64K128K
    pub fn set_reads64_k128_k(&mut self, value: u64) {
        self.reads64_k128_k = Some(value);
    }

    /// Gets the value of Reads64K128K
    pub fn get_reads64_k128_k(&self) -> Option<&u64> {
        self.reads64_k128_k.as_ref()
    }

    /// Sets the value of Reads8K16K
    pub fn set_reads8_k16_k(&mut self, value: u64) {
        self.reads8_k16_k = Some(value);
    }

    /// Gets the value of Reads8K16K
    pub fn get_reads8_k16_k(&self) -> Option<&u64> {
        self.reads8_k16_k.as_ref()
    }

    /// Sets the value of Readsnotaligned
    pub fn set_readsnotaligned(&mut self, value: u64) {
        self.readsnotaligned = Some(value);
    }

    /// Gets the value of Readsnotaligned
    pub fn get_readsnotaligned(&self) -> Option<&u64> {
        self.readsnotaligned.as_ref()
    }

    /// Sets the value of ReadsPagingIO
    pub fn set_reads_paging_io(&mut self, value: u64) {
        self.reads_paging_io = Some(value);
    }

    /// Gets the value of ReadsPagingIO
    pub fn get_reads_paging_io(&self) -> Option<&u64> {
        self.reads_paging_io.as_ref()
    }

    /// Sets the value of ReadsPersec0K4K
    pub fn set_reads_persec0_k4_k(&mut self, value: u64) {
        self.reads_persec0_k4_k = Some(value);
    }

    /// Gets the value of ReadsPersec0K4K
    pub fn get_reads_persec0_k4_k(&self) -> Option<&u64> {
        self.reads_persec0_k4_k.as_ref()
    }

    /// Sets the value of ReadsPersec1024K2048K
    pub fn set_reads_persec1024_k2048_k(&mut self, value: u64) {
        self.reads_persec1024_k2048_k = Some(value);
    }

    /// Gets the value of ReadsPersec1024K2048K
    pub fn get_reads_persec1024_k2048_k(&self) -> Option<&u64> {
        self.reads_persec1024_k2048_k.as_ref()
    }

    /// Sets the value of ReadsPersec128K256K
    pub fn set_reads_persec128_k256_k(&mut self, value: u64) {
        self.reads_persec128_k256_k = Some(value);
    }

    /// Gets the value of ReadsPersec128K256K
    pub fn get_reads_persec128_k256_k(&self) -> Option<&u64> {
        self.reads_persec128_k256_k.as_ref()
    }

    /// Sets the value of ReadsPersec16K32K
    pub fn set_reads_persec16_k32_k(&mut self, value: u64) {
        self.reads_persec16_k32_k = Some(value);
    }

    /// Gets the value of ReadsPersec16K32K
    pub fn get_reads_persec16_k32_k(&self) -> Option<&u64> {
        self.reads_persec16_k32_k.as_ref()
    }

    /// Sets the value of ReadsPersec2048K4096K
    pub fn set_reads_persec2048_k4096_k(&mut self, value: u64) {
        self.reads_persec2048_k4096_k = Some(value);
    }

    /// Gets the value of ReadsPersec2048K4096K
    pub fn get_reads_persec2048_k4096_k(&self) -> Option<&u64> {
        self.reads_persec2048_k4096_k.as_ref()
    }

    /// Sets the value of ReadsPersec256K512K
    pub fn set_reads_persec256_k512_k(&mut self, value: u64) {
        self.reads_persec256_k512_k = Some(value);
    }

    /// Gets the value of ReadsPersec256K512K
    pub fn get_reads_persec256_k512_k(&self) -> Option<&u64> {
        self.reads_persec256_k512_k.as_ref()
    }

    /// Sets the value of ReadsPersec32K64K
    pub fn set_reads_persec32_k64_k(&mut self, value: u64) {
        self.reads_persec32_k64_k = Some(value);
    }

    /// Gets the value of ReadsPersec32K64K
    pub fn get_reads_persec32_k64_k(&self) -> Option<&u64> {
        self.reads_persec32_k64_k.as_ref()
    }

    /// Sets the value of ReadsPersec4096Koo
    pub fn set_reads_persec4096_koo(&mut self, value: u64) {
        self.reads_persec4096_koo = Some(value);
    }

    /// Gets the value of ReadsPersec4096Koo
    pub fn get_reads_persec4096_koo(&self) -> Option<&u64> {
        self.reads_persec4096_koo.as_ref()
    }

    /// Sets the value of ReadsPersec4K8K
    pub fn set_reads_persec4_k8_k(&mut self, value: u64) {
        self.reads_persec4_k8_k = Some(value);
    }

    /// Gets the value of ReadsPersec4K8K
    pub fn get_reads_persec4_k8_k(&self) -> Option<&u64> {
        self.reads_persec4_k8_k.as_ref()
    }

    /// Sets the value of ReadsPersec512K1024K
    pub fn set_reads_persec512_k1024_k(&mut self, value: u64) {
        self.reads_persec512_k1024_k = Some(value);
    }

    /// Gets the value of ReadsPersec512K1024K
    pub fn get_reads_persec512_k1024_k(&self) -> Option<&u64> {
        self.reads_persec512_k1024_k.as_ref()
    }

    /// Sets the value of ReadsPersec64K128K
    pub fn set_reads_persec64_k128_k(&mut self, value: u64) {
        self.reads_persec64_k128_k = Some(value);
    }

    /// Gets the value of ReadsPersec64K128K
    pub fn get_reads_persec64_k128_k(&self) -> Option<&u64> {
        self.reads_persec64_k128_k.as_ref()
    }

    /// Sets the value of ReadsPersec8K16K
    pub fn set_reads_persec8_k16_k(&mut self, value: u64) {
        self.reads_persec8_k16_k = Some(value);
    }

    /// Gets the value of ReadsPersec8K16K
    pub fn get_reads_persec8_k16_k(&self) -> Option<&u64> {
        self.reads_persec8_k16_k.as_ref()
    }

    /// Sets the value of ReadsPersecnotaligned
    pub fn set_reads_persecnotaligned(&mut self, value: u64) {
        self.reads_persecnotaligned = Some(value);
    }

    /// Gets the value of ReadsPersecnotaligned
    pub fn get_reads_persecnotaligned(&self) -> Option<&u64> {
        self.reads_persecnotaligned.as_ref()
    }

    /// Sets the value of ReadsPersecPagingIO
    pub fn set_reads_persec_paging_io(&mut self, value: u64) {
        self.reads_persec_paging_io = Some(value);
    }

    /// Gets the value of ReadsPersecPagingIO
    pub fn get_reads_persec_paging_io(&self) -> Option<&u64> {
        self.reads_persec_paging_io.as_ref()
    }

    /// Sets the value of ReadsPersecTotal
    pub fn set_reads_persec_total(&mut self, value: u64) {
        self.reads_persec_total = Some(value);
    }

    /// Gets the value of ReadsPersecTotal
    pub fn get_reads_persec_total(&self) -> Option<&u64> {
        self.reads_persec_total.as_ref()
    }

    /// Sets the value of ReadsTotal
    pub fn set_reads_total(&mut self, value: u64) {
        self.reads_total = Some(value);
    }

    /// Gets the value of ReadsTotal
    pub fn get_reads_total(&self) -> Option<&u64> {
        self.reads_total.as_ref()
    }

    /// Sets the value of Writes0K4K
    pub fn set_writes0_k4_k(&mut self, value: u64) {
        self.writes0_k4_k = Some(value);
    }

    /// Gets the value of Writes0K4K
    pub fn get_writes0_k4_k(&self) -> Option<&u64> {
        self.writes0_k4_k.as_ref()
    }

    /// Sets the value of Writes1024K2048K
    pub fn set_writes1024_k2048_k(&mut self, value: u64) {
        self.writes1024_k2048_k = Some(value);
    }

    /// Gets the value of Writes1024K2048K
    pub fn get_writes1024_k2048_k(&self) -> Option<&u64> {
        self.writes1024_k2048_k.as_ref()
    }

    /// Sets the value of Writes128K256K
    pub fn set_writes128_k256_k(&mut self, value: u64) {
        self.writes128_k256_k = Some(value);
    }

    /// Gets the value of Writes128K256K
    pub fn get_writes128_k256_k(&self) -> Option<&u64> {
        self.writes128_k256_k.as_ref()
    }

    /// Sets the value of Writes16K32K
    pub fn set_writes16_k32_k(&mut self, value: u64) {
        self.writes16_k32_k = Some(value);
    }

    /// Gets the value of Writes16K32K
    pub fn get_writes16_k32_k(&self) -> Option<&u64> {
        self.writes16_k32_k.as_ref()
    }

    /// Sets the value of Writes2048K4096K
    pub fn set_writes2048_k4096_k(&mut self, value: u64) {
        self.writes2048_k4096_k = Some(value);
    }

    /// Gets the value of Writes2048K4096K
    pub fn get_writes2048_k4096_k(&self) -> Option<&u64> {
        self.writes2048_k4096_k.as_ref()
    }

    /// Sets the value of Writes256K512K
    pub fn set_writes256_k512_k(&mut self, value: u64) {
        self.writes256_k512_k = Some(value);
    }

    /// Gets the value of Writes256K512K
    pub fn get_writes256_k512_k(&self) -> Option<&u64> {
        self.writes256_k512_k.as_ref()
    }

    /// Sets the value of Writes32K64K
    pub fn set_writes32_k64_k(&mut self, value: u64) {
        self.writes32_k64_k = Some(value);
    }

    /// Gets the value of Writes32K64K
    pub fn get_writes32_k64_k(&self) -> Option<&u64> {
        self.writes32_k64_k.as_ref()
    }

    /// Sets the value of Writes4096Koo
    pub fn set_writes4096_koo(&mut self, value: u64) {
        self.writes4096_koo = Some(value);
    }

    /// Gets the value of Writes4096Koo
    pub fn get_writes4096_koo(&self) -> Option<&u64> {
        self.writes4096_koo.as_ref()
    }

    /// Sets the value of Writes4K8K
    pub fn set_writes4_k8_k(&mut self, value: u64) {
        self.writes4_k8_k = Some(value);
    }

    /// Gets the value of Writes4K8K
    pub fn get_writes4_k8_k(&self) -> Option<&u64> {
        self.writes4_k8_k.as_ref()
    }

    /// Sets the value of Writes512K1024K
    pub fn set_writes512_k1024_k(&mut self, value: u64) {
        self.writes512_k1024_k = Some(value);
    }

    /// Gets the value of Writes512K1024K
    pub fn get_writes512_k1024_k(&self) -> Option<&u64> {
        self.writes512_k1024_k.as_ref()
    }

    /// Sets the value of Writes64K128K
    pub fn set_writes64_k128_k(&mut self, value: u64) {
        self.writes64_k128_k = Some(value);
    }

    /// Gets the value of Writes64K128K
    pub fn get_writes64_k128_k(&self) -> Option<&u64> {
        self.writes64_k128_k.as_ref()
    }

    /// Sets the value of Writes8K16K
    pub fn set_writes8_k16_k(&mut self, value: u64) {
        self.writes8_k16_k = Some(value);
    }

    /// Gets the value of Writes8K16K
    pub fn get_writes8_k16_k(&self) -> Option<&u64> {
        self.writes8_k16_k.as_ref()
    }

    /// Sets the value of Writesnotaligned
    pub fn set_writesnotaligned(&mut self, value: u64) {
        self.writesnotaligned = Some(value);
    }

    /// Gets the value of Writesnotaligned
    pub fn get_writesnotaligned(&self) -> Option<&u64> {
        self.writesnotaligned.as_ref()
    }

    /// Sets the value of WritesPagingIO
    pub fn set_writes_paging_io(&mut self, value: u64) {
        self.writes_paging_io = Some(value);
    }

    /// Gets the value of WritesPagingIO
    pub fn get_writes_paging_io(&self) -> Option<&u64> {
        self.writes_paging_io.as_ref()
    }

    /// Sets the value of WritesPersec0K4K
    pub fn set_writes_persec0_k4_k(&mut self, value: u64) {
        self.writes_persec0_k4_k = Some(value);
    }

    /// Gets the value of WritesPersec0K4K
    pub fn get_writes_persec0_k4_k(&self) -> Option<&u64> {
        self.writes_persec0_k4_k.as_ref()
    }

    /// Sets the value of WritesPersec1024K2048K
    pub fn set_writes_persec1024_k2048_k(&mut self, value: u64) {
        self.writes_persec1024_k2048_k = Some(value);
    }

    /// Gets the value of WritesPersec1024K2048K
    pub fn get_writes_persec1024_k2048_k(&self) -> Option<&u64> {
        self.writes_persec1024_k2048_k.as_ref()
    }

    /// Sets the value of WritesPersec128K256K
    pub fn set_writes_persec128_k256_k(&mut self, value: u64) {
        self.writes_persec128_k256_k = Some(value);
    }

    /// Gets the value of WritesPersec128K256K
    pub fn get_writes_persec128_k256_k(&self) -> Option<&u64> {
        self.writes_persec128_k256_k.as_ref()
    }

    /// Sets the value of WritesPersec16K32K
    pub fn set_writes_persec16_k32_k(&mut self, value: u64) {
        self.writes_persec16_k32_k = Some(value);
    }

    /// Gets the value of WritesPersec16K32K
    pub fn get_writes_persec16_k32_k(&self) -> Option<&u64> {
        self.writes_persec16_k32_k.as_ref()
    }

    /// Sets the value of WritesPersec2048K4096K
    pub fn set_writes_persec2048_k4096_k(&mut self, value: u64) {
        self.writes_persec2048_k4096_k = Some(value);
    }

    /// Gets the value of WritesPersec2048K4096K
    pub fn get_writes_persec2048_k4096_k(&self) -> Option<&u64> {
        self.writes_persec2048_k4096_k.as_ref()
    }

    /// Sets the value of WritesPersec256K512K
    pub fn set_writes_persec256_k512_k(&mut self, value: u64) {
        self.writes_persec256_k512_k = Some(value);
    }

    /// Gets the value of WritesPersec256K512K
    pub fn get_writes_persec256_k512_k(&self) -> Option<&u64> {
        self.writes_persec256_k512_k.as_ref()
    }

    /// Sets the value of WritesPersec32K64K
    pub fn set_writes_persec32_k64_k(&mut self, value: u64) {
        self.writes_persec32_k64_k = Some(value);
    }

    /// Gets the value of WritesPersec32K64K
    pub fn get_writes_persec32_k64_k(&self) -> Option<&u64> {
        self.writes_persec32_k64_k.as_ref()
    }

    /// Sets the value of WritesPersec4096Koo
    pub fn set_writes_persec4096_koo(&mut self, value: u64) {
        self.writes_persec4096_koo = Some(value);
    }

    /// Gets the value of WritesPersec4096Koo
    pub fn get_writes_persec4096_koo(&self) -> Option<&u64> {
        self.writes_persec4096_koo.as_ref()
    }

    /// Sets the value of WritesPersec4K8K
    pub fn set_writes_persec4_k8_k(&mut self, value: u64) {
        self.writes_persec4_k8_k = Some(value);
    }

    /// Gets the value of WritesPersec4K8K
    pub fn get_writes_persec4_k8_k(&self) -> Option<&u64> {
        self.writes_persec4_k8_k.as_ref()
    }

    /// Sets the value of WritesPersec512K1024K
    pub fn set_writes_persec512_k1024_k(&mut self, value: u64) {
        self.writes_persec512_k1024_k = Some(value);
    }

    /// Gets the value of WritesPersec512K1024K
    pub fn get_writes_persec512_k1024_k(&self) -> Option<&u64> {
        self.writes_persec512_k1024_k.as_ref()
    }

    /// Sets the value of WritesPersec64K128K
    pub fn set_writes_persec64_k128_k(&mut self, value: u64) {
        self.writes_persec64_k128_k = Some(value);
    }

    /// Gets the value of WritesPersec64K128K
    pub fn get_writes_persec64_k128_k(&self) -> Option<&u64> {
        self.writes_persec64_k128_k.as_ref()
    }

    /// Sets the value of WritesPersec8K16K
    pub fn set_writes_persec8_k16_k(&mut self, value: u64) {
        self.writes_persec8_k16_k = Some(value);
    }

    /// Gets the value of WritesPersec8K16K
    pub fn get_writes_persec8_k16_k(&self) -> Option<&u64> {
        self.writes_persec8_k16_k.as_ref()
    }

    /// Sets the value of WritesPersecnotaligned
    pub fn set_writes_persecnotaligned(&mut self, value: u64) {
        self.writes_persecnotaligned = Some(value);
    }

    /// Gets the value of WritesPersecnotaligned
    pub fn get_writes_persecnotaligned(&self) -> Option<&u64> {
        self.writes_persecnotaligned.as_ref()
    }

    /// Sets the value of WritesPersecPagingIO
    pub fn set_writes_persec_paging_io(&mut self, value: u64) {
        self.writes_persec_paging_io = Some(value);
    }

    /// Gets the value of WritesPersecPagingIO
    pub fn get_writes_persec_paging_io(&self) -> Option<&u64> {
        self.writes_persec_paging_io.as_ref()
    }

    /// Sets the value of WritesPersecTotal
    pub fn set_writes_persec_total(&mut self, value: u64) {
        self.writes_persec_total = Some(value);
    }

    /// Gets the value of WritesPersecTotal
    pub fn get_writes_persec_total(&self) -> Option<&u64> {
        self.writes_persec_total.as_ref()
    }

    /// Sets the value of WritesTotal
    pub fn set_writes_total(&mut self, value: u64) {
        self.writes_total = Some(value);
    }

    /// Gets the value of WritesTotal
    pub fn get_writes_total(&self) -> Option<&u64> {
        self.writes_total.as_ref()
    }
}


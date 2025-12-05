// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_LmPerfProvider_HyperVVMLiveMigration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_LmPerfProvider_HyperVVMLiveMigration {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "CompressorBytestobeCompressed")]
    pub compressor_bytestobe_compressed: Option<u64>,

/// 
    #[serde(rename = "CompressorCompressedBytesSent")]
    pub compressor_compressed_bytes_sent: Option<u64>,

/// 
    #[serde(rename = "CompressorCompressedBytesSentPersec")]
    pub compressor_compressed_bytes_sent_persec: Option<u64>,

/// 
    #[serde(rename = "CompressorEnabledThreads")]
    pub compressor_enabled_threads: Option<u64>,

/// 
    #[serde(rename = "CompressorMaximumThreads")]
    pub compressor_maximum_threads: Option<u64>,

/// 
    #[serde(rename = "MemoryWalkerBytesReadPersec")]
    pub memory_walker_bytes_read_persec: Option<u64>,

/// 
    #[serde(rename = "MemoryWalkerBytesSentforCompression")]
    pub memory_walker_bytes_sentfor_compression: Option<u64>,

/// 
    #[serde(rename = "MemoryWalkerBytesSentforCompressionPersec")]
    pub memory_walker_bytes_sentfor_compression_persec: Option<u64>,

/// 
    #[serde(rename = "MemoryWalkerMaximumThreads")]
    pub memory_walker_maximum_threads: Option<u64>,

/// 
    #[serde(rename = "MemoryWalkerUncompressedBytesSent")]
    pub memory_walker_uncompressed_bytes_sent: Option<u64>,

/// 
    #[serde(rename = "MemoryWalkerUncompressedBytesSentPersec")]
    pub memory_walker_uncompressed_bytes_sent_persec: Option<u64>,

/// 
    #[serde(rename = "ReceiverBytesPendingDecompression")]
    pub receiver_bytes_pending_decompression: Option<u64>,

/// 
    #[serde(rename = "ReceiverBytesPendingWrite")]
    pub receiver_bytes_pending_write: Option<u64>,

/// 
    #[serde(rename = "ReceiverBytesWrittenPersec")]
    pub receiver_bytes_written_persec: Option<u64>,

/// 
    #[serde(rename = "ReceiverCompressedBytesReceivedPersec")]
    pub receiver_compressed_bytes_received_persec: Option<u64>,

/// 
    #[serde(rename = "ReceiverDecompressedBytesPersec")]
    pub receiver_decompressed_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "ReceiverMaximumThreadpoolThreadCount")]
    pub receiver_maximum_threadpool_thread_count: Option<u64>,

/// 
    #[serde(rename = "ReceiverUncompressedBytesReceivedPersec")]
    pub receiver_uncompressed_bytes_received_persec: Option<u64>,

/// 
    #[serde(rename = "SMBTransportBytesSent")]
    pub smbtransport_bytes_sent: Option<u64>,

/// 
    #[serde(rename = "SMBTransportBytesSentPersec")]
    pub smbtransport_bytes_sent_persec: Option<u64>,

/// 
    #[serde(rename = "SMBTransportPendingSendBytes")]
    pub smbtransport_pending_send_bytes: Option<u64>,

/// 
    #[serde(rename = "SMBTransportPendingSendCount")]
    pub smbtransport_pending_send_count: Option<u64>,

/// 
    #[serde(rename = "TCPTransportBytesPendingProcessing")]
    pub tcptransport_bytes_pending_processing: Option<u64>,

/// 
    #[serde(rename = "TCPTransportBytesPendingSend")]
    pub tcptransport_bytes_pending_send: Option<u64>,

/// 
    #[serde(rename = "TCPTransportBytesReceivedPersec")]
    pub tcptransport_bytes_received_persec: Option<u64>,

/// 
    #[serde(rename = "TCPTransportBytesSentPersec")]
    pub tcptransport_bytes_sent_persec: Option<u64>,

/// 
    #[serde(rename = "TCPTransportPendingSendCount")]
    pub tcptransport_pending_send_count: Option<u64>,

/// 
    #[serde(rename = "TCPTransportPostedReceiveBufferCount")]
    pub tcptransport_posted_receive_buffer_count: Option<u64>,

/// 
    #[serde(rename = "TCPTransportTotalbuffercount")]
    pub tcptransport_totalbuffercount: Option<u64>,

/// 
    #[serde(rename = "TransferpassCPUCap")]
    pub transferpass_cpucap: Option<u64>,

/// 
    #[serde(rename = "TransferpassDirtyPageCount")]
    pub transferpass_dirty_page_count: Option<u64>,

/// 
    #[serde(rename = "TransferPassIsblackout")]
    pub transfer_pass_isblackout: Option<u64>,

/// 
    #[serde(rename = "TransferPassNumber")]
    pub transfer_pass_number: Option<u64>,
}

impl Win32_PerfFormattedData_LmPerfProvider_HyperVVMLiveMigration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            compressor_bytestobe_compressed: None,
            compressor_compressed_bytes_sent: None,
            compressor_compressed_bytes_sent_persec: None,
            compressor_enabled_threads: None,
            compressor_maximum_threads: None,
            memory_walker_bytes_read_persec: None,
            memory_walker_bytes_sentfor_compression: None,
            memory_walker_bytes_sentfor_compression_persec: None,
            memory_walker_maximum_threads: None,
            memory_walker_uncompressed_bytes_sent: None,
            memory_walker_uncompressed_bytes_sent_persec: None,
            receiver_bytes_pending_decompression: None,
            receiver_bytes_pending_write: None,
            receiver_bytes_written_persec: None,
            receiver_compressed_bytes_received_persec: None,
            receiver_decompressed_bytes_persec: None,
            receiver_maximum_threadpool_thread_count: None,
            receiver_uncompressed_bytes_received_persec: None,
            smbtransport_bytes_sent: None,
            smbtransport_bytes_sent_persec: None,
            smbtransport_pending_send_bytes: None,
            smbtransport_pending_send_count: None,
            tcptransport_bytes_pending_processing: None,
            tcptransport_bytes_pending_send: None,
            tcptransport_bytes_received_persec: None,
            tcptransport_bytes_sent_persec: None,
            tcptransport_pending_send_count: None,
            tcptransport_posted_receive_buffer_count: None,
            tcptransport_totalbuffercount: None,
            transferpass_cpucap: None,
            transferpass_dirty_page_count: None,
            transfer_pass_isblackout: None,
            transfer_pass_number: None,
        }
    }


    /// Sets the value of CompressorBytestobeCompressed
    pub fn set_compressor_bytestobe_compressed(&mut self, value: u64) {
        self.compressor_bytestobe_compressed = Some(value);
    }

    /// Gets the value of CompressorBytestobeCompressed
    pub fn get_compressor_bytestobe_compressed(&self) -> Option<&u64> {
        self.compressor_bytestobe_compressed.as_ref()
    }

    /// Sets the value of CompressorCompressedBytesSent
    pub fn set_compressor_compressed_bytes_sent(&mut self, value: u64) {
        self.compressor_compressed_bytes_sent = Some(value);
    }

    /// Gets the value of CompressorCompressedBytesSent
    pub fn get_compressor_compressed_bytes_sent(&self) -> Option<&u64> {
        self.compressor_compressed_bytes_sent.as_ref()
    }

    /// Sets the value of CompressorCompressedBytesSentPersec
    pub fn set_compressor_compressed_bytes_sent_persec(&mut self, value: u64) {
        self.compressor_compressed_bytes_sent_persec = Some(value);
    }

    /// Gets the value of CompressorCompressedBytesSentPersec
    pub fn get_compressor_compressed_bytes_sent_persec(&self) -> Option<&u64> {
        self.compressor_compressed_bytes_sent_persec.as_ref()
    }

    /// Sets the value of CompressorEnabledThreads
    pub fn set_compressor_enabled_threads(&mut self, value: u64) {
        self.compressor_enabled_threads = Some(value);
    }

    /// Gets the value of CompressorEnabledThreads
    pub fn get_compressor_enabled_threads(&self) -> Option<&u64> {
        self.compressor_enabled_threads.as_ref()
    }

    /// Sets the value of CompressorMaximumThreads
    pub fn set_compressor_maximum_threads(&mut self, value: u64) {
        self.compressor_maximum_threads = Some(value);
    }

    /// Gets the value of CompressorMaximumThreads
    pub fn get_compressor_maximum_threads(&self) -> Option<&u64> {
        self.compressor_maximum_threads.as_ref()
    }

    /// Sets the value of MemoryWalkerBytesReadPersec
    pub fn set_memory_walker_bytes_read_persec(&mut self, value: u64) {
        self.memory_walker_bytes_read_persec = Some(value);
    }

    /// Gets the value of MemoryWalkerBytesReadPersec
    pub fn get_memory_walker_bytes_read_persec(&self) -> Option<&u64> {
        self.memory_walker_bytes_read_persec.as_ref()
    }

    /// Sets the value of MemoryWalkerBytesSentforCompression
    pub fn set_memory_walker_bytes_sentfor_compression(&mut self, value: u64) {
        self.memory_walker_bytes_sentfor_compression = Some(value);
    }

    /// Gets the value of MemoryWalkerBytesSentforCompression
    pub fn get_memory_walker_bytes_sentfor_compression(&self) -> Option<&u64> {
        self.memory_walker_bytes_sentfor_compression.as_ref()
    }

    /// Sets the value of MemoryWalkerBytesSentforCompressionPersec
    pub fn set_memory_walker_bytes_sentfor_compression_persec(&mut self, value: u64) {
        self.memory_walker_bytes_sentfor_compression_persec = Some(value);
    }

    /// Gets the value of MemoryWalkerBytesSentforCompressionPersec
    pub fn get_memory_walker_bytes_sentfor_compression_persec(&self) -> Option<&u64> {
        self.memory_walker_bytes_sentfor_compression_persec.as_ref()
    }

    /// Sets the value of MemoryWalkerMaximumThreads
    pub fn set_memory_walker_maximum_threads(&mut self, value: u64) {
        self.memory_walker_maximum_threads = Some(value);
    }

    /// Gets the value of MemoryWalkerMaximumThreads
    pub fn get_memory_walker_maximum_threads(&self) -> Option<&u64> {
        self.memory_walker_maximum_threads.as_ref()
    }

    /// Sets the value of MemoryWalkerUncompressedBytesSent
    pub fn set_memory_walker_uncompressed_bytes_sent(&mut self, value: u64) {
        self.memory_walker_uncompressed_bytes_sent = Some(value);
    }

    /// Gets the value of MemoryWalkerUncompressedBytesSent
    pub fn get_memory_walker_uncompressed_bytes_sent(&self) -> Option<&u64> {
        self.memory_walker_uncompressed_bytes_sent.as_ref()
    }

    /// Sets the value of MemoryWalkerUncompressedBytesSentPersec
    pub fn set_memory_walker_uncompressed_bytes_sent_persec(&mut self, value: u64) {
        self.memory_walker_uncompressed_bytes_sent_persec = Some(value);
    }

    /// Gets the value of MemoryWalkerUncompressedBytesSentPersec
    pub fn get_memory_walker_uncompressed_bytes_sent_persec(&self) -> Option<&u64> {
        self.memory_walker_uncompressed_bytes_sent_persec.as_ref()
    }

    /// Sets the value of ReceiverBytesPendingDecompression
    pub fn set_receiver_bytes_pending_decompression(&mut self, value: u64) {
        self.receiver_bytes_pending_decompression = Some(value);
    }

    /// Gets the value of ReceiverBytesPendingDecompression
    pub fn get_receiver_bytes_pending_decompression(&self) -> Option<&u64> {
        self.receiver_bytes_pending_decompression.as_ref()
    }

    /// Sets the value of ReceiverBytesPendingWrite
    pub fn set_receiver_bytes_pending_write(&mut self, value: u64) {
        self.receiver_bytes_pending_write = Some(value);
    }

    /// Gets the value of ReceiverBytesPendingWrite
    pub fn get_receiver_bytes_pending_write(&self) -> Option<&u64> {
        self.receiver_bytes_pending_write.as_ref()
    }

    /// Sets the value of ReceiverBytesWrittenPersec
    pub fn set_receiver_bytes_written_persec(&mut self, value: u64) {
        self.receiver_bytes_written_persec = Some(value);
    }

    /// Gets the value of ReceiverBytesWrittenPersec
    pub fn get_receiver_bytes_written_persec(&self) -> Option<&u64> {
        self.receiver_bytes_written_persec.as_ref()
    }

    /// Sets the value of ReceiverCompressedBytesReceivedPersec
    pub fn set_receiver_compressed_bytes_received_persec(&mut self, value: u64) {
        self.receiver_compressed_bytes_received_persec = Some(value);
    }

    /// Gets the value of ReceiverCompressedBytesReceivedPersec
    pub fn get_receiver_compressed_bytes_received_persec(&self) -> Option<&u64> {
        self.receiver_compressed_bytes_received_persec.as_ref()
    }

    /// Sets the value of ReceiverDecompressedBytesPersec
    pub fn set_receiver_decompressed_bytes_persec(&mut self, value: u64) {
        self.receiver_decompressed_bytes_persec = Some(value);
    }

    /// Gets the value of ReceiverDecompressedBytesPersec
    pub fn get_receiver_decompressed_bytes_persec(&self) -> Option<&u64> {
        self.receiver_decompressed_bytes_persec.as_ref()
    }

    /// Sets the value of ReceiverMaximumThreadpoolThreadCount
    pub fn set_receiver_maximum_threadpool_thread_count(&mut self, value: u64) {
        self.receiver_maximum_threadpool_thread_count = Some(value);
    }

    /// Gets the value of ReceiverMaximumThreadpoolThreadCount
    pub fn get_receiver_maximum_threadpool_thread_count(&self) -> Option<&u64> {
        self.receiver_maximum_threadpool_thread_count.as_ref()
    }

    /// Sets the value of ReceiverUncompressedBytesReceivedPersec
    pub fn set_receiver_uncompressed_bytes_received_persec(&mut self, value: u64) {
        self.receiver_uncompressed_bytes_received_persec = Some(value);
    }

    /// Gets the value of ReceiverUncompressedBytesReceivedPersec
    pub fn get_receiver_uncompressed_bytes_received_persec(&self) -> Option<&u64> {
        self.receiver_uncompressed_bytes_received_persec.as_ref()
    }

    /// Sets the value of SMBTransportBytesSent
    pub fn set_smbtransport_bytes_sent(&mut self, value: u64) {
        self.smbtransport_bytes_sent = Some(value);
    }

    /// Gets the value of SMBTransportBytesSent
    pub fn get_smbtransport_bytes_sent(&self) -> Option<&u64> {
        self.smbtransport_bytes_sent.as_ref()
    }

    /// Sets the value of SMBTransportBytesSentPersec
    pub fn set_smbtransport_bytes_sent_persec(&mut self, value: u64) {
        self.smbtransport_bytes_sent_persec = Some(value);
    }

    /// Gets the value of SMBTransportBytesSentPersec
    pub fn get_smbtransport_bytes_sent_persec(&self) -> Option<&u64> {
        self.smbtransport_bytes_sent_persec.as_ref()
    }

    /// Sets the value of SMBTransportPendingSendBytes
    pub fn set_smbtransport_pending_send_bytes(&mut self, value: u64) {
        self.smbtransport_pending_send_bytes = Some(value);
    }

    /// Gets the value of SMBTransportPendingSendBytes
    pub fn get_smbtransport_pending_send_bytes(&self) -> Option<&u64> {
        self.smbtransport_pending_send_bytes.as_ref()
    }

    /// Sets the value of SMBTransportPendingSendCount
    pub fn set_smbtransport_pending_send_count(&mut self, value: u64) {
        self.smbtransport_pending_send_count = Some(value);
    }

    /// Gets the value of SMBTransportPendingSendCount
    pub fn get_smbtransport_pending_send_count(&self) -> Option<&u64> {
        self.smbtransport_pending_send_count.as_ref()
    }

    /// Sets the value of TCPTransportBytesPendingProcessing
    pub fn set_tcptransport_bytes_pending_processing(&mut self, value: u64) {
        self.tcptransport_bytes_pending_processing = Some(value);
    }

    /// Gets the value of TCPTransportBytesPendingProcessing
    pub fn get_tcptransport_bytes_pending_processing(&self) -> Option<&u64> {
        self.tcptransport_bytes_pending_processing.as_ref()
    }

    /// Sets the value of TCPTransportBytesPendingSend
    pub fn set_tcptransport_bytes_pending_send(&mut self, value: u64) {
        self.tcptransport_bytes_pending_send = Some(value);
    }

    /// Gets the value of TCPTransportBytesPendingSend
    pub fn get_tcptransport_bytes_pending_send(&self) -> Option<&u64> {
        self.tcptransport_bytes_pending_send.as_ref()
    }

    /// Sets the value of TCPTransportBytesReceivedPersec
    pub fn set_tcptransport_bytes_received_persec(&mut self, value: u64) {
        self.tcptransport_bytes_received_persec = Some(value);
    }

    /// Gets the value of TCPTransportBytesReceivedPersec
    pub fn get_tcptransport_bytes_received_persec(&self) -> Option<&u64> {
        self.tcptransport_bytes_received_persec.as_ref()
    }

    /// Sets the value of TCPTransportBytesSentPersec
    pub fn set_tcptransport_bytes_sent_persec(&mut self, value: u64) {
        self.tcptransport_bytes_sent_persec = Some(value);
    }

    /// Gets the value of TCPTransportBytesSentPersec
    pub fn get_tcptransport_bytes_sent_persec(&self) -> Option<&u64> {
        self.tcptransport_bytes_sent_persec.as_ref()
    }

    /// Sets the value of TCPTransportPendingSendCount
    pub fn set_tcptransport_pending_send_count(&mut self, value: u64) {
        self.tcptransport_pending_send_count = Some(value);
    }

    /// Gets the value of TCPTransportPendingSendCount
    pub fn get_tcptransport_pending_send_count(&self) -> Option<&u64> {
        self.tcptransport_pending_send_count.as_ref()
    }

    /// Sets the value of TCPTransportPostedReceiveBufferCount
    pub fn set_tcptransport_posted_receive_buffer_count(&mut self, value: u64) {
        self.tcptransport_posted_receive_buffer_count = Some(value);
    }

    /// Gets the value of TCPTransportPostedReceiveBufferCount
    pub fn get_tcptransport_posted_receive_buffer_count(&self) -> Option<&u64> {
        self.tcptransport_posted_receive_buffer_count.as_ref()
    }

    /// Sets the value of TCPTransportTotalbuffercount
    pub fn set_tcptransport_totalbuffercount(&mut self, value: u64) {
        self.tcptransport_totalbuffercount = Some(value);
    }

    /// Gets the value of TCPTransportTotalbuffercount
    pub fn get_tcptransport_totalbuffercount(&self) -> Option<&u64> {
        self.tcptransport_totalbuffercount.as_ref()
    }

    /// Sets the value of TransferpassCPUCap
    pub fn set_transferpass_cpucap(&mut self, value: u64) {
        self.transferpass_cpucap = Some(value);
    }

    /// Gets the value of TransferpassCPUCap
    pub fn get_transferpass_cpucap(&self) -> Option<&u64> {
        self.transferpass_cpucap.as_ref()
    }

    /// Sets the value of TransferpassDirtyPageCount
    pub fn set_transferpass_dirty_page_count(&mut self, value: u64) {
        self.transferpass_dirty_page_count = Some(value);
    }

    /// Gets the value of TransferpassDirtyPageCount
    pub fn get_transferpass_dirty_page_count(&self) -> Option<&u64> {
        self.transferpass_dirty_page_count.as_ref()
    }

    /// Sets the value of TransferPassIsblackout
    pub fn set_transfer_pass_isblackout(&mut self, value: u64) {
        self.transfer_pass_isblackout = Some(value);
    }

    /// Gets the value of TransferPassIsblackout
    pub fn get_transfer_pass_isblackout(&self) -> Option<&u64> {
        self.transfer_pass_isblackout.as_ref()
    }

    /// Sets the value of TransferPassNumber
    pub fn set_transfer_pass_number(&mut self, value: u64) {
        self.transfer_pass_number = Some(value);
    }

    /// Gets the value of TransferPassNumber
    pub fn get_transfer_pass_number(&self) -> Option<&u64> {
        self.transfer_pass_number.as_ref()
    }
}


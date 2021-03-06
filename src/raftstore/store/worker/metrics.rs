// Copyright 2016 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use prometheus::{CounterVec, HistogramVec, Histogram, exponential_buckets};

lazy_static! {
    pub static ref PD_REQ_COUNTER_VEC: CounterVec =
        register_counter_vec!(
            "tikv_raftstore_pd_request_sent_total",
            "Total number of pd client request sent.",
            &["type", "status"]
        ).unwrap();

    pub static ref PD_HEARTBEAT_COUNTER_VEC: CounterVec =
        register_counter_vec!(
            "tikv_raftstore_pd_heartbeat_sent_total",
            "Total number of raftstore pd client heartbeat sent.",
            &["type"]
        ).unwrap();

    pub static ref PD_VALIDATE_PEER_COUNTER_VEC: CounterVec =
        register_counter_vec!(
            "tikv_raftstore_pd_validate_peer_total",
            "Total number of raftstore pd worker validate peer task.",
            &["type"]
        ).unwrap();

    pub static ref SNAP_COUNTER_VEC: CounterVec =
        register_counter_vec!(
            "tikv_raftstore_snapshot_total",
            "Total number of raftstore snapshot processed.",
            &["type", "status"]
        ).unwrap();

    pub static ref CHECK_SPILT_COUNTER_VEC: CounterVec =
        register_counter_vec!(
            "tikv_raftstore_check_split_total",
            "Total number of raftstore split check.",
            &["type"]
        ).unwrap();

    pub static ref SNAP_HISTOGRAM: HistogramVec =
        register_histogram_vec!(
            "tikv_raftstore_snapshot_duration_seconds",
            "Bucketed histogram of raftstore snapshot process duration",
            &["type"]
        ).unwrap();

    pub static ref CHECK_SPILT_HISTOGRAM: Histogram =
        register_histogram!(
            "tikv_raftstore_check_split_duration_seconds",
            "Bucketed histogram of raftstore split check duration"
        ).unwrap();

    pub static ref COMPACT_RANGE_CF: HistogramVec =
        register_histogram_vec!(
            "tikv_compact_range_cf_duration_seconds",
            "Bucketed histogram of compact range for cf execution",
            &["cf"]
        ).unwrap();

    pub static ref REGION_HASH_HISTOGRAM: Histogram =
        register_histogram!(
            "tikv_raftstore_hash_duration_seconds",
            "Bucketed histogram of raftstore hash compution duration"
        ).unwrap();

    pub static ref SNAPSHOT_KV_COUNT_HISTOGRAM: Histogram =
        register_histogram!(
            "tikv_snapshot_kv_count",
            "Total number of kv in snapshot",
             exponential_buckets(100.0, 2.0, 20).unwrap() //100,100*2^1,...100M
        ).unwrap();
    pub static ref SNAPSHOT_SIZE_HISTOGRAM: Histogram =
        register_histogram!(
            "tikv_snapshot_size",
            "Size of snapshot",
             exponential_buckets(1024.0, 2.0, 22).unwrap() // 1024,1024*2^1,..,4G
        ).unwrap();
}

// Copyright (C) Parity Technologies and the various Polkadot contributors, see Contributions.md
// for a list of specific contributors.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `snowbridge_pallet_inbound_queue`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 47.2.0
//! DATE: 2025-06-15, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// frame-omni-bencher
// v1
// benchmark
// pallet
// --runtime=target/production/wbuild/bridge-hub-polkadot-runtime/bridge_hub_polkadot_runtime.compact.compressed.wasm
// --header=.github/scripts/cmd/file_header.txt
// --output=./system-parachains/bridge-hubs/bridge-hub-polkadot/src/weights/
// --all
// --quiet

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `snowbridge_pallet_inbound_queue`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> snowbridge_pallet_inbound_queue::WeightInfo for WeightInfo<T> {
	/// Storage: `EthereumInboundQueue::OperatingMode` (r:1 w:0)
	/// Proof: `EthereumInboundQueue::OperatingMode` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::LatestFinalizedBlockRoot` (r:1 w:0)
	/// Proof: `EthereumBeaconClient::LatestFinalizedBlockRoot` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::FinalizedBeaconState` (r:1 w:0)
	/// Proof: `EthereumBeaconClient::FinalizedBeaconState` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xaed97c7854d601808b98ae43079dafb3` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xaed97c7854d601808b98ae43079dafb3` (r:1 w:0)
	/// Storage: `EthereumSystem::Channels` (r:1 w:0)
	/// Proof: `EthereumSystem::Channels` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	/// Storage: `EthereumInboundQueue::Nonce` (r:1 w:1)
	/// Proof: `EthereumInboundQueue::Nonce` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `EthereumSystem::PricingParameters` (r:1 w:0)
	/// Proof: `EthereumSystem::PricingParameters` (`max_values`: Some(1), `max_size`: Some(112), added: 607, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `625`
		//  Estimated: `4090`
		// Minimum execution time: 199_150_000 picoseconds.
		Weight::from_parts(200_231_000, 0)
			.saturating_add(Weight::from_parts(0, 4090))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

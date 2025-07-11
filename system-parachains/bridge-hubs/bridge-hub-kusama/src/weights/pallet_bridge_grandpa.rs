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

//! Autogenerated weights for `pallet_bridge_grandpa`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 47.2.0
//! DATE: 2025-06-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// frame-omni-bencher
// v1
// benchmark
// pallet
// --runtime=target/production/wbuild/bridge-hub-kusama-runtime/bridge_hub_kusama_runtime.compact.compressed.wasm
// --header=.github/scripts/cmd/file_header.txt
// --output=./system-parachains/bridge-hubs/bridge-hub-kusama/src/weights/
// --all
// --quiet

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_bridge_grandpa`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_grandpa::WeightInfo for WeightInfo<T> {
	/// Storage: `BridgePolkadotGrandpa::CurrentAuthoritySet` (r:1 w:0)
	/// Proof: `BridgePolkadotGrandpa::CurrentAuthoritySet` (`max_values`: Some(1), `max_size`: Some(50250), added: 50745, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotGrandpa::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotGrandpa::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotGrandpa::BestFinalized` (r:1 w:1)
	/// Proof: `BridgePolkadotGrandpa::BestFinalized` (`max_values`: Some(1), `max_size`: Some(36), added: 531, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotGrandpa::ImportedHashesPointer` (r:1 w:1)
	/// Proof: `BridgePolkadotGrandpa::ImportedHashesPointer` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotGrandpa::ImportedHashes` (r:1 w:1)
	/// Proof: `BridgePolkadotGrandpa::ImportedHashes` (`max_values`: Some(1200), `max_size`: Some(36), added: 1521, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotGrandpa::ImportedHeaders` (r:0 w:2)
	/// Proof: `BridgePolkadotGrandpa::ImportedHeaders` (`max_values`: Some(1200), `max_size`: Some(68), added: 1553, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 168]`.
	/// The range of component `v` is `[50, 100]`.
	fn submit_finality_proof(p: u32, v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `31 + p * (60 ±0)`
		//  Estimated: `51735`
		// Minimum execution time: 366_932_000 picoseconds.
		Weight::from_parts(20_510_107, 0)
			.saturating_add(Weight::from_parts(0, 51735))
			// Standard Error: 4_274
			.saturating_add(Weight::from_parts(49_987_235, 0).saturating_mul(p.into()))
			// Standard Error: 14_262
			.saturating_add(Weight::from_parts(2_978_795, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `BridgePolkadotGrandpa::CurrentAuthoritySet` (r:1 w:1)
	/// Proof: `BridgePolkadotGrandpa::CurrentAuthoritySet` (`max_values`: Some(1), `max_size`: Some(50250), added: 50745, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotGrandpa::ImportedHashesPointer` (r:1 w:1)
	/// Proof: `BridgePolkadotGrandpa::ImportedHashesPointer` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotGrandpa::ImportedHashes` (r:1 w:1)
	/// Proof: `BridgePolkadotGrandpa::ImportedHashes` (`max_values`: Some(1200), `max_size`: Some(36), added: 1521, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotGrandpa::BestFinalized` (r:0 w:1)
	/// Proof: `BridgePolkadotGrandpa::BestFinalized` (`max_values`: Some(1), `max_size`: Some(36), added: 531, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotGrandpa::ImportedHeaders` (r:0 w:2)
	/// Proof: `BridgePolkadotGrandpa::ImportedHeaders` (`max_values`: Some(1200), `max_size`: Some(68), added: 1553, mode: `MaxEncodedLen`)
	fn force_set_pallet_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `84`
		//  Estimated: `51735`
		// Minimum execution time: 104_191_000 picoseconds.
		Weight::from_parts(106_340_000, 0)
			.saturating_add(Weight::from_parts(0, 51735))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
	}
}

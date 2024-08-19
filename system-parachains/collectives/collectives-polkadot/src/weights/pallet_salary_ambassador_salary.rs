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

//! Autogenerated weights for `pallet_salary`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-15, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("./collectives-polkadot-chain-spec.json")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --chain=./collectives-polkadot-chain-spec.json
// --steps=50
// --repeat=20
// --pallet=pallet_salary
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./collectives-polkadot-weights/
// --header=./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_salary`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_salary::WeightInfo for WeightInfo<T> {
	/// Storage: `AmbassadorSalary::Status` (r:1 w:1)
	/// Proof: `AmbassadorSalary::Status` (`max_values`: Some(1), `max_size`: Some(56), added: 551, mode: `MaxEncodedLen`)
	fn init() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1541`
		// Minimum execution time: 8_310_000 picoseconds.
		Weight::from_parts(8_630_000, 0)
			.saturating_add(Weight::from_parts(0, 1541))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorSalary::Status` (r:1 w:1)
	/// Proof: `AmbassadorSalary::Status` (`max_values`: Some(1), `max_size`: Some(56), added: 551, mode: `MaxEncodedLen`)
	fn bump() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `191`
		//  Estimated: `1541`
		// Minimum execution time: 10_920_000 picoseconds.
		Weight::from_parts(11_230_000, 0)
			.saturating_add(Weight::from_parts(0, 1541))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorSalary::Status` (r:1 w:0)
	/// Proof: `AmbassadorSalary::Status` (`max_values`: Some(1), `max_size`: Some(56), added: 551, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::Members` (r:1 w:0)
	/// Proof: `AmbassadorCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorSalary::Claimant` (r:1 w:1)
	/// Proof: `AmbassadorSalary::Claimant` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	fn induct() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `400`
		//  Estimated: `3551`
		// Minimum execution time: 19_540_000 picoseconds.
		Weight::from_parts(20_220_000, 0)
			.saturating_add(Weight::from_parts(0, 3551))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorCollective::Members` (r:1 w:0)
	/// Proof: `AmbassadorCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorSalary::Status` (r:1 w:1)
	/// Proof: `AmbassadorSalary::Status` (`max_values`: Some(1), `max_size`: Some(56), added: 551, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorSalary::Claimant` (r:1 w:1)
	/// Proof: `AmbassadorSalary::Claimant` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `467`
		//  Estimated: `3551`
		// Minimum execution time: 23_371_000 picoseconds.
		Weight::from_parts(23_940_000, 0)
			.saturating_add(Weight::from_parts(0, 3551))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorSalary::Status` (r:1 w:1)
	/// Proof: `AmbassadorSalary::Status` (`max_values`: Some(1), `max_size`: Some(56), added: 551, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorSalary::Claimant` (r:1 w:1)
	/// Proof: `AmbassadorSalary::Claimant` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::Members` (r:1 w:0)
	/// Proof: `AmbassadorCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::QueryCounter` (r:1 w:1)
	/// Proof: `PolkadotXcm::QueryCounter` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::DeliveryFeeFactor` (r:1 w:0)
	/// Proof: `XcmpQueue::DeliveryFeeFactor` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::RelevantMessagingState` (r:1 w:0)
	/// Proof: `ParachainSystem::RelevantMessagingState` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: Some(1282), added: 1777, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpMessages` (r:0 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpMessages` (`max_values`: None, `max_size`: Some(105506), added: 107981, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::Queries` (r:0 w:1)
	/// Proof: `PolkadotXcm::Queries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn payout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `904`
		//  Estimated: `4369`
		// Minimum execution time: 70_000_000 picoseconds.
		Weight::from_parts(70_800_000, 0)
			.saturating_add(Weight::from_parts(0, 4369))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `AmbassadorSalary::Status` (r:1 w:1)
	/// Proof: `AmbassadorSalary::Status` (`max_values`: Some(1), `max_size`: Some(56), added: 551, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorSalary::Claimant` (r:1 w:1)
	/// Proof: `AmbassadorSalary::Claimant` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::Members` (r:1 w:0)
	/// Proof: `AmbassadorCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::QueryCounter` (r:1 w:1)
	/// Proof: `PolkadotXcm::QueryCounter` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::DeliveryFeeFactor` (r:1 w:0)
	/// Proof: `XcmpQueue::DeliveryFeeFactor` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::RelevantMessagingState` (r:1 w:0)
	/// Proof: `ParachainSystem::RelevantMessagingState` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: Some(1282), added: 1777, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpMessages` (r:0 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpMessages` (`max_values`: None, `max_size`: Some(105506), added: 107981, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::Queries` (r:0 w:1)
	/// Proof: `PolkadotXcm::Queries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn payout_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `904`
		//  Estimated: `4369`
		// Minimum execution time: 69_401_000 picoseconds.
		Weight::from_parts(70_450_000, 0)
			.saturating_add(Weight::from_parts(0, 4369))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `AmbassadorSalary::Status` (r:1 w:1)
	/// Proof: `AmbassadorSalary::Status` (`max_values`: Some(1), `max_size`: Some(56), added: 551, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorSalary::Claimant` (r:1 w:1)
	/// Proof: `AmbassadorSalary::Claimant` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::Queries` (r:1 w:1)
	/// Proof: `PolkadotXcm::Queries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn check_payment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `466`
		//  Estimated: `3931`
		// Minimum execution time: 25_210_000 picoseconds.
		Weight::from_parts(25_610_000, 0)
			.saturating_add(Weight::from_parts(0, 3931))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use crate::{collectives_send_whitelist, *};
use codec::Encode;
use emulated_integration_tests_common::xcm_emulator::{Chain, TestExt};
use frame_support::{assert_err, assert_ok, dispatch::DispatchResultWithPostInfo};
use polkadot_runtime::governance::pallet_custom_origins::Origin;
use polkadot_system_emulated_network::{
	polkadot_emulated_chain::PolkadotRelayPallet as PolkadotPallet, PolkadotRelay as Polkadot,
};
use sp_core::H256;
use sp_runtime::{
	traits::{Dispatchable, Hash},
	DispatchError,
};

fn store_preimage(call: polkadot_runtime::RuntimeCall) -> H256 {
	Polkadot::execute_with(|| {
		type Runtime = <Polkadot as Chain>::Runtime;
		type Preimage = <Polkadot as PolkadotPallet>::Preimage;

		// get hash and store preimage
		let call_hash = <Runtime as frame_system::Config>::Hashing::hash(call.encode().as_ref());
		// Root is not important here, we could have an account with enough balance also.
		assert_ok!(Preimage::note_preimage(polkadot_runtime::RuntimeOrigin::root(), call.encode()));

		call_hash
	})
}

fn dispatch_whitelisted_call_with_preimage(
	call: polkadot_runtime::RuntimeCall,
	origin: polkadot_runtime::RuntimeOrigin,
) -> DispatchResultWithPostInfo {
	Polkadot::execute_with(|| {
		type Runtime = <Polkadot as Chain>::Runtime;

		// wrap with whitelist call
		let whitelist_call = polkadot_runtime::RuntimeCall::Whitelist(
			pallet_whitelist::Call::<Runtime>::dispatch_whitelisted_call_with_preimage {
				call: Box::new(call),
			},
		);

		whitelist_call.dispatch(origin)
	})
}

#[test]
fn can_authorize_upgrade_for_relaychain() {
	let code_hash = [1u8; 32].into();
	type Runtime = <Polkadot as Chain>::Runtime;

	let authorize_upgrade =
		polkadot_runtime::RuntimeCall::Utility(pallet_utility::Call::<Runtime>::force_batch {
			calls: vec![
				// upgrade the relaychain
				polkadot_runtime::RuntimeCall::System(frame_system::Call::authorize_upgrade {
					code_hash,
				}),
			],
		});

	// bad origin
	let invalid_origin: polkadot_runtime::RuntimeOrigin = Origin::StakingAdmin.into();
	// ok origin
	let ok_origin: polkadot_runtime::RuntimeOrigin = Origin::WhitelistedCaller.into();

	// store preimage
	let call_hash = store_preimage(authorize_upgrade.clone());

	// Err - when dispatch non-whitelisted
	assert_err!(
		dispatch_whitelisted_call_with_preimage(authorize_upgrade.clone(), ok_origin.clone()),
		DispatchError::Module(sp_runtime::ModuleError {
			index: 23,
			error: [3, 0, 0, 0],
			message: Some("CallIsNotWhitelisted")
		})
	);

	// whitelist
	collectives_send_whitelist(Location::parent(), || {
		polkadot_runtime::RuntimeCall::Whitelist(
			pallet_whitelist::Call::<Runtime>::whitelist_call { call_hash },
		)
		.encode()
	});

	// Err - when dispatch wrong origin
	assert_err!(
		dispatch_whitelisted_call_with_preimage(authorize_upgrade.clone(), invalid_origin),
		DispatchError::BadOrigin
	);

	// check before
	Polkadot::execute_with(|| assert!(polkadot_runtime::System::authorized_upgrade().is_none()));

	// ok - authorized
	assert_ok!(dispatch_whitelisted_call_with_preimage(authorize_upgrade, ok_origin));

	// check after - authorized
	Polkadot::execute_with(|| assert!(polkadot_runtime::System::authorized_upgrade().is_some()));
}

#[test]
fn can_authorize_upgrade_for_system_chains() {
	let code_hash = [1u8; 32].into();
	type Runtime = <Polkadot as Chain>::Runtime;

	let asset_hub_location = Polkadot::child_location_of(AssetHubPolkadot::para_id());
	let collectives_location = Polkadot::child_location_of(CollectivesPolkadot::para_id());
	let bridgehub_location = Polkadot::child_location_of(BridgeHubPolkadot::para_id());

	let asset_hub_authorize_upgrade_call =
		asset_hub_polkadot_runtime::RuntimeCall::System(frame_system::Call::authorize_upgrade {
			code_hash,
		});
	let collectives_authorize_upgrade_call =
		collectives_polkadot_runtime::RuntimeCall::System(frame_system::Call::authorize_upgrade {
			code_hash,
		});
	let bridge_hub_authorize_upgrade_call =
		bridge_hub_polkadot_runtime::RuntimeCall::System(frame_system::Call::authorize_upgrade {
			code_hash,
		});

	let authorize_upgrade =
		polkadot_runtime::RuntimeCall::Utility(pallet_utility::Call::<Runtime>::force_batch {
			calls: vec![
				polkadot_runtime::RuntimeCall::XcmPallet(pallet_xcm::Call::send {
					dest: bx!(VersionedLocation::from(asset_hub_location)),
					message: bx!(VersionedXcm::from(Xcm(vec![
						UnpaidExecution { weight_limit: Unlimited, check_origin: None },
						Transact {
							origin_kind: OriginKind::Superuser,
							require_weight_at_most: Weight::from_parts(5_000_000_000, 500_000),
							call: asset_hub_authorize_upgrade_call.encode().into(),
						}
					]))),
				}),
				polkadot_runtime::RuntimeCall::XcmPallet(pallet_xcm::Call::send {
					dest: bx!(VersionedLocation::from(collectives_location)),
					message: bx!(VersionedXcm::from(Xcm(vec![
						UnpaidExecution { weight_limit: Unlimited, check_origin: None },
						Transact {
							origin_kind: OriginKind::Superuser,
							require_weight_at_most: Weight::from_parts(5_000_000_000, 500_000),
							call: collectives_authorize_upgrade_call.encode().into(),
						}
					]))),
				}),
				polkadot_runtime::RuntimeCall::XcmPallet(pallet_xcm::Call::send {
					dest: bx!(VersionedLocation::from(bridgehub_location)),
					message: bx!(VersionedXcm::from(Xcm(vec![
						UnpaidExecution { weight_limit: Unlimited, check_origin: None },
						Transact {
							origin_kind: OriginKind::Superuser,
							require_weight_at_most: Weight::from_parts(5_000_000_000, 500_000),
							call: bridge_hub_authorize_upgrade_call.encode().into(),
						}
					]))),
				}),
			],
		});

	// bad origin
	let invalid_origin: polkadot_runtime::RuntimeOrigin = Origin::StakingAdmin.into();
	// ok origin
	let ok_origin: polkadot_runtime::RuntimeOrigin = Origin::WhitelistedCaller.into();

	// store preimage
	let call_hash = store_preimage(authorize_upgrade.clone());

	// Err - when dispatch non-whitelisted
	assert_err!(
		dispatch_whitelisted_call_with_preimage(authorize_upgrade.clone(), ok_origin.clone()),
		DispatchError::Module(sp_runtime::ModuleError {
			index: 23,
			error: [3, 0, 0, 0],
			message: Some("CallIsNotWhitelisted")
		})
	);

	// whitelist
	collectives_send_whitelist(Location::parent(), || {
		polkadot_runtime::RuntimeCall::Whitelist(
			pallet_whitelist::Call::<Runtime>::whitelist_call { call_hash },
		)
		.encode()
	});

	// Err - when dispatch wrong origin
	assert_err!(
		dispatch_whitelisted_call_with_preimage(authorize_upgrade.clone(), invalid_origin),
		DispatchError::BadOrigin
	);

	// check before
	AssetHubPolkadot::execute_with(|| {
		assert!(asset_hub_polkadot_runtime::System::authorized_upgrade().is_none())
	});
	CollectivesPolkadot::execute_with(|| {
		assert!(collectives_polkadot_runtime::System::authorized_upgrade().is_none())
	});
	BridgeHubPolkadot::execute_with(|| {
		assert!(bridge_hub_polkadot_runtime::System::authorized_upgrade().is_none())
	});

	// ok - authorized
	assert_ok!(dispatch_whitelisted_call_with_preimage(authorize_upgrade, ok_origin));

	// check after - authorized
	AssetHubPolkadot::execute_with(|| {
		assert!(asset_hub_polkadot_runtime::System::authorized_upgrade().is_some())
	});
	CollectivesPolkadot::execute_with(|| {
		assert!(collectives_polkadot_runtime::System::authorized_upgrade().is_some())
	});
	BridgeHubPolkadot::execute_with(|| {
		assert!(bridge_hub_polkadot_runtime::System::authorized_upgrade().is_some())
	});
}

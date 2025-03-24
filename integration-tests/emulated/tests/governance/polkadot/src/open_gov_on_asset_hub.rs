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
use crate::*;
use asset_hub_polkadot_runtime::governance::pallet_custom_origins::Origin;
use codec::Encode;
use emulated_integration_tests_common::{
	impls::RelayChain,
	xcm_emulator::{Chain, Parachain, TestExt},
};
use frame_support::{assert_err, assert_ok, dispatch::DispatchResultWithPostInfo};
use polkadot_system_emulated_network::{
	asset_hub_polkadot_emulated_chain::AssetHubPolkadotParaPallet as AssetHubPallet,
	polkadot_emulated_chain::PolkadotRelayPallet as PolkadotPallet,
	AssetHubPolkadotPara as AssetHubPolkadot, BridgeHubPolkadotPara as BridgeHubPolkadot,
	CoretimePolkadotPara as CoretimePolkadot, PeoplePolkadotPara as PeoplePolkadot,
	PolkadotRelay as Polkadot,
};
use sp_core::H256;
use sp_runtime::{
	traits::{Dispatchable, Hash},
	DispatchError,
};

fn store_preimage(call: asset_hub_polkadot_runtime::RuntimeCall) -> H256 {
	AssetHubPolkadot::execute_with(|| {
		type Runtime = <AssetHubPolkadot as Chain>::Runtime;
		type Preimage = <AssetHubPolkadot as AssetHubPallet>::Preimage;

		// get hash and store preimage
		let call_hash = <Runtime as frame_system::Config>::Hashing::hash(call.encode().as_ref());
		// Root is not important here, we could have an account with enough balance also.
		assert_ok!(Preimage::note_preimage(
			asset_hub_polkadot_runtime::RuntimeOrigin::root(),
			call.encode()
		));

		call_hash
	})
}

fn dispatch_whitelisted_call_with_preimage(
	call: asset_hub_polkadot_runtime::RuntimeCall,
	origin: asset_hub_polkadot_runtime::RuntimeOrigin,
) -> DispatchResultWithPostInfo {
	AssetHubPolkadot::execute_with(|| {
		type Runtime = <AssetHubPolkadot as Chain>::Runtime;
		type RuntimeCall = <AssetHubPolkadot as Chain>::RuntimeCall;

		// wrap with whitelist call
		let whitelist_call = RuntimeCall::Whitelist(
			pallet_whitelist::Call::<Runtime>::dispatch_whitelisted_call_with_preimage {
				call: Box::new(call),
			},
		);

		whitelist_call.dispatch(origin)
	})
}

fn build_xcm_send_authorize_upgrade_call<R: frame_system::Config<Hash = H256>>(
) -> <AssetHubPolkadot as Chain>::RuntimeCall {
	let code_hash = [1u8; 32].into();
	let location = AssetHubPolkadot::parent_location();
	// TODO: calculate real weight
	let weight = Weight::from_parts(5_000_000_000, 500_000);

	let call: R::RuntimeCall = frame_system::Call::<R>::authorize_upgrade { code_hash }.into();

	<AssetHubPolkadot as Chain>::RuntimeCall::PolkadotXcm(pallet_xcm::Call::send {
		dest: bx!(VersionedLocation::from(location)),
		message: bx!(VersionedXcm::from(Xcm(vec![
			UnpaidExecution { weight_limit: Unlimited, check_origin: None },
			Transact {
				origin_kind: OriginKind::Superuser,
				require_weight_at_most: weight,
				call: call.encode().into(),
			}
		]))),
	})
}

#[test]
fn assethub_can_authorize_upgrade_for_itself() {
	let code_hash = [1u8; 32].into();
	type AssetHubRuntime = <AssetHubPolkadot as Chain>::Runtime;
	type AssetHubRuntimeCall = <AssetHubPolkadot as Chain>::RuntimeCall;
	type AssetHubRuntimeOrigin = <AssetHubPolkadot as Chain>::RuntimeOrigin;

	let authorize_upgrade =
		AssetHubRuntimeCall::Utility(pallet_utility::Call::<AssetHubRuntime>::force_batch {
			calls: vec![
				// upgrade the relaychain
				AssetHubRuntimeCall::System(frame_system::Call::authorize_upgrade { code_hash }),
			],
		});

	// bad origin
	let invalid_origin: AssetHubRuntimeOrigin = Origin::StakingAdmin.into();
	// ok origin
	let ok_origin: AssetHubRuntimeOrigin = Origin::WhitelistedCaller.into();

	// store preimage
	let call_hash = store_preimage(authorize_upgrade.clone());

	// Err - when dispatch non-whitelisted
	assert_err!(
		dispatch_whitelisted_call_with_preimage(authorize_upgrade.clone(), ok_origin.clone()),
		DispatchError::Module(sp_runtime::ModuleError {
			index: 64,
			error: [3, 0, 0, 0],
			message: Some("CallIsNotWhitelisted")
		})
	);

	// whitelist
	collectives_send_whitelist(
		CollectivesPolkadot::sibling_location_of(<AssetHubPolkadot as Parachain>::para_id()),
		|| {
			AssetHubRuntimeCall::Whitelist(
				pallet_whitelist::Call::<AssetHubRuntime>::whitelist_call { call_hash },
			)
			.encode()
		},
	);

	// Err - when dispatch wrong origin
	assert_err!(
		dispatch_whitelisted_call_with_preimage(authorize_upgrade.clone(), invalid_origin),
		DispatchError::BadOrigin
	);

	// check before
	AssetHubPolkadot::execute_with(|| {
		assert!(asset_hub_polkadot_runtime::System::authorized_upgrade().is_none())
	});

	// ok - authorized
	assert_ok!(dispatch_whitelisted_call_with_preimage(authorize_upgrade, ok_origin));

	// check after - authorized
	AssetHubPolkadot::execute_with(|| {
		assert!(asset_hub_polkadot_runtime::System::authorized_upgrade().is_some())
	});
}

#[test]
fn assethub_can_authorize_upgrade_for_polkadot() {
	type AssetHubRuntime = <AssetHubPolkadot as Chain>::Runtime;
	type AssetHubRuntimeCall = <AssetHubPolkadot as Chain>::RuntimeCall;
	type AssetHubRuntimeOrigin = <AssetHubPolkadot as Chain>::RuntimeOrigin;

	let authorize_upgrade =
		AssetHubRuntimeCall::Utility(pallet_utility::Call::<AssetHubRuntime>::force_batch {
			calls: vec![build_xcm_send_authorize_upgrade_call::<<Polkadot as Chain>::Runtime>()],
		});

	// bad origin
	let invalid_origin: AssetHubRuntimeOrigin = Origin::StakingAdmin.into();
	// ok origin
	let ok_origin: AssetHubRuntimeOrigin = Origin::WhitelistedCaller.into();

	// store preimage
	let call_hash = store_preimage(authorize_upgrade.clone());

	// Err - when dispatch non-whitelisted
	assert_err!(
		dispatch_whitelisted_call_with_preimage(authorize_upgrade.clone(), ok_origin.clone()),
		DispatchError::Module(sp_runtime::ModuleError {
			index: 64,
			error: [3, 0, 0, 0],
			message: Some("CallIsNotWhitelisted")
		})
	);

	// whitelist
	collectives_send_whitelist(
		CollectivesPolkadot::sibling_location_of(<AssetHubPolkadot as Parachain>::para_id()),
		|| {
			AssetHubRuntimeCall::Whitelist(
				pallet_whitelist::Call::<AssetHubRuntime>::whitelist_call { call_hash },
			)
			.encode()
		},
	);

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

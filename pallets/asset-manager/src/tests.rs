// Copyright 2021 Parallel Finance Developer.
// This file is part of Parallel Finance.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Tests for AssetManager Pallet
use crate::*;
use mock::*;

use frame_support::{
    assert_noop, assert_ok,
    storage::migration::{put_storage_value, storage_key_iter},
    Blake2_128Concat,
};
use xcm::latest::prelude::*;

#[test]
fn registering_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(AssetManager::register_asset(
            Origin::root(),
            MockAssetType::MockAsset(1),
            None,
            Some(0u32.into()),
            1u32.into(),
            true
        ));

        assert_eq!(
            AssetManager::asset_id_type(1).unwrap(),
            MockAssetType::MockAsset(1)
        );
        assert_eq!(
            AssetManager::asset_type_id(MockAssetType::MockAsset(1)).unwrap(),
            1
        );
        expect_events(vec![crate::Event::AssetRegistered {
            asset_id: 1,
            asset: MockAssetType::MockAsset(1),
        }])
    });
}

#[test]
fn test_asset_exists_error() {
    new_test_ext().execute_with(|| {
        assert_ok!(AssetManager::register_asset(
            Origin::root(),
            MockAssetType::MockAsset(1),
            None,
            Some(0u32.into()),
            1u32.into(),
            true
        ));

        assert_eq!(
            AssetManager::asset_id_type(1).unwrap(),
            MockAssetType::MockAsset(1)
        );
        assert_noop!(
            AssetManager::register_asset(
                Origin::root(),
                MockAssetType::MockAsset(1),
                None,
                Some(0u32.into()),
                1u32.into(),
                true
            ),
            Error::<Test>::AssetAlreadyExists
        );
    });
}

#[test]
fn test_asset_create_failed_error() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            AssetManager::register_asset(
                Origin::root(),
                MockAssetType::MockAsset(1),
                None,
                Some(0u32.into()),
                0u32.into(),
                true
            ),
            Error::<Test>::ErrorCreatingAsset
        );
    });
}

#[test]
fn test_root_can_change_units_per_second() {
    new_test_ext().execute_with(|| {
		assert_ok!(AssetManager::register_asset(
			Origin::root(),
			MockAssetType::MockAsset(1),
			None,
            Some(0u32.into()),
			1u32.into(),
			true
		));

		assert_ok!(AssetManager::set_asset_units_per_second(
			Origin::root(),
			MockAssetType::MockAsset(1),
			200u128.into(),
		));

		assert_eq!(
			AssetManager::asset_type_units_per_second(MockAssetType::MockAsset(1)).unwrap(),
			200
		);
		assert!(AssetManager::supported_fee_payment_assets().contains(&MockAssetType::MockAsset(1)));

		expect_events(vec![
			crate::Event::AssetRegistered {
				asset_id: 1,
				asset: MockAssetType::MockAsset(1),
			},
			crate::Event::UnitsPerSecondUpdated {
				asset_type: MockAssetType::MockAsset(1),
				units_per_second: 200,
			},
		])
	});
}

#[test]
fn test_regular_user_cannot_call_extrinsics() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            AssetManager::register_asset(
                Origin::signed(1),
                MockAssetType::MockAsset(1),
                None,
                Some(0u32.into()),
                1u32.into(),
                true
            ),
            sp_runtime::DispatchError::BadOrigin
        );

        assert_noop!(
            AssetManager::set_asset_units_per_second(
                Origin::signed(1),
                MockAssetType::MockAsset(1),
                200u128.into(),
            ),
            sp_runtime::DispatchError::BadOrigin
        );

        assert_noop!(
            AssetManager::change_existing_asset_type(
                Origin::signed(1),
                1,
                MockAssetType::MockAsset(2),
            ),
            sp_runtime::DispatchError::BadOrigin
        );
    });
}

#[test]
fn test_root_can_change_asset_id_type() {
    new_test_ext().execute_with(|| {
        assert_ok!(AssetManager::register_asset(
            Origin::root(),
            MockAssetType::MockAsset(1),
            None,
            Some(0u32.into()),
            1u32.into(),
            true
        ));

        assert_ok!(AssetManager::set_asset_units_per_second(
            Origin::root(),
            MockAssetType::MockAsset(1),
            200u128.into(),
        ));

        assert_ok!(AssetManager::change_existing_asset_type(
            Origin::root(),
            1,
            MockAssetType::MockAsset(2),
        ));

        // New one contains the new asset type units per second
        assert_eq!(
            AssetManager::asset_type_units_per_second(MockAssetType::MockAsset(2)).unwrap(),
            200
        );

        // Old one does not contain units per second
        assert!(AssetManager::asset_type_units_per_second(MockAssetType::MockAsset(1)).is_none());

        // New associations are stablished
        assert_eq!(
            AssetManager::asset_id_type(1).unwrap(),
            MockAssetType::MockAsset(2)
        );
        assert_eq!(
            AssetManager::asset_type_id(MockAssetType::MockAsset(2)).unwrap(),
            1
        );

        // Old ones are deleted
        assert!(AssetManager::asset_type_id(MockAssetType::MockAsset(1)).is_none());

        expect_events(vec![
            crate::Event::AssetRegistered {
                asset_id: 1,
                asset: MockAssetType::MockAsset(1),
            },
            crate::Event::UnitsPerSecondUpdated {
                asset_type: MockAssetType::MockAsset(1),
                units_per_second: 200,
            },
            crate::Event::AssetTypeUpdated {
                asset_id: 1,
                new_asset_type: MockAssetType::MockAsset(2),
            },
        ])
    });
}

#[test]
fn test_change_units_per_second_after_setting_it_once() {
    new_test_ext().execute_with(|| {
		assert_ok!(AssetManager::register_asset(
			Origin::root(),
			MockAssetType::MockAsset(1),
            None,
			Some(0u32.into()),
			1u32.into(),
			true,
		));

		assert_ok!(AssetManager::set_asset_units_per_second(
			Origin::root(),
			MockAssetType::MockAsset(1),
			200u128.into(),
		));

		assert_eq!(
			AssetManager::asset_type_units_per_second(MockAssetType::MockAsset(1)).unwrap(),
			200
		);
		assert!(AssetManager::supported_fee_payment_assets().contains(&MockAssetType::MockAsset(1)));

		assert_ok!(AssetManager::set_asset_units_per_second(
			Origin::root(),
			MockAssetType::MockAsset(1),
			100u128.into(),
		));

		assert_eq!(
			AssetManager::asset_type_units_per_second(MockAssetType::MockAsset(1)).unwrap(),
			100
		);
		assert!(AssetManager::supported_fee_payment_assets().contains(&MockAssetType::MockAsset(1)));

		expect_events(vec![
			crate::Event::AssetRegistered {
				asset_id: 1,
				asset: MockAssetType::MockAsset(1),
			},
			crate::Event::UnitsPerSecondUpdated {
				asset_type: MockAssetType::MockAsset(1),
				units_per_second: 200,
			},
			crate::Event::UnitsPerSecondUpdated {
				asset_type: MockAssetType::MockAsset(1),
				units_per_second: 100,
			},
		]);
	});
}

#[test]
fn test_root_can_change_units_per_second_and_then_remove() {
    new_test_ext().execute_with(|| {
		assert_ok!(AssetManager::register_asset(
			Origin::root(),
			MockAssetType::MockAsset(1),
			None,
            Some(0u32.into()),
			1u32.into(),
			true,
		));

		assert_ok!(AssetManager::set_asset_units_per_second(
			Origin::root(),
			MockAssetType::MockAsset(1),
			200u128.into(),
		));

		assert_eq!(
			AssetManager::asset_type_units_per_second(MockAssetType::MockAsset(1)).unwrap(),
			200
		);
		assert!(AssetManager::supported_fee_payment_assets().contains(&MockAssetType::MockAsset(1)));

		assert_ok!(AssetManager::remove_supported_asset(
			Origin::root(),
			MockAssetType::MockAsset(1),
		));

		assert!(
			!AssetManager::supported_fee_payment_assets().contains(&MockAssetType::MockAsset(1))
		);

		expect_events(vec![
			crate::Event::AssetRegistered {
				asset_id: 1,
				asset: MockAssetType::MockAsset(1),
			},
			crate::Event::UnitsPerSecondUpdated {
				asset_type: MockAssetType::MockAsset(1),
				units_per_second: 200,
			},
			crate::Event::SupportedAssetRemoved {
				asset_type: MockAssetType::MockAsset(1),
			},
		]);
	});
}

#[test]
fn test_weight_hint_error() {
    new_test_ext().execute_with(|| {
        assert_ok!(AssetManager::register_asset(
            Origin::root(),
            MockAssetType::MockAsset(1),
            None,
            Some(0u32.into()),
            1u32.into(),
            true,
        ));

        assert_ok!(AssetManager::set_asset_units_per_second(
            Origin::root(),
            MockAssetType::MockAsset(1),
            200u128.into(),
        ));

        assert_ok!(AssetManager::remove_supported_asset(
            Origin::root(),
            MockAssetType::MockAsset(1)
        ));
    });
}

#[test]
fn test_asset_id_non_existent_error() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            AssetManager::set_asset_units_per_second(
                Origin::root(),
                MockAssetType::MockAsset(1),
                200u128.into(),
            ),
            Error::<Test>::AssetDoesNotExist
        );
        assert_noop!(
            AssetManager::change_existing_asset_type(
                Origin::root(),
                1,
                MockAssetType::MockAsset(2),
            ),
            Error::<Test>::AssetDoesNotExist
        );
    });
}

#[test]
fn test_populate_supported_fee_payment_assets_works() {
    new_test_ext().execute_with(|| {
		use frame_support::StorageHasher;
		let pallet_prefix: &[u8] = b"AssetManager";
		let storage_item_prefix: &[u8] = b"AssetTypeUnitsPerSecond";
		use frame_support::traits::OnRuntimeUpgrade;
		use parity_scale_codec::Encode;

		put_storage_value(
			pallet_prefix,
			storage_item_prefix,
			&Blake2_128Concat::hash(&MockAssetType::MockAsset(1).encode()),
			10u128,
		);

		assert_noop!(
			AssetManager::set_asset_units_per_second(
				Origin::root(),
				MockAssetType::MockAsset(1),
				200u128.into(),
			),
			Error::<Test>::AssetDoesNotExist
		);

		assert!(AssetManager::supported_fee_payment_assets().len() == 0);

		// We run the migration
		crate::migrations::PopulateSupportedFeePaymentAssets::<Test>::on_runtime_upgrade();

		assert!(AssetManager::supported_fee_payment_assets().len() == 1);
		assert!(AssetManager::supported_fee_payment_assets().contains(&MockAssetType::MockAsset(1)));
	});
}

#[test]
fn test_asset_manager_units_with_asset_type_migration_works() {
    new_test_ext().execute_with(|| {
        let pallet_prefix: &[u8] = b"AssetManager";
        let storage_item_prefix: &[u8] = b"AssetIdUnitsPerSecond";
        use frame_support::traits::OnRuntimeUpgrade;
        use frame_support::StorageHasher;
        use parity_scale_codec::Encode;

        assert_ok!(AssetManager::register_asset(
            Origin::root(),
            MockAssetType::MockAsset(1),
            None,
            Some(0u32.into()),
            1u32.into(),
            true
        ));

        // We populate the previous storage with assetId as key
        put_storage_value(
            pallet_prefix,
            storage_item_prefix,
            &Blake2_128Concat::hash(&1u32.encode()),
            200u128,
        );

        // We run the migration
        crate::migrations::UnitsWithAssetType::<Test>::on_runtime_upgrade();

        // After migration, units per second should be indexed by AssetType
        assert_eq!(
            AssetManager::asset_type_units_per_second(MockAssetType::MockAsset(1)).unwrap(),
            200
        );

        let pallet_prefix: &[u8] = b"AssetManager";
        let storage_item_prefix: &[u8] = b"AssetIdType";

        // Assert that old storage is empty
        assert!(storage_key_iter::<mock::AssetId, u128, Blake2_128Concat>(
            pallet_prefix,
            storage_item_prefix
        )
        .next()
        .is_none());
    });
}

#[test]
fn test_asset_manager_populate_asset_type_id_storage_migration_works() {
    new_test_ext().execute_with(|| {
        let pallet_prefix: &[u8] = b"AssetManager";
        let storage_item_prefix: &[u8] = b"AssetIdType";
        use frame_support::traits::OnRuntimeUpgrade;
        use frame_support::StorageHasher;
        use parity_scale_codec::Encode;

        // We populate AssetIdType manually
        put_storage_value(
            pallet_prefix,
            storage_item_prefix,
            &Blake2_128Concat::hash(&1u32.encode()),
            MockAssetType::MockAsset(1),
        );

        // We run the migration
        crate::migrations::PopulateAssetTypeIdStorage::<Test>::on_runtime_upgrade();

        // After migration, the new storage item should be populated
        assert_eq!(
            AssetManager::asset_type_id(MockAssetType::MockAsset(1)).unwrap(),
            1
        );
    });
}

#[test]
fn test_asset_manager_change_statemine_prefixes() {
    new_test_ext().execute_with(|| {
        let pallet_prefix: &[u8] = b"AssetManager";
        let storage_item_prefix: &[u8] = b"AssetIdType";
        use frame_support::traits::OnRuntimeUpgrade;
        use frame_support::StorageHasher;
        use parity_scale_codec::Encode;

        let statemine_para_id = mock::StatemineParaIdInfo::get();
        let statemine_assets_pallet = mock::StatemineAssetsInstanceInfo::get();

        let statemine_multilocation = MockAssetType::Xcm(MultiLocation {
            parents: 1,
            interior: X2(Parachain(statemine_para_id), GeneralIndex(1)),
        });

        let statemine_multilocation_2 = MockAssetType::Xcm(MultiLocation {
            parents: 1,
            interior: X2(Parachain(statemine_para_id), GeneralIndex(2)),
        });

        let statemine_multilocation_3 = MockAssetType::Xcm(MultiLocation {
            parents: 1,
            interior: X2(Parachain(statemine_para_id), GeneralIndex(3)),
        });

        let asset_id: mock::AssetId = statemine_multilocation.clone().into();

        // We are gonna test three cases:
        // Case 1: AssetManagerPopulateAssetTypeIdStorage has not executed yet
        // (only AssetIdType is populated)
        // Case 2: AssetManagerPopulateAssetTypeIdStorage has already executed
        // Case 3: AssetManagerUnitsWithAssetType has already executed

        // To mimic case 1, we populate AssetIdType manually but not AssetTypeId
        put_storage_value(
            pallet_prefix,
            storage_item_prefix,
            &Blake2_128Concat::hash(&asset_id.encode()),
            statemine_multilocation.clone(),
        );

        // Assert the storage item is well populated
        assert_eq!(
            AssetManager::asset_id_type(asset_id).unwrap(),
            statemine_multilocation
        );

        // To mimic case 2, we can simply register the asset through the extrinsic
        assert_ok!(AssetManager::register_asset(
            Origin::root(),
            statemine_multilocation_2.clone(),
            None,
            Some(0u32.into()),
            1u32.into(),
            true
        ));

        // To mimic case 3, we can simply register the asset through the extrinsic
        // But we also need to set units per second
        assert_ok!(AssetManager::register_asset(
            Origin::root(),
            statemine_multilocation_3.clone(),
            None,
            Some(0u32.into()),
            1u32.into(),
            true
        ));

        assert_ok!(AssetManager::set_asset_units_per_second(
            Origin::root(),
            statemine_multilocation_3.clone(),
            1u128,
        ));

        // We run the migration
        crate::migrations::ChangeStateminePrefixes::<
            Test,
            mock::StatemineParaIdInfo,
            mock::StatemineAssetsInstanceInfo,
        >::on_runtime_upgrade();

        // Check case 1
        let expected_statemine_multilocation = MockAssetType::Xcm(MultiLocation {
            parents: 1,
            interior: X3(
                Parachain(statemine_para_id),
                PalletInstance(statemine_assets_pallet),
                GeneralIndex(1),
            ),
        });

        // After migration, the storage item should have been upgraded
        assert_eq!(
            AssetManager::asset_id_type(asset_id).unwrap(),
            expected_statemine_multilocation
        );

        // Check case 2
        let expected_statemine_multilocation_2 = MockAssetType::Xcm(MultiLocation {
            parents: 1,
            interior: X3(
                Parachain(statemine_para_id),
                PalletInstance(statemine_assets_pallet),
                GeneralIndex(2),
            ),
        });

        let asset_id_2: mock::AssetId = statemine_multilocation_2.clone().into();

        // After migration, both storage items should have been upgraded
        assert_eq!(
            AssetManager::asset_id_type(asset_id_2).unwrap(),
            expected_statemine_multilocation_2
        );

        assert_eq!(
            AssetManager::asset_type_id(expected_statemine_multilocation_2).unwrap(),
            asset_id_2
        );

        // And the previous one should be cleaned
        assert!(AssetManager::asset_type_id(&statemine_multilocation_2).is_none());

        // Check case 3
        let expected_statemine_multilocation_3 = MockAssetType::Xcm(MultiLocation {
            parents: 1,
            interior: X3(
                Parachain(statemine_para_id),
                PalletInstance(statemine_assets_pallet),
                GeneralIndex(3),
            ),
        });

        let asset_id_3: mock::AssetId = statemine_multilocation_3.clone().into();

        // After migration, both storage items should have been upgraded
        assert_eq!(
            AssetManager::asset_id_type(asset_id_3).unwrap(),
            expected_statemine_multilocation_3
        );

        assert_eq!(
            AssetManager::asset_type_id(&expected_statemine_multilocation_3).unwrap(),
            asset_id_3
        );

        // The previous one should be cleaned
        assert!(AssetManager::asset_type_id(&statemine_multilocation_3).is_none());

        // Units per second updated
        assert_eq!(
            AssetManager::asset_type_units_per_second(&expected_statemine_multilocation_3).unwrap(),
            1
        );
        assert!(AssetManager::asset_type_units_per_second(&statemine_multilocation_3).is_none());
    });
}

#[test]
fn test_root_can_remove_asset_association() {
    new_test_ext().execute_with(|| {
        assert_ok!(AssetManager::register_asset(
            Origin::root(),
            MockAssetType::MockAsset(1),
            None,
            Some(0u32.into()),
            1u32.into(),
            true
        ));

        assert_ok!(AssetManager::set_asset_units_per_second(
            Origin::root(),
            MockAssetType::MockAsset(1),
            200u128.into(),
        ));

        assert_ok!(AssetManager::remove_existing_asset_type(Origin::root(), 1,));

        // Mappings are deleted
        assert!(AssetManager::asset_type_id(MockAssetType::MockAsset(1)).is_none());
        assert!(AssetManager::asset_id_type(1).is_none());

        // Units per second removed
        assert!(AssetManager::asset_type_units_per_second(MockAssetType::MockAsset(1)).is_none());

        expect_events(vec![
            crate::Event::AssetRegistered {
                asset_id: 1,
                asset: MockAssetType::MockAsset(1),
            },
            crate::Event::UnitsPerSecondUpdated {
                asset_type: MockAssetType::MockAsset(1),
                units_per_second: 200,
            },
            crate::Event::AssetRemoved {
                asset_id: 1,
                asset_type: MockAssetType::MockAsset(1),
            },
        ])
    });
}

#[test]
fn test_removing_without_asset_units_per_second_does_not_panic() {
    new_test_ext().execute_with(|| {
        assert_ok!(AssetManager::register_asset(
            Origin::root(),
            MockAssetType::MockAsset(1),
            None,
            Some(0u32.into()),
            1u32.into(),
            true
        ));

        assert_ok!(AssetManager::remove_existing_asset_type(Origin::root(), 1,));

        // Mappings are deleted
        assert!(AssetManager::asset_type_id(MockAssetType::MockAsset(1)).is_none());
        assert!(AssetManager::asset_id_type(1).is_none());

        // Units per second removed
        assert!(AssetManager::asset_type_units_per_second(MockAssetType::MockAsset(1)).is_none());

        expect_events(vec![
            crate::Event::AssetRegistered {
                asset_id: 1,
                asset: MockAssetType::MockAsset(1),
            },
            crate::Event::AssetRemoved {
                asset_id: 1,
                asset_type: MockAssetType::MockAsset(1),
            },
        ])
    });
}
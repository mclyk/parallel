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

use kerria_runtime::{
    opaque::SessionKeys, BalancesConfig, BaseFeeConfig, CollatorSelectionConfig, DemocracyConfig,
    EVMConfig, GeneralCouncilConfig, GeneralCouncilMembershipConfig, GenesisConfig,
    ParachainInfoConfig, ParallelPrecompilesType, PolkadotXcmConfig, SessionConfig, SudoConfig,
    SystemConfig, TechnicalCommitteeMembershipConfig, WASM_BINARY,
};
use primitives::{network::NetworkType, *};
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
// use sp_core::sr25519;
use sp_runtime::traits::Zero;

use crate::chain_spec::{as_properties, get_authority_keys_from_seed, Extensions, TELEMETRY_URL};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

pub fn kerria_dev_config(id: ParaId) -> ChainSpec {
    let initial_account: AccountId = "5DyUFZag8FHYT2bSSMX9DK2hCzeinL5xbdaQnyibsGAMqsDC"
        .parse()
        .unwrap();
    ChainSpec::from_genesis(
        // Name
        "Kerria Dev",
        // ID
        "kerria-dev",
        ChainType::Development,
        move || {
            let root_key = initial_account.clone();
            let invulnerables = vec![get_authority_keys_from_seed("Alice")];
            // let oracle_accounts = vec![get_account_id_from_seed::<sr25519::Public>("Ferdie")];
            // let bridge_accounts = vec![get_account_id_from_seed::<sr25519::Public>("Alice")];
            // let liquid_staking_agents = vec![get_account_id_from_seed::<sr25519::Public>("Eve")];
            // let crowdloans_automators = vec![get_account_id_from_seed::<sr25519::Public>("Bob")];
            // let initial_allocation: Vec<(AccountId, Balance)> = accumulate(
            //     [substrate_accounts(), evm_accounts()]
            //         .concat()
            //         .iter()
            //         .flat_map(|x| {
            //             if x == &"5HHMY7e8UAqR5ZaHGaQnRW5EDR8dP7QpAyjeBu6V7vdXxxbf"
            //                 .parse()
            //                 .unwrap()
            //             {
            //                 vec![(x.clone(), 10_u128.pow(20))]
            //             } else {
            //                 vec![(x.clone(), 10_u128.pow(16))]
            //             }
            //         }),
            // );
            let initial_allocation: Vec<(AccountId, Balance)> =
                vec![(initial_account.clone(), 10_u128.pow(20))];
            // let vesting_list = vec![];
            let council = vec![initial_account.clone()];
            let technical_committee = vec![initial_account.clone()];

            kerria_genesis(
                root_key,
                invulnerables,
                initial_allocation,
                // vesting_list,
                // oracle_accounts,
                // bridge_accounts,
                // liquid_staking_agents,
                // crowdloans_automators,
                council,
                technical_committee,
                id,
            )
        },
        vec![],
        TelemetryEndpoints::new(vec![(TELEMETRY_URL.into(), 0)]).ok(),
        Some("kerria-dev"),
        None,
        Some(as_properties(NetworkType::Parallel)),
        Extensions {
            relay_chain: "polkadot-local".into(),
            para_id: id.into(),
        },
    )
}

pub fn kerria_config(_id: ParaId) -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("./kerria-3350.json")[..])
}

fn kerria_genesis(
    root_key: AccountId,
    invulnerables: Vec<(AccountId, AuraId)>,
    initial_allocation: Vec<(AccountId, Balance)>,
    // vesting_list: Vec<(AccountId, BlockNumber, BlockNumber, u32, Balance)>,
    // oracle_accounts: Vec<AccountId>,
    // bridge_accounts: Vec<AccountId>,
    // liquid_staking_agents: Vec<AccountId>,
    // crowdloans_automators: Vec<AccountId>,
    council: Vec<AccountId>,
    technical_committee: Vec<AccountId>,
    id: ParaId,
) -> GenesisConfig {
    let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];
    GenesisConfig {
        system: SystemConfig {
            code: WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
        },
        balances: BalancesConfig {
            balances: initial_allocation,
        },
        collator_selection: CollatorSelectionConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
            candidacy_bond: Zero::zero(),
            desired_candidates: 16,
        },
        session: SessionConfig {
            keys: invulnerables
                .iter()
                .cloned()
                .map(|(acc, aura)| {
                    (
                        acc.clone(),          // account id
                        acc,                  // validator id
                        SessionKeys { aura }, // session keys
                    )
                })
                .collect(),
        },
        aura: Default::default(),
        aura_ext: Default::default(),
        parachain_system: Default::default(),
        sudo: SudoConfig {
            key: Some(root_key),
        },
        parachain_info: ParachainInfoConfig { parachain_id: id },
        // liquid_staking: LiquidStakingConfig {
        //     exchange_rate: Rate::saturating_from_rational(100u32, 100u32), // 1
        //     reserve_factor: Ratio::from_rational(5u32, 10_000u32),         // 0.05%
        // },
        democracy: DemocracyConfig::default(),
        general_council: GeneralCouncilConfig::default(),
        general_council_membership: GeneralCouncilMembershipConfig {
            members: council.try_into().unwrap(),
            phantom: Default::default(),
        },
        technical_committee: Default::default(),
        technical_committee_membership: TechnicalCommitteeMembershipConfig {
            members: technical_committee.try_into().unwrap(),
            phantom: Default::default(),
        },
        treasury: Default::default(),
        // oracle_membership: OracleMembershipConfig {
        //     members: oracle_accounts.try_into().unwrap(),
        //     phantom: Default::default(),
        // },
        // bridge_membership: BridgeMembershipConfig {
        //     members: bridge_accounts.try_into().unwrap(),
        //     phantom: Default::default(),
        // },
        // liquid_staking_agents_membership: LiquidStakingAgentsMembershipConfig {
        //     members: liquid_staking_agents.try_into().unwrap(),
        //     phantom: Default::default(),
        // },
        // crowdloans_automators_membership: CrowdloansAutomatorsMembershipConfig {
        //     members: crowdloans_automators.try_into().unwrap(),
        //     phantom: Default::default(),
        // },
        // vesting: VestingConfig {
        //     vesting: vesting_list.try_into().unwrap(),
        // },
        polkadot_xcm: PolkadotXcmConfig {
            safe_xcm_version: Some(2),
        },
        evm: EVMConfig {
            // We need _some_ code inserted at the precompile address so that
            // the evm will actually call the address.
            accounts: ParallelPrecompilesType::used_addresses()
                .map(|addr| {
                    (
                        addr,
                        fp_evm::GenesisAccount {
                            nonce: Default::default(),
                            balance: Default::default(),
                            storage: Default::default(),
                            code: revert_bytecode.clone(),
                        },
                    )
                })
                .collect(),
        },
        base_fee: BaseFeeConfig::new(sp_core::U256::from(10_000_000), sp_runtime::Permill::zero()),
        ethereum: Default::default(),
    }
}

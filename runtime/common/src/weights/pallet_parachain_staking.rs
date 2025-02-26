// Copyright 2021-2022 Zeitgeist PM LLC.
//
// This file is part of Zeitgeist.
//
// Zeitgeist is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation, either version 3 of the License, or (at
// your option) any later version.
//
// Zeitgeist is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Zeitgeist. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_parachain_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/zeitgeist
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_parachain_staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./misc/frame_weight_template.hbs
// --output=./runtime/common/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use core::marker::PhantomData;
use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions for pallet_parachain_staking (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_parachain_staking::weights::WeightInfo for WeightInfo<T> {
    // Storage: ParachainStaking InflationConfig (r:1 w:1)
    fn set_staking_expectations() -> Weight {
        Weight::from_ref_time(36_750_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: ParachainStaking InflationConfig (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    fn set_inflation() -> Weight {
        Weight::from_ref_time(94_430_000)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
    fn set_parachain_bond_account() -> Weight {
        Weight::from_ref_time(33_460_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
    fn set_parachain_bond_reserve_percent() -> Weight {
        Weight::from_ref_time(32_350_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: ParachainStaking TotalSelected (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    fn set_total_selected() -> Weight {
        Weight::from_ref_time(36_990_000)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: ParachainStaking CollatorCommission (r:1 w:1)
    fn set_collator_commission() -> Weight {
        Weight::from_ref_time(31_200_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: ParachainStaking Round (r:1 w:1)
    // Storage: ParachainStaking TotalSelected (r:1 w:0)
    // Storage: ParachainStaking InflationConfig (r:1 w:1)
    fn set_blocks_per_round() -> Weight {
        Weight::from_ref_time(99_720_000)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking DelegatorState (r:1 w:0)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:0 w:1)
    // Storage: ParachainStaking BottomDelegations (r:0 w:1)
    fn join_candidates(x: u32) -> Weight {
        Weight::from_ref_time(105_148_000)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(369_000).saturating_mul(x.into()))
            .saturating_add(T::DbWeight::get().reads(6))
            .saturating_add(T::DbWeight::get().writes(7))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    fn schedule_leave_candidates(x: u32) -> Weight {
        Weight::from_ref_time(118_551_000)
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(313_000).saturating_mul(x.into()))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: Balances Locks (r:2 w:2)
    // Storage: System Account (r:2 w:2)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    // Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
    // Storage: ParachainStaking BottomDelegations (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    fn execute_leave_candidates(x: u32) -> Weight {
        Weight::from_ref_time(0)
            // Standard Error: 226_000
            .saturating_add(Weight::from_ref_time(44_890_000).saturating_mul(x.into()))
            .saturating_add(T::DbWeight::get().reads(6))
            .saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(x.into())))
            .saturating_add(T::DbWeight::get().writes(5))
            .saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(x.into())))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    fn cancel_leave_candidates(x: u32) -> Weight {
        Weight::from_ref_time(115_324_000)
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(358_000).saturating_mul(x.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    fn go_offline() -> Weight {
        Weight::from_ref_time(52_570_000)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    fn go_online() -> Weight {
        Weight::from_ref_time(51_920_000)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    fn candidate_bond_more() -> Weight {
        Weight::from_ref_time(97_240_000)
            .saturating_add(T::DbWeight::get().reads(5))
            .saturating_add(T::DbWeight::get().writes(5))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    fn schedule_candidate_bond_less() -> Weight {
        Weight::from_ref_time(47_370_000)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    // Storage: ParachainStaking Total (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    fn execute_candidate_bond_less() -> Weight {
        Weight::from_ref_time(94_561_000)
            .saturating_add(T::DbWeight::get().reads(6))
            .saturating_add(T::DbWeight::get().writes(5))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    fn cancel_candidate_bond_less() -> Weight {
        Weight::from_ref_time(50_120_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    fn delegate(x: u32, y: u32) -> Weight {
        Weight::from_ref_time(221_620_000)
            // Standard Error: 48_000
            .saturating_add(Weight::from_ref_time(372_000).saturating_mul(x.into()))
            // Standard Error: 15_000
            .saturating_add(Weight::from_ref_time(527_000).saturating_mul(y.into()))
            .saturating_add(T::DbWeight::get().reads(7))
            .saturating_add(T::DbWeight::get().writes(7))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    fn schedule_leave_delegators() -> Weight {
        Weight::from_ref_time(63_370_000)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    // Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:0)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn execute_leave_delegators(x: u32) -> Weight {
        Weight::from_ref_time(27_207_000)
            // Standard Error: 226_000
            .saturating_add(Weight::from_ref_time(34_240_000).saturating_mul(x.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(x.into())))
            .saturating_add(T::DbWeight::get().writes(2))
            .saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(x.into())))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    fn cancel_leave_delegators() -> Weight {
        Weight::from_ref_time(52_221_000)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    fn schedule_revoke_delegation() -> Weight {
        Weight::from_ref_time(51_631_000)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    fn delegator_bond_more() -> Weight {
        Weight::from_ref_time(109_970_000)
            .saturating_add(T::DbWeight::get().reads(8))
            .saturating_add(T::DbWeight::get().writes(7))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    fn schedule_delegator_bond_less() -> Weight {
        Weight::from_ref_time(51_671_000)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:0)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    fn execute_revoke_delegation() -> Weight {
        Weight::from_ref_time(138_000_000)
            .saturating_add(T::DbWeight::get().reads(10))
            .saturating_add(T::DbWeight::get().writes(8))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    fn execute_delegator_bond_less() -> Weight {
        Weight::from_ref_time(121_801_000)
            .saturating_add(T::DbWeight::get().reads(9))
            .saturating_add(T::DbWeight::get().writes(8))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    fn cancel_revoke_delegation() -> Weight {
        Weight::from_ref_time(49_720_000)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    fn cancel_delegator_bond_less() -> Weight {
        Weight::from_ref_time(61_280_000)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    // Storage: ParachainStaking Round (r:1 w:1)
    // Storage: ParachainStaking Points (r:1 w:0)
    // Storage: ParachainStaking Staked (r:1 w:2)
    // Storage: ParachainStaking InflationConfig (r:1 w:0)
    // Storage: ParachainStaking ParachainBondInfo (r:1 w:0)
    // Storage: System Account (r:302 w:301)
    // Storage: ParachainStaking CollatorCommission (r:1 w:0)
    // Storage: ParachainStaking CandidatePool (r:1 w:0)
    // Storage: ParachainStaking TotalSelected (r:1 w:0)
    // Storage: ParachainStaking CandidateInfo (r:9 w:0)
    // Storage: ParachainStaking DelegationScheduledRequests (r:9 w:0)
    // Storage: ParachainStaking TopDelegations (r:9 w:0)
    // Storage: ParachainStaking AutoCompoundingDelegations (r:9 w:0)
    // Storage: ParachainStaking Total (r:1 w:0)
    // Storage: ParachainStaking AwardedPts (r:2 w:1)
    // Storage: ParachainStaking AtStake (r:1 w:10)
    // Storage: ParachainStaking SelectedCandidates (r:0 w:1)
    // Storage: ParachainStaking DelayedPayouts (r:0 w:1)
    fn round_transition_on_initialize(x: u32, y: u32) -> Weight {
        Weight::from_ref_time(0)
            // Standard Error: 1_640_000
            .saturating_add(Weight::from_ref_time(58_147_000).saturating_mul(x.into()))
            // Standard Error: 5_000
            .saturating_add(Weight::from_ref_time(398_000).saturating_mul(y.into()))
            .saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(x.into())))
    }
    // Storage: ParachainStaking DelayedPayouts (r:1 w:0)
    // Storage: ParachainStaking Points (r:1 w:0)
    // Storage: ParachainStaking AwardedPts (r:2 w:1)
    // Storage: ParachainStaking AtStake (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn pay_one_collator_reward(y: u32) -> Weight {
        Weight::from_ref_time(74_512_000)
            // Standard Error: 78_000
            .saturating_add(Weight::from_ref_time(18_939_000).saturating_mul(y.into()))
            .saturating_add(T::DbWeight::get().reads(6))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(y.into())))
            .saturating_add(T::DbWeight::get().writes(3))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(y.into())))
    }
    // Storage: ParachainStaking Round (r:1 w:0)
    fn base_on_initialize() -> Weight {
        Weight::from_ref_time(15_540_000).saturating_add(T::DbWeight::get().reads(1))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:0)
    // Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
    fn set_auto_compound(x: u32, y: u32) -> Weight {
        Weight::from_ref_time(172_244_000)
            // Standard Error: 11_000
            .saturating_add(Weight::from_ref_time(387_000).saturating_mul(x.into()))
            // Standard Error: 35_000
            .saturating_add(Weight::from_ref_time(30_000).saturating_mul(y.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    // Storage: ParachainStaking BottomDelegations (r:1 w:1)
    fn delegate_with_auto_compound(x: u32, y: u32, _z: u32) -> Weight {
        Weight::from_ref_time(290_047_000)
            // Standard Error: 13_000
            .saturating_add(Weight::from_ref_time(115_000).saturating_mul(x.into()))
            // Standard Error: 13_000
            .saturating_add(Weight::from_ref_time(184_000).saturating_mul(y.into()))
            .saturating_add(T::DbWeight::get().reads(8))
            .saturating_add(T::DbWeight::get().writes(8))
    }
}

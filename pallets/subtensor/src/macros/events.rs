use frame_support::pallet_macros::pallet_section;

/// A [`pallet_section`] that defines the events for a pallet.
/// This can later be imported into the pallet using [`import_section`].
#[pallet_section]
mod events {
    use codec::Compact;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// a new network is added.
        NetworkAdded(u16, u16),
        /// a network is removed.
        NetworkRemoved(u16),
        /// stake has been transferred from the a coldkey account onto the hotkey staking account.
        StakeAdded(T::AccountId, T::AccountId, u64, u64, u16, u64),
        /// stake has been removed from the hotkey staking account onto the coldkey account.
        StakeRemoved(T::AccountId, T::AccountId, u64, u64, u16, u64),
        /// stake has been transferred from the coldkey account onto the hotkey staking account (at the end of the block)
        AggregatedStakeAdded(T::AccountId, T::AccountId, u16, u64),
        /// adding aggregated stake has failed
        FailedToAddAggregatedStake(T::AccountId, T::AccountId, u16, u64),
        /// limited stake has been transferred from the coldkey account onto the hotkey staking account (at the end of the block)
        AggregatedLimitedStakeAdded(T::AccountId, T::AccountId, u16, u64, u64, bool),
        /// adding limited aggregated stake has failed
        FailedToAddAggregatedLimitedStake(T::AccountId, T::AccountId, u16, u64, u64, bool),
        /// stake has been removed from the hotkey staking account into the coldkey account (at the end of the block).
        AggregatedStakeRemoved(T::AccountId, T::AccountId, u16, u64),
        /// removing aggregated stake has failed
        FailedToRemoveAggregatedStake(T::AccountId, T::AccountId, u16, u64),
        /// aggregated limited stake has been removed from the hotkey staking account into the coldkey account (at the end of the block).
        AggregatedLimitedStakeRemoved(T::AccountId, T::AccountId, u16, u64, u64, bool),
        /// removing limited aggregated stake has failed
        FailedToRemoveAggregatedLimitedStake(T::AccountId, T::AccountId, u16, u64, u64, bool),
        ///  aggregated unstake_all operation has succeeded
        AggregatedUnstakeAllSucceeded(T::AccountId, T::AccountId),
        /// aggregated unstake_all operation has failed
        AggregatedUnstakeAllFailed(T::AccountId, T::AccountId),
        ///  aggregated unstake_all_alpha operation has succeeded
        AggregatedUnstakeAllAlphaSucceeded(T::AccountId, T::AccountId),
        /// aggregated unstake_all_alpha operation has failed
        AggregatedUnstakeAllAlphaFailed(T::AccountId, T::AccountId),
        /// stake has been moved from origin (hotkey, subnet ID) to destination (hotkey, subnet ID) of this amount (in TAO).
        StakeMoved(T::AccountId, T::AccountId, u16, T::AccountId, u16, u64),
        /// a caller successfully sets their weights on a subnetwork.
        WeightsSet(u16, u16),
        /// a new neuron account has been registered to the chain.
        NeuronRegistered(u16, u16, T::AccountId),
        /// multiple uids have been concurrently registered.
        BulkNeuronsRegistered(u16, u16),
        /// FIXME: Not used yet
        BulkBalancesSet(u16, u16),
        /// max allowed uids has been set for a subnetwork.
        MaxAllowedUidsSet(u16, u16),
        /// the max weight limit has been set for a subnetwork.
        MaxWeightLimitSet(u16, u16),
        /// the difficulty has been set for a subnet.
        DifficultySet(u16, u64),
        /// the adjustment interval is set for a subnet.
        AdjustmentIntervalSet(u16, u16),
        /// registration per interval is set for a subnet.
        RegistrationPerIntervalSet(u16, u16),
        /// we set max registrations per block.
        MaxRegistrationsPerBlockSet(u16, u16),
        /// an activity cutoff is set for a subnet.
        ActivityCutoffSet(u16, u16),
        /// Rho value is set.
        RhoSet(u16, u16),
        /// steepness of the sigmoid used to compute alpha values.
        AlphaSigmoidSteepnessSet(u16, u16),
        /// Kappa is set for a subnet.
        KappaSet(u16, u16),
        /// minimum allowed weight is set for a subnet.
        MinAllowedWeightSet(u16, u16),
        /// the validator pruning length has been set.
        ValidatorPruneLenSet(u16, u64),
        /// the scaling law power has been set for a subnet.
        ScalingLawPowerSet(u16, u16),
        /// weights set rate limit has been set for a subnet.
        WeightsSetRateLimitSet(u16, u64),
        /// immunity period is set for a subnet.
        ImmunityPeriodSet(u16, u16),
        /// bonds moving average is set for a subnet.
        BondsMovingAverageSet(u16, u64),
        /// bonds penalty is set for a subnet.
        BondsPenaltySet(u16, u16),
        /// bonds reset is set for a subnet.
        BondsResetOnSet(u16, bool),
        /// setting the max number of allowed validators on a subnet.
        MaxAllowedValidatorsSet(u16, u16),
        /// the axon server information is added to the network.
        AxonServed(u16, T::AccountId),
        /// the prometheus server information is added to the network.
        PrometheusServed(u16, T::AccountId),
        /// a hotkey has become a delegate.
        DelegateAdded(T::AccountId, T::AccountId, u16),
        /// the default take is set.
        DefaultTakeSet(u16),
        /// weights version key is set for a network.
        WeightsVersionKeySet(u16, u64),
        /// setting min difficulty on a network.
        MinDifficultySet(u16, u64),
        /// setting max difficulty on a network.
        MaxDifficultySet(u16, u64),
        /// setting the prometheus serving rate limit.
        ServingRateLimitSet(u16, u64),
        /// setting burn on a network.
        BurnSet(u16, u64),
        /// setting max burn on a network.
        MaxBurnSet(u16, u64),
        /// setting min burn on a network.
        MinBurnSet(u16, u64),
        /// setting the transaction rate limit.
        TxRateLimitSet(u64),
        /// setting the delegate take transaction rate limit.
        TxDelegateTakeRateLimitSet(u64),
        /// setting the childkey take transaction rate limit.
        TxChildKeyTakeRateLimitSet(u64),
        /// minimum childkey take set
        MinChildKeyTakeSet(u16),
        /// maximum childkey take set
        MaxChildKeyTakeSet(u16),
        /// childkey take set
        ChildKeyTakeSet(T::AccountId, u16),
        /// a sudo call is done.
        Sudid(DispatchResult),
        /// registration is allowed/disallowed for a subnet.
        RegistrationAllowed(u16, bool),
        /// POW registration is allowed/disallowed for a subnet.
        PowRegistrationAllowed(u16, bool),
        /// setting tempo on a network
        TempoSet(u16, u16),
        /// setting the RAO recycled for registration.
        RAORecycledForRegistrationSet(u16, u64),
        /// min stake is set for validators to set weights.
        StakeThresholdSet(u64),
        /// setting the minimum required stake amount for senate registration.
        SenateRequiredStakePercentSet(u64),
        /// setting the adjustment alpha on a subnet.
        AdjustmentAlphaSet(u16, u64),
        /// the faucet it called on the test net.
        Faucet(T::AccountId, u64),
        /// the subnet owner cut is set.
        SubnetOwnerCutSet(u16),
        /// the network creation rate limit is set.
        NetworkRateLimitSet(u64),
        /// the network immunity period is set.
        NetworkImmunityPeriodSet(u64),
        /// the network minimum locking cost is set.
        NetworkMinLockCostSet(u64),
        /// the maximum number of subnets is set
        // SubnetLimitSet(u16),
        /// the lock cost reduction is set
        NetworkLockCostReductionIntervalSet(u64),
        /// the take for a delegate is decreased.
        TakeDecreased(T::AccountId, T::AccountId, u16),
        /// the take for a delegate is increased.
        TakeIncreased(T::AccountId, T::AccountId, u16),
        /// the hotkey is swapped
        HotkeySwapped {
            /// the account ID of coldkey
            coldkey: T::AccountId,
            /// the account ID of old hotkey
            old_hotkey: T::AccountId,
            /// the account ID of new hotkey
            new_hotkey: T::AccountId,
        },
        /// maximum delegate take is set by sudo/admin transaction
        MaxDelegateTakeSet(u16),
        /// minimum delegate take is set by sudo/admin transaction
        MinDelegateTakeSet(u16),
        /// a member of the senate is adjusted
        SenateAdjusted {
            /// the account ID of the old senate member, if any
            old_member: Option<T::AccountId>,
            /// the account ID of the new senate member
            new_member: T::AccountId,
        },
        /// A coldkey has been swapped
        ColdkeySwapped {
            /// the account ID of old coldkey
            old_coldkey: T::AccountId,
            /// the account ID of new coldkey
            new_coldkey: T::AccountId,
            /// the swap cost
            swap_cost: u64,
        },
        /// All balance of a hotkey has been unstaked and transferred to a new coldkey
        AllBalanceUnstakedAndTransferredToNewColdkey {
            /// The account ID of the current coldkey
            current_coldkey: T::AccountId,
            /// The account ID of the new coldkey
            new_coldkey: T::AccountId,
            /// The total balance of the hotkey
            total_balance: <<T as Config>::Currency as fungible::Inspect<
                <T as frame_system::Config>::AccountId,
            >>::Balance,
        },
        /// A coldkey swap has been scheduled
        ColdkeySwapScheduled {
            /// The account ID of the old coldkey
            old_coldkey: T::AccountId,
            /// The account ID of the new coldkey
            new_coldkey: T::AccountId,
            /// The arbitration block for the coldkey swap
            execution_block: BlockNumberFor<T>,
            /// The swap cost
            swap_cost: u64,
        },
        /// The arbitration period has been extended
        ArbitrationPeriodExtended {
            /// The account ID of the coldkey
            coldkey: T::AccountId,
        },
        /// Setting of children of a hotkey have been scheduled
        SetChildrenScheduled(T::AccountId, u16, u64, Vec<(u64, T::AccountId)>),
        /// The children of a hotkey have been set
        SetChildren(T::AccountId, u16, Vec<(u64, T::AccountId)>),
        // /// The hotkey emission tempo has been set
        // HotkeyEmissionTempoSet(u64),
        // /// The network maximum stake has been set
        // NetworkMaxStakeSet(u16, u64),
        /// The identity of a coldkey has been set
        ChainIdentitySet(T::AccountId),
        /// The identity of a subnet has been set
        SubnetIdentitySet(u16),
        /// The identity of a subnet has been removed
        SubnetIdentityRemoved(u16),
        /// A dissolve network extrinsic scheduled.
        DissolveNetworkScheduled {
            /// The account ID schedule the dissolve network extrisnic
            account: T::AccountId,
            /// network ID will be dissolved
            netuid: u16,
            /// extrinsic execution block number
            execution_block: BlockNumberFor<T>,
        },
        /// The duration of schedule coldkey swap has been set
        ColdkeySwapScheduleDurationSet(BlockNumberFor<T>),
        /// The duration of dissolve network has been set
        DissolveNetworkScheduleDurationSet(BlockNumberFor<T>),
        /// Commit-reveal v3 weights have been successfully committed.
        ///
        /// - **who**: The account ID of the user committing the weights.
        /// - **netuid**: The network identifier.
        /// - **commit_hash**: The hash representing the committed weights.
        CRV3WeightsCommitted(T::AccountId, u16, H256),
        /// Weights have been successfully committed.
        ///
        /// - **who**: The account ID of the user committing the weights.
        /// - **netuid**: The network identifier.
        /// - **commit_hash**: The hash representing the committed weights.
        WeightsCommitted(T::AccountId, u16, H256),

        /// Weights have been successfully revealed.
        ///
        /// - **who**: The account ID of the user revealing the weights.
        /// - **netuid**: The network identifier.
        /// - **commit_hash**: The hash of the revealed weights.
        WeightsRevealed(T::AccountId, u16, H256),

        /// Weights have been successfully batch revealed.
        ///
        /// - **who**: The account ID of the user revealing the weights.
        /// - **netuid**: The network identifier.
        /// - **revealed_hashes**: A vector of hashes representing each revealed weight set.
        WeightsBatchRevealed(T::AccountId, u16, Vec<H256>),

        /// A batch of weights (or commits) have been force-set.
        ///
        /// - **netuids**: The netuids these weights were successfully set/committed for.
        /// - **who**: The hotkey that set this batch.
        BatchWeightsCompleted(Vec<Compact<u16>>, T::AccountId),

        /// A batch extrinsic completed but with some errors.
        BatchCompletedWithErrors(),

        /// A weight set among a batch of weights failed.
        ///
        /// - **error**: The dispatch error emitted by the failed item.
        BatchWeightItemFailed(sp_runtime::DispatchError),

        /// Stake has been transferred from one coldkey to another on the same subnet.
        /// Parameters:
        /// (origin_coldkey, destination_coldkey, hotkey, origin_netuid, destination_netuid, amount)
        StakeTransferred(T::AccountId, T::AccountId, T::AccountId, u16, u16, u64),

        /// Stake has been swapped from one subnet to another for the same coldkey-hotkey pair.
        ///
        /// Parameters:
        /// (coldkey, hotkey, origin_netuid, destination_netuid, amount)
        StakeSwapped(T::AccountId, T::AccountId, u16, u16, u64),

        /// Event called when transfer is toggled on a subnet.
        ///
        /// Parameters:
        /// (netuid, bool)
        TransferToggle(u16, bool),

        /// The owner hotkey for a subnet has been set.
        ///
        /// Parameters:
        /// (netuid, new_hotkey)
        SubnetOwnerHotkeySet(u16, T::AccountId),
        /// FirstEmissionBlockNumber is set via start call extrinsic
        ///
        /// Parameters:
        /// netuid
        /// block number
        FirstEmissionBlockNumberSet(u16, u64),

        /// Alpha has been recycled, reducing AlphaOut on a subnet.
        ///
        /// Parameters:
        /// (coldkey, hotkey, amount, subnet_id)
        AlphaRecycled(T::AccountId, T::AccountId, u64, u16),

        /// Alpha have been burned without reducing AlphaOut.
        ///
        /// Parameters:
        /// (coldkey, hotkey, amount, subnet_id)
        AlphaBurned(T::AccountId, T::AccountId, u64, u16),

        /// An EVM key has been associated with a hotkey.
        EvmKeyAssociated {
            /// The subnet that the hotkey belongs to.
            netuid: u16,
            /// The hotkey associated with the EVM key.
            hotkey: T::AccountId,
            /// The EVM key being associated with the hotkey.
            evm_key: H160,
            /// The block where the association happened.
            block_associated: u64,
        },

        /// CRV3 Weights have been successfully revealed.
        ///
        /// - **netuid**: The network identifier.
        /// - **who**: The account ID of the user revealing the weights.
        CRV3WeightsRevealed(u16, T::AccountId),

        /// Commit-Reveal periods has been successfully set.
        ///
        /// - **netuid**: The network identifier.
        /// - **periods**: The number of epochs before the reveal.
        CommitRevealPeriodsSet(u16, u64),

        /// Commit-Reveal has been successfully toggled.
        ///
        /// - **netuid**: The network identifier.
        /// - **Enabled**: Is Commit-Reveal enabled.
        CommitRevealEnabled(u16, bool),

        /// the hotkey is swapped
        HotkeySwappedOnSubnet {
            /// the account ID of coldkey
            coldkey: T::AccountId,
            /// the account ID of old hotkey
            old_hotkey: T::AccountId,
            /// the account ID of new hotkey
            new_hotkey: T::AccountId,
            /// the subnet ID
            netuid: u16,
        },
    }
}

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

//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use crate::service::{AdditionalRuntimeApiCollection, RuntimeApiCollection};
use sc_client_api::{BlockBackend, ExecutorProvider};
use sc_consensus_aura::{ImportQueueParams, SlotProportion, StartAuraParams};
use sc_executor::{NativeElseWasmExecutor, NativeExecutionDispatch};
use sc_finality_grandpa::{grandpa_peers_set_config, protocol_standard_name, SharedVoterState};
use sc_keystore::LocalKeystore;
use sc_service::{
    error::Error as ServiceError, Configuration, TFullBackend, TFullClient, TaskManager,
};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sp_api::ConstructRuntimeApi;
use sp_consensus_aura::sr25519::AuthorityPair as AuraPair;
use std::{sync::Arc, time::Duration};
use zeitgeist_primitives::types::Block;

pub type FullClient<RuntimeApi, Executor> =
    TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>;
pub type FullBackend = TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

/// Builds a new service for a full client.
pub fn new_full<RuntimeApi, Executor>(
    mut config: Configuration,
    disable_hardware_benchmarks: bool,
) -> Result<TaskManager, ServiceError>
where
    RuntimeApi:
        ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>> + Send + Sync + 'static,
    RuntimeApi::RuntimeApi: RuntimeApiCollection<StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>>
        + AdditionalRuntimeApiCollection<
            StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>,
        >,
    Executor: NativeExecutionDispatch + 'static,
{
    let hwbench = if !disable_hardware_benchmarks {
        config.database.path().map(|database_path| {
            let _ = std::fs::create_dir_all(&database_path);
            sc_sysinfo::gather_hwbench(Some(database_path))
        })
    } else {
        None
    };

    let sc_service::PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        mut keystore_container,
        select_chain,
        transaction_pool,
        other: (block_import, grandpa_link, mut telemetry),
    } = new_partial::<RuntimeApi, Executor>(&config)?;

    if let Some(url) = &config.keystore_remote {
        match remote_keystore(url) {
            Ok(k) => keystore_container.set_remote_keystore(k),
            Err(e) => {
                return Err(ServiceError::Other(format!(
                    "Error hooking up remote keystore for {}: {}",
                    url, e
                )));
            }
        };
    }

    let grandpa_protocol_name = protocol_standard_name(
        &client.block_hash(0).ok().flatten().expect("Genesis block exists; qed"),
        &config.chain_spec,
    );

    config.network.extra_sets.push(grandpa_peers_set_config(grandpa_protocol_name.clone()));
    let warp_sync = Arc::new(sc_finality_grandpa::warp_proof::NetworkProvider::new(
        backend.clone(),
        grandpa_link.shared_authority_set().clone(),
        Vec::default(),
    ));

    let (network, system_rpc_tx, network_starter) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync: Some(warp_sync),
        })?;

    if config.offchain_worker.enabled {
        sc_service::build_offchain_workers(
            &config,
            task_manager.spawn_handle(),
            client.clone(),
            network.clone(),
        );
    }

    let role = config.role.clone();
    let force_authoring = config.force_authoring;
    let backoff_authoring_blocks: Option<()> = None;
    let name = config.network.node_name.clone();
    let enable_grandpa = !config.disable_grandpa;
    let prometheus_registry = config.prometheus_registry().cloned();

    let rpc_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps =
                crate::rpc::FullDeps { client: client.clone(), pool: pool.clone(), deny_unsafe };

            crate::rpc::create_full(deps).map_err(Into::into)
        })
    };

    let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        network: network.clone(),
        client: client.clone(),
        keystore: keystore_container.sync_keystore(),
        task_manager: &mut task_manager,
        transaction_pool: transaction_pool.clone(),
        rpc_builder: rpc_builder,
        backend,
        system_rpc_tx,
        config,
        telemetry: telemetry.as_mut(),
    })?;

    if let Some(hwbench) = hwbench {
        sc_sysinfo::print_hwbench(&hwbench);

        if let Some(ref mut telemetry) = telemetry {
            let telemetry_handle = telemetry.handle();
            task_manager.spawn_handle().spawn(
                "telemetry_hwbench",
                None,
                sc_sysinfo::initialize_hwbench_telemetry(telemetry_handle, hwbench),
            );
        }
    }

    if role.is_authority() {
        let proposer_factory = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool,
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|x| x.handle()),
        );

        let can_author_with =
            sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

        let slot_duration = sc_consensus_aura::slot_duration(&*client)?;

        let aura = sc_consensus_aura::start_aura::<AuraPair, _, _, _, _, _, _, _, _, _, _, _>(
            StartAuraParams {
                slot_duration,
                client,
                select_chain,
                block_import,
                proposer_factory,
                create_inherent_data_providers: move |_, ()| async move {
                    let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

                    let slot =
                        sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
                            *timestamp,
                            slot_duration,
                        );

                    Ok((timestamp, slot))
                },
                force_authoring,
                backoff_authoring_blocks,
                keystore: keystore_container.sync_keystore(),
                can_author_with,
                sync_oracle: network.clone(),
                justification_sync_link: network.clone(),
                block_proposal_slot_portion: SlotProportion::new(2f32 / 3f32),
                max_block_proposal_slot_portion: None,
                telemetry: telemetry.as_ref().map(|x| x.handle()),
            },
        )?;

        // the AURA authoring task is considered essential, i.e. if it
        // fails we take down the service with it.
        task_manager.spawn_essential_handle().spawn_blocking("aura", Some("block-authoring"), aura);
    }

    // if the node isn't actively participating in consensus then it doesn't
    // need a keystore, regardless of which protocol we use below.
    let keystore =
        if role.is_authority() { Some(keystore_container.sync_keystore()) } else { None };

    let grandpa_config = sc_finality_grandpa::Config {
        gossip_duration: Duration::from_millis(333),
        justification_period: 512,
        name: Some(name),
        observer_enabled: false,
        keystore,
        local_role: role,
        telemetry: telemetry.as_ref().map(|x| x.handle()),
        protocol_name: grandpa_protocol_name,
    };

    if enable_grandpa {
        // start the full GRANDPA voter
        // NOTE: non-authorities could run the GRANDPA observer protocol, but at
        // this point the full voter should provide better guarantees of block
        // and vote data availability than the observer. The observer has not
        // been tested extensively yet and having most nodes in a network run it
        // could lead to finality stalls.
        let grandpa_config = sc_finality_grandpa::GrandpaParams {
            config: grandpa_config,
            link: grandpa_link,
            network,
            voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
            prometheus_registry,
            shared_voter_state: SharedVoterState::empty(),
            telemetry: telemetry.as_ref().map(|x| x.handle()),
        };

        // the GRANDPA voter task is considered infallible, i.e.
        // if it fails we take down the service with it.
        task_manager.spawn_essential_handle().spawn_blocking(
            "grandpa-voter",
            None,
            sc_finality_grandpa::run_grandpa_voter(grandpa_config)?,
        );
    }

    network_starter.start_network();
    Ok(task_manager)
}

pub fn new_partial<RuntimeApi, Executor>(
    config: &Configuration,
) -> Result<
    sc_service::PartialComponents<
        FullClient<RuntimeApi, Executor>,
        FullBackend,
        FullSelectChain,
        sc_consensus::DefaultImportQueue<Block, FullClient<RuntimeApi, Executor>>,
        sc_transaction_pool::FullPool<Block, FullClient<RuntimeApi, Executor>>,
        (
            sc_finality_grandpa::GrandpaBlockImport<
                FullBackend,
                Block,
                FullClient<RuntimeApi, Executor>,
                FullSelectChain,
            >,
            sc_finality_grandpa::LinkHalf<Block, FullClient<RuntimeApi, Executor>, FullSelectChain>,
            Option<Telemetry>,
        ),
    >,
    ServiceError,
>
where
    RuntimeApi:
        ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>> + Send + Sync + 'static,
    RuntimeApi::RuntimeApi: RuntimeApiCollection<StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>>
        + AdditionalRuntimeApiCollection<
            StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>,
        >,
    Executor: NativeExecutionDispatch + 'static,
{
    if config.keystore_remote.is_some() {
        return Err(ServiceError::Other(format!("Remote Keystores are not supported.")));
    }

    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?;
            let telemetry = worker.handle().new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()?;

    let executor = NativeElseWasmExecutor::<Executor>::new(
        config.wasm_method,
        config.default_heap_pages,
        config.max_runtime_instances,
        config.runtime_cache_size,
    );

    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, _>(
            &config,
            telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
            executor,
        )?;
    let client = Arc::new(client);

    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager.spawn_handle().spawn("telemetry", None, worker.run());
        telemetry
    });

    let select_chain = sc_consensus::LongestChain::new(backend.clone());

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
        client.clone(),
        &(client.clone() as Arc<_>),
        select_chain.clone(),
        telemetry.as_ref().map(|x| x.handle()),
    )?;

    let slot_duration = sc_consensus_aura::slot_duration(&*client)?;

    let import_queue = sc_consensus_aura::import_queue::<AuraPair, _, _, _, _, _, _>(
        ImportQueueParams {
            block_import: grandpa_block_import.clone(),
            justification_import: Some(Box::new(grandpa_block_import.clone())),
            client: client.clone(),
            create_inherent_data_providers: move |_, ()| async move {
                let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

                let slot =
                    sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
                        *timestamp,
                        slot_duration,
                    );

                Ok((timestamp, slot))
            },
            spawner: &task_manager.spawn_essential_handle(),
            can_author_with: sp_consensus::CanAuthorWithNativeVersion::new(
                client.executor().clone(),
            ),
            registry: config.prometheus_registry(),
            check_for_equivocation: Default::default(),
            telemetry: telemetry.as_ref().map(|x| x.handle()),
        },
    )?;

    Ok(sc_service::PartialComponents {
        client,
        backend,
        task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (grandpa_block_import, grandpa_link, telemetry),
    })
}

fn remote_keystore(_url: &String) -> Result<Arc<LocalKeystore>, &'static str> {
    // FIXME: here would the concrete keystore be built,
    //        must return a concrete type (NOT `LocalKeystore`) that
    //        implements `CryptoStore` and `SyncCryptoStore`
    Err("Remote Keystore not supported.")
}

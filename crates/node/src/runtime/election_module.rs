use std::{
    collections::{hash_map::DefaultHasher, BTreeMap, HashMap},
    error::Error,
    fmt::Debug,
    hash::{Hash, Hasher},
};

use async_trait::async_trait;
use block::{
    header::BlockHeader,
    invalid::BlockError,
    Conflict,
    ConflictList,
    RefHash,
    ResolvedConflicts,
};
use ethereum_types::U256;
use events::{ConflictBytes, Event};
use primitives::NodeId;
use quorum::{
    election::Election,
    quorum::{InvalidQuorum, Quorum},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use storage::vrrbdb::VrrbDbReadHandle;
use telemetry::info;
use theater::{ActorId, ActorLabel, ActorState, Handler, TheaterError};
use tokio::{sync::mpsc::UnboundedSender, task::JoinHandle};
use vrrb_core::claim::Claim;

pub type Seed = u64;

pub trait ElectionType: Clone + Debug {}
pub trait ElectionOutcome: Clone + Debug {}

pub type MinerElectionResult = Vec<ElectionResult>;
pub type QuorumElectionResult = HashMap<u8, Vec<ElectionResult>>;

#[derive(Clone, Debug)]
pub struct MinerElection;

#[derive(Clone, Debug)]
pub struct QuorumElection;

pub struct ElectionModuleConfig {
    pub db_read_handle: VrrbDbReadHandle,
    pub events_tx: tokio::sync::mpsc::UnboundedSender<Event>,
    pub local_claim: Claim,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElectionResult {
    pub claim_pointer: u128,
    pub claim_hash: String,
    pub node_id: NodeId,
}

#[derive(Clone, Debug)]
pub struct ElectionModule<E, T>
where
    E: ElectionType,
    T: ElectionOutcome,
{
    election_type: E,
    status: ActorState,
    id: ActorId,
    label: ActorLabel,
    pub db_read_handle: VrrbDbReadHandle,
    pub local_claim: Claim,
    pub outcome: Option<T>,
    pub events_tx: tokio::sync::mpsc::UnboundedSender<Event>,
}

impl ElectionModule<MinerElection, MinerElectionResult> {
    pub fn new(config: ElectionModuleConfig) -> ElectionModule<MinerElection, MinerElectionResult> {
        ElectionModule {
            election_type: MinerElection,
            status: ActorState::Stopped,
            id: uuid::Uuid::new_v4().to_string(),
            label: String::from("Election module"),
            db_read_handle: config.db_read_handle,
            local_claim: config.local_claim,
            outcome: None,
            events_tx: config.events_tx,
        }
    }

    pub fn name(&self) -> ActorLabel {
        String::from("Miner Election Module")
    }
}

impl ElectionModule<QuorumElection, QuorumElectionResult> {
    pub fn new(
        config: ElectionModuleConfig,
    ) -> ElectionModule<QuorumElection, QuorumElectionResult> {
        ElectionModule {
            election_type: QuorumElection,
            status: ActorState::Stopped,
            id: uuid::Uuid::new_v4().to_string(),
            label: String::from("Election module"),
            db_read_handle: config.db_read_handle,
            local_claim: config.local_claim,
            outcome: None,
            events_tx: config.events_tx,
        }
    }

    pub fn name(&self) -> ActorLabel {
        String::from("Quorum Election Module")
    }
}

impl ElectionType for MinerElection {}
impl ElectionType for QuorumElection {}

impl ElectionOutcome for MinerElectionResult {}
impl ElectionOutcome for QuorumElectionResult {}

#[async_trait]
impl Handler<Event> for ElectionModule<MinerElection, MinerElectionResult> {
    fn id(&self) -> ActorId {
        self.id.clone()
    }

    fn label(&self) -> ActorLabel {
        self.name()
    }

    fn status(&self) -> ActorState {
        self.status.clone()
    }

    fn set_status(&mut self, actor_status: ActorState) {
        self.status = actor_status;
    }

    fn on_stop(&self) {
        info!(
            "{}-{} received stop signal. Stopping",
            self.name(),
            self.label()
        );
    }

    async fn handle(&mut self, event: Event) -> theater::Result<ActorState> {
        match event {
            Event::MinerElection(header_bytes) => {
                let header_result: serde_json::Result<BlockHeader> =
                    serde_json::from_slice(&header_bytes);
                if let Ok(header) = header_result {
                    let claims = self.db_read_handle.claim_store_values();
                    let mut election_results: BTreeMap<U256, Claim> =
                        elect_miner(claims, header.block_seed);
                    let winner = get_winner(&mut election_results);

                    let _ = self.events_tx.send(Event::ElectedMiner(winner));
                }
            },
            _ => {},
        }

        Ok(ActorState::Running)
    }
}

#[async_trait]
impl Handler<Event> for ElectionModule<QuorumElection, QuorumElectionResult> {
    fn id(&self) -> ActorId {
        self.id.clone()
    }

    fn label(&self) -> ActorLabel {
        self.name()
    }

    fn status(&self) -> ActorState {
        self.status.clone()
    }

    fn set_status(&mut self, actor_status: ActorState) {
        self.status = actor_status;
    }

    fn on_stop(&self) {
        info!(
            "{}-{} received stop signal. Stopping",
            self.name(),
            self.label()
        );
    }

    async fn handle(&mut self, event: Event) -> theater::Result<ActorState> {
        match event {
            Event::QuorumElection(header_bytes) => {
                let header_result: serde_json::Result<BlockHeader> =
                    serde_json::from_slice(&header_bytes);

                if let Ok(header) = header_result {
                    let claims = self.db_read_handle.claim_store_values();
                    if let Ok(quorum) = elect_quorum(claims, header) {
                        let _ = self.events_tx.send(Event::ElectedQuorum(quorum));
                    }
                }
            },
            _ => {},
        }

        Ok(ActorState::Running)
    }
}

fn elect_miner(claims: HashMap<NodeId, Claim>, block_seed: u64) -> BTreeMap<U256, Claim> {
    claims
        .iter()
        .filter(|(_, claim)| claim.eligible)
        .map(|(nodeid, claim)| single_miner_results(claim, block_seed))
        .collect()
}

fn single_miner_results(claim: &Claim, block_seed: u64) -> (U256, Claim) {
    (claim.get_election_result(block_seed), claim.clone())
}

fn get_winner(election_results: &mut BTreeMap<U256, Claim>) -> (U256, Claim) {
    let mut iter = election_results.iter();
    let first: (U256, Claim);
    loop {
        if let Some((pointer_sum, claim)) = iter.next() {
            first = (pointer_sum.clone(), claim.clone());
            break;
        }
    }

    return first;
}

fn elect_quorum(
    claims: HashMap<NodeId, Claim>,
    header: BlockHeader,
) -> Result<Quorum, InvalidQuorum> {
    let last_block_height = header.block_height;
    let seed = header.next_block_seed;

    if let Ok(mut quorum) = Quorum::new(seed, last_block_height) {
        let claim_vec: Vec<Claim> = claims.values().cloned().collect();
        if let Ok(elected_quorum) = quorum.run_election(claim_vec) {
            return Ok(elected_quorum.clone());
        }
    }

    return Err(InvalidQuorum::InvalidSeedError());
}
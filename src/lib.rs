#[macro_use]
extern crate wascc_codec as codec;

#[macro_use]
extern crate log;

use crate::kv::KeyValueStore;
use codec::capabilities::{CapabilityProvider, Dispatcher, NullDispatcher};
use codec::core::OP_CONFIGURE;
use prost::Message; // Required for 'decode's.
use wascc_codec::core::CapabilityConfiguration;

use std::error::Error;
use std::sync::RwLock;

pub mod kv;

capability_provider!(WasccKeyvalueProvider, WasccKeyvalueProvider::new);

const CAPABILITY_ID: &str = "wascc::keyvalue";

pub struct WasccKeyvalueProvider {
    dispatcher: RwLock<Box<dyn Dispatcher>>,
    store: RwLock<KeyValueStore>,
}

impl Default for WasccKeyvalueProvider {
    fn default() -> Self {
        env_logger::init();

        WasccKeyvalueProvider {
            dispatcher: RwLock::new(Box::new(NullDispatcher::new())),
            store: RwLock::new(KeyValueStore::new()),
        }
    }
}

impl WasccKeyvalueProvider {
    pub fn new() -> Self {
        Self::default()
    }

    fn configure(
        &self,
        config: impl Into<CapabilityConfiguration>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let _config = config.into();

        Ok(vec![])
    }

    fn remove_actor(&self, _config: CapabilityConfiguration) -> Result<Vec<u8>, Box<dyn Error>> {
        trace!("remove_actor noop");

        Ok(Vec::new())
    }
}

impl CapabilityProvider for WasccKeyvalueProvider {
    fn capability_id(&self) -> &'static str {
        CAPABILITY_ID
    }

    // Invoked by the runtime host to give this provider plugin the ability to communicate
    // with actors
    fn configure_dispatch(&self, dispatcher: Box<dyn Dispatcher>) -> Result<(), Box<dyn Error>> {
        trace!("Dispatcher received.");
        let mut lock = self.dispatcher.write().unwrap();
        *lock = dispatcher;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "waSCC sample in-memory KV provider"
    }

    // Invoked by host runtime to allow an actor to make use of the capability
    // All providers MUST handle the "configure" message, even if no work will be done
    fn handle_call(&self, actor: &str, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        trace!("Received host call from {}, operation - {}", actor, op);

        match op {
            OP_CONFIGURE if actor == "system" => self.configure(msg.to_vec().as_ref()),
            OP_REMOVE_ACTOR if actor == "system" => {
                self.remove_actor(CapabilityConfiguration::decode(msg).unwrap())
            }
            _ => Err("bad dispatch".into()),
        }
    }
}

use std::sync::Arc;

use crate::{
    network_dispatcher::NetworkDispatcher, network_handler::NetworkHandler,
    sound_engine::SoundEngine,
};

pub struct AppService {
    network_handler: NetworkHandler,
    network_dispatcher: NetworkDispatcher,
}

impl AppService {
    pub fn new() -> Self {
        let sound_engine = Arc::new(SoundEngine::new());
        Self {
            network_handler: NetworkHandler::new(sound_engine.clone()),
            network_dispatcher: NetworkDispatcher::new(),
        }
    }

    pub fn dispatch_sample(&self) {
        self.network_dispatcher
            .dispatch_sound("localhost:3000".to_owned(), 1, "sound".to_owned())
            .unwrap();
    }

    pub fn listen(&self) {
        self.network_handler.listen();
    }
}

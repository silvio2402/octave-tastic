use crate::{
    network_dispatcher::NetworkDispatcher, network_handler::NetworkHandler,
    sound_engine::SoundEngine,
};

pub struct AppService {
    sound_engine: SoundEngine,
    network_handler: NetworkHandler,
    network_dispatcher: NetworkDispatcher,
}

impl AppService {
    pub fn new() -> Self {
        Self {
            sound_engine: SoundEngine::new(),
            network_handler: NetworkHandler::new(),
            network_dispatcher: NetworkDispatcher::new(),
        }
    }

    pub fn dispatch_sample(&self) {
        self.network_dispatcher
            .dispatch_sound("localhost:3000".to_owned(), 1, "sound".to_owned());
    }

    pub fn listen(&self) {
        self.network_handler.listen();
    }
}

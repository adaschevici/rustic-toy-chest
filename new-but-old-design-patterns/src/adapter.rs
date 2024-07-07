use async_trait::async_trait;

#[async_trait]
trait Animal {
    async fn make_sound(&self);
}

struct Lion;
struct Elephant;

#[async_trait]
impl Animal for Lion {
    async fn make_sound(&self) {
        println!("Roar!");
    }
}

#[async_trait]
impl Animal for Elephant {
    async fn make_sound(&self) {
        println!("Trumpet!");
    }
}

struct EnrichmentDevice {
    sound: String,
}

impl EnrichmentDevice {
    async fn play_sound(&self) {
        println!("Playing sound: {}", self.sound);
    }
}

struct AnimalSoundAdapter {
    enrichment_device: EnrichmentDevice,
}

#[async_trait]
impl Animal for AnimalSoundAdapter {
    async fn make_sound(&self) {
        self.enrichment_device.play_sound().await;
    }
}

pub async fn run_adapter() {
    let lion_sound = EnrichmentDevice {
        sound: "Roar!".to_string(),
    };
    let elephant_sound = EnrichmentDevice {
        sound: "Trumpet!".to_string(),
    };
    let lion = AnimalSoundAdapter {
        enrichment_device: lion_sound,
    };
    let elephant = AnimalSoundAdapter {
        enrichment_device: elephant_sound,
    };
    lion.make_sound().await; // Outputs: Playing sound: Roar!
    elephant.make_sound().await; // Outputs: Playing sound: Trumpet!
}

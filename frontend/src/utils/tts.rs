use natural_tts::{models::gtts::GttsModel, Model, NaturalTtsBuilder};
use std::error::Error;

pub fn say_message(message: String) -> Result<(), Box<dyn Error>> {
    let mut natural = NaturalTtsBuilder::default()
        .default_model(Model::Gtts)
        .gtts_model(GttsModel::default())
        .build()?;

    // Use the pre-included function to say a message using the default_model.
    let _ = natural.say_auto(message)?;

    Ok(())
}

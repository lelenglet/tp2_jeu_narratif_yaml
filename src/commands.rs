use crate::scene::Story;

pub trait GameCommand {
    fn execute(&self, story: &mut Story, state: &mut GameState) -> Result<CommandOutcome, GameError>;
}

pub struct ChooseCommand { pub destination: String }
impl GameCommand for ChooseCommand {
    fn execute(&self, story: &mut Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        let current_scene = story.scenes.iter()
            .find(|s| s.id == state.current_scene_id)
            .ok_or_else(|| GameError::CommandError("Erreur critique : Scène actuelle introuvable".into()))?;

        if let Some(choices) = &current_scene.choices {
            let matching_choice = choices.iter().find(|c| {
                c.label.to_lowercase() == self.destination.to_lowercase() ||
                    c.next.to_lowercase() == self.destination.to_lowercase()
            });

            if let Some(choice) = matching_choice {
                if let Some(req_item) = &choice.required_item {
                    if !state.inventory.contains(req_item) {
                        return Err(GameError::MissingItem(req_item.clone()));
                    }
                }

                state.current_scene_id = choice.next.clone();

                println!("Vous vous déplacez vers : {}", choice.label);

                Ok(CommandOutcome::UpdateState)
            } else {
                Err(GameError::InvalidChoice(format!(
                    "Vous ne pouvez pas aller à '{}' d'ici.",
                    self.destination
                )))
            }
        } else {
            Err(GameError::InvalidChoice("Il n'y a aucune issue ici.".into()))
        }
    }
}
pub struct InventoryCommand;
impl GameCommand for InventoryCommand {
    fn execute(&self, _story: &mut Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        println!("Sac à dos : {:?}", state.inventory);
        Ok(CommandOutcome::Continue)
    }
}

pub struct StatusCommand;
impl GameCommand for StatusCommand {
    fn execute(&self, _story: &mut Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        println!("Points de Vie : {} HP | Scène courante : {}", state.current_hp, state.current_scene_id);
        Ok(CommandOutcome::Continue)
    }
}
pub struct QuitCommand;
impl GameCommand for QuitCommand {
    fn execute(&self, _story: &mut Story, _state: &mut GameState) -> Result<CommandOutcome, GameError> {
        println!("Merci d'avoir joué !");
        Ok(CommandOutcome::Exit)
    }
}

pub struct LookCommand;
impl GameCommand for LookCommand {
    fn execute(&self, story: &mut Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        if let Some(scene) = story.scenes.iter().find(|s| s.id == state.current_scene_id) {
            println!("\n--- {} ---", scene.title);
            println!("{}", scene.text);
            if let Some(choices) = &scene.choices {
                println!("\nChoix disponibles :");
                for (i, c) in choices.iter().enumerate() {
                    println!("{}. {} ({})", i + 1, c.label, c.next);
                }
            }
            Ok(CommandOutcome::Continue)
        } else {
            Err(GameError::CommandError("Scène introuvable".into()))
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnknownCommand(String),
    EmptyInput,
    MissingArgument(String),
}

pub struct GameState {
    pub current_scene_id: String,
    pub current_hp: i32,
    pub inventory: Vec<String>,
}

pub enum CommandOutcome {
    Continue,    // Le jeu continue normalement
    UpdateState, // L'état a changé (ex: changement de pièce)
    Exit,        // L'utilisateur veut quitter
}


#[derive(Debug)]
pub enum GameError {
    InvalidChoice(String),
    MissingItem(String),
    CommandError(String),
}







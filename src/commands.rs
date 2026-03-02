use crate::scene::Story;
use crate::project_error::*;

pub struct GameState {
    pub current_scene_id: String,
    pub current_hp: i32,
    pub inventory: Vec<String>,
}

pub enum CommandOutcome {
    Continue,
    UpdateState,
    Exit,
}

pub trait GameCommand {
    fn execute(&self, story: &mut Story, state: &mut GameState) -> Result<CommandOutcome, GameError>;
}

pub struct ChooseCommand { pub index: usize} // déplacement vers un choix

impl GameCommand for ChooseCommand {
    fn execute(&self, story: &mut Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        let current_scene = story.scenes.iter()
            .find(|s| s.id == state.current_scene_id)
            .ok_or_else(|| GameError::CommandError("Scène actuelle introuvable".into()))?;

        if let Some(choices) = &current_scene.choices {
            let choice = choices.get(self.index - 1)
                .ok_or_else(|| GameError::InvalidChoice(format!("Le numéro {} n'existe pas.", self.index)))?;

            if let Some(req_item) = &choice.required_item {
                if !state.inventory.contains(req_item) {
                    return Err(GameError::MissingItem(req_item.clone()));
                }
            }
            state.current_scene_id = choice.next.clone();
            println!("Vous avez choisi : {}", choice.label);

            Ok(CommandOutcome::UpdateState)
        } else {
            Err(GameError::InvalidChoice("Aucun choix possible pour cette scène.".into()))
        }
    }
}
pub struct InventoryCommand; // affichage de l'inventaire
impl GameCommand for InventoryCommand {
    fn execute(&self, _story: &mut Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        println!("Sac à dos : {:?}", state.inventory);
        Ok(CommandOutcome::Continue)
    }
}

pub struct StatusCommand; // affichage du status de la partie
impl GameCommand for StatusCommand {
    fn execute(&self, _story: &mut Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        println!("Points de Vie : {} HP | Scène courante : {}", state.current_hp, state.current_scene_id);
        Ok(CommandOutcome::Continue)
    }
}
pub struct QuitCommand; // quitter le programme
impl GameCommand for QuitCommand {
    fn execute(&self, _story: &mut Story, _state: &mut GameState) -> Result<CommandOutcome, GameError> {
        println!("Merci d'avoir joué !");
        Ok(CommandOutcome::Exit)
    }
}

pub struct LookCommand; // affichage de la scène courante
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














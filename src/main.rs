use std::io::{self, Write};
use tp2_jeu_narratif_yaml::scene::Story;
use tp2_jeu_narratif_yaml::commands::{GameState, CommandOutcome, LookCommand,GameCommand};

fn main() {
    let story_path = "story.yaml";
    let mut story = Story::deserialyze_yaml_to_story(story_path)
        .expect("Erreur lors de la désérialisation du scénario");

    Story::validate_story(&story).expect("Échec de la validation du scénario");

    let mut state = GameState {
        current_scene_id: story.get_start_scene().to_string(),
        current_hp: story.get_initial_hp(),
        inventory: vec![],
    };

    println!("Bienvenue dans l'aventure !\n");

    let first_look = LookCommand;
    let _ = first_look.execute(&mut story, &mut state);

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Erreur lors de la lecture de l'entrée.");
            continue;
        }
        match Story::parse_command(&input) {
            Ok(command) => {
                match command.execute(&mut story, &mut state) {
                    Ok(CommandOutcome::Exit) => {
                        break;
                    }
                    Ok(CommandOutcome::UpdateState) => {
                        let _ = LookCommand.execute(&mut story, &mut state);
                    }
                    Ok(CommandOutcome::Continue) => (),
                    Err(e) => println!("Erreur : {:?}", e),
                }
            }
            Err(e) => println!("Commande inconnue ou incomplète : {:?}", e),
        }

        if state.current_hp <= 0 {
            println!("\n--- GAME OVER ---");
            break;
        }
    }
}
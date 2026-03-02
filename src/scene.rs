use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use crate::project_error::ParseError;
use crate::commands::{GameCommand,InventoryCommand,StatusCommand,ChooseCommand,LookCommand,QuitCommand};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Story{
    start_scene : String,
    initial_hp : i32,
    pub scenes : Vec<Scene>,
}

impl Story {
    pub fn get_start_scene(&self) -> &str { &self.start_scene }
    pub fn get_initial_hp(&self) -> i32 { self.initial_hp }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Scene {
    pub id: String,
    pub title: String,
    pub text : String,
    #[serde(default)]
    hp_delta:Option<i32>,
    #[serde(default)]
    found_item :Option<String>,
    #[serde(default)]
    ending : Option<String>,
    #[serde(default)]
    pub choices: Option<Vec<Choice>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Choice{
    pub label : String,
    pub next : String,
    #[serde(default)]
    pub required_item:Option<String>,
}

impl Story{
    pub fn deserialyze_yaml_to_story(filename: &str) ->  Result<Story,serde_yaml::Error> { // fichier yaml -> scénario
        let mut file:File = File::open(filename).expect("File not found");
        let mut story_string:String = String::new();
        file.read_to_string(&mut story_string).expect("Can't read file");
        let story:Story = serde_yaml::from_str(&story_string)?;
        Ok(story)
    }
    
    pub fn validate_story(story: &Story) -> Result<(), String> { //valide la bonne formation de notre scénario
        let mut scene_ids = HashSet::new();
        for scene in &story.scenes {
            if !scene_ids.insert(&scene.id) {
                return Err(format!("ID de scène dupliqué détecté : {}", scene.id));
            }
        }
        if !scene_ids.contains(&story.start_scene) {
            return Err(format!(
                "La scène de départ '{}' n'existe pas dans la liste des scènes.",
                story.start_scene
            ));
        }
        for scene in &story.scenes {
            if let Some(choices) = &scene.choices {
                for c in choices {
                    if !scene_ids.contains(&c.next) {
                        return Err(format!(
                            "Dans la scène '{}', le choix '{}' pointe vers une destination inexistante : '{}'",
                            scene.id, c.label, c.next
                        ));
                    }
                }
            }
        }
        println!("Validation réussie : Le scénario est cohérent.");
        Ok(())
    }

    pub fn parse_command(line: &str) -> Result<Box<dyn GameCommand>, ParseError> { // redirection de l'entrée utilisateur
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Err(ParseError::EmptyInput);
        }
        let command_word = parts[0].to_lowercase();
        let args = &parts[1..];
        match command_word.as_str() {
            "look" => {
                Ok(Box::new(LookCommand))
            },
            "choose" => {
                if let Some(arg) = args.first() {
                    match arg.parse::<usize>() {
                        Ok(num) => Ok(Box::new(ChooseCommand { index: num })),
                        Err(_) => Err(ParseError::UnknownCommand("Veuillez entrer un numéro valide (ex: choose 1)".to_string())),
                    }
                } else {
                    Err(ParseError::MissingArgument("Usage: choose <numéro>".to_string()))
                }
            },
            "inventory" => {
                Ok(Box::new(InventoryCommand))
            },
            "status" => {
                Ok(Box::new(StatusCommand))
            },
            "quit" => {
                Ok(Box::new(QuitCommand))
            },
            _ => Err(ParseError::UnknownCommand(command_word)),
        }
    }
}

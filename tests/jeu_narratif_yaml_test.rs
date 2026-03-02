#[cfg(test)]
mod tests {
    use tp2_jeu_narratif_yaml::scene::{Story};
    use tp2_jeu_narratif_yaml::commands::{ChooseCommand,GameState};
    use tp2_jeu_narratif_yaml::{GameCommand, GameError};

    fn setup_test_story() -> Story {
        serde_yaml::from_str(r#"
            start_scene: "start"
            initial_hp: 10
            items: []
            scenes:
              - id: "start"
                title: "Départ"
                text: "Une porte et un gouffre."
                choices:
                  - label: "Sortie"
                    next: "victory"
                  - label: "Piège"
                    next: "death"
                  - label: "Porte Verrouillée"
                    next: "victory"
                    required_item: "clé"
              - id: "victory"
                title: "Victoire"
                text: "Gagné !"
                ending: "Bravo"
              - id: "death"
                title: "Mort"
                text: "Aïe."
                hp_delta: -20
        "#).unwrap()
    }

    #[test]
    fn test_nominal_path_to_victory() {
        let mut story = setup_test_story();
        let mut state = GameState {
            current_scene_id: "start".to_string(),
            current_hp: 10,
            inventory: vec![],
        };

        let cmd = ChooseCommand { index: 1 };
        let result = cmd.execute(&mut story, &mut state);

        assert!(result.is_ok());
        assert_eq!(state.current_scene_id, "victory");
    }

    #[test]
    fn test_invalid_choice_index() {
        let mut story = setup_test_story();
        let mut state = GameState {
            current_scene_id: "start".to_string(),
            current_hp: 10,
            inventory: vec![],
        };

        let cmd = ChooseCommand { index: 3 };
        let result = cmd.execute(&mut story, &mut state);

        match result {
            Err(GameError::MissingItem(_)) => assert!(true),
            _ => panic!("Devrait retourner MissingItem"),
        }
    }

    #[test]
    fn test_conditional_choice_without_item() {
        let mut story = setup_test_story();
        let mut state = GameState {
            current_scene_id: "start".to_string(),
            current_hp: 10,
            inventory: vec![], // Inventaire vide
        };

        // Le choix index 3 requiert une "clé"
        let cmd = ChooseCommand { index: 3  };
        let result = cmd.execute(&mut story, &mut state);

        match result {
            Err(GameError::MissingItem(item)) => assert_eq!(item, "clé"),
            _ => panic!("Devrait bloquer car la clé manque"),
        }
    }

    #[test]
    fn test_game_over_hp_loss() {
        let mut story = setup_test_story();
        let mut state = GameState {
            current_scene_id: "start".to_string(),
            current_hp: 10,
            inventory: vec![],
        };

        // On va vers la scène "death" (hp_delta: -20)
        let cmd = ChooseCommand { index: 2 };
        let _ = cmd.execute(&mut story, &mut state);

        assert_eq!(state.current_scene_id, "death");
    }

    #[test]
    fn test_invalid_choice_index_bound() {
        let mut story = setup_test_story();
        let mut state = GameState {
            current_scene_id: "start".to_string(),
            current_hp: 10,
            inventory: vec![],
        };

        let cmd = ChooseCommand { index: 99 };
        let result = cmd.execute(&mut story, &mut state);

        match result {
            Err(GameError::InvalidChoice(_)) => assert!(true),
            _ => panic!("Devrait retourner InvalidChoice"),
        }
    }
    #[test]
    fn test_invalid_yaml_validation() {
        let yaml = r#"
            start_scene: "inconnue"
            initial_hp: 10
            scenes:
              - id: "start"
                title: "Titre"
                text: "Text"
        "#;
        let story: Story = serde_yaml::from_str(yaml).unwrap();
        let result = Story::validate_story(&story);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("n'existe pas"));
    }
}
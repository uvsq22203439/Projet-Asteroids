//! Module `main.rs`
//!
//! Ce module définit les principales fonctions qui permettent de faire tourner le jeu, il gère la logique globale.

use asteroid::Asteroid;
use macroquad::prelude::*;
use macroquad::prelude::{load_texture, DrawTextureParams, Texture2D};
use missile::Missile;
use spaceship::Spaceship;
use stellarobject::StellarObject;

mod asteroid;
mod missile;
mod spaceship;
mod stellarobject;

/// Configure les paramètres de la fenêtre pour le jeu.
/// 
/// Définit le titre de la fenêtre et active le mode plein écran.
/// Retourne un objet de configuration utilisé par Macroquad.
fn window_conf() -> Conf {
    Conf {
        window_title: "Projet Jeu Asteroid".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

/// Gère les entrées utilisateur pour contrôler le vaisseau et tirer des missiles.
/// 
/// - `spaceship`: Référence mutable vers le vaisseau spatial.
/// - `missiles`: Référence mutable vers la liste des missiles existants.
/// 
/// Retourne `true` si le joueur appuie sur Échap pour quitter le jeu.

fn handle_input(spaceship: &mut Spaceship, missiles: &mut Vec<Missile>) -> bool {
    if is_key_down(KeyCode::Escape) {
        return true;
    }
    // Gestion de la propulsion.
    if is_key_down(KeyCode::Right) {
        spaceship.turn_right();
    }
    if is_key_down(KeyCode::Left) {
        spaceship.turn_left();
    }
    if is_key_down(KeyCode::Up) {
        spaceship.apply_thrust(0.005);
    } else if is_key_down(KeyCode::Down) {
        spaceship.apply_thrust(-0.0025);
    } else {
        spaceship.apply_thrust(0.0);
    }
    // Tir d'un missile lorsque la barre d'espace est pressée.
    if is_key_pressed(KeyCode::Space) {
        let missile_position = spaceship.position()
            + vec2(
                spaceship.radius() * spaceship.angle().cos(),
                spaceship.radius() * spaceship.angle().sin(),
            );
        missiles.push(Missile::new(missile_position, spaceship.angle()));
    }
    false
}

/// Dessine l'arrière-plan du jeu.
/// 
/// - `texture`: Texture utilisée pour l'arrière-plan.
/// - `opacity`: Opacité appliquée à la texture.

fn draw_background(texture: &Texture2D, opacity: f32) {
    draw_texture_ex(
        texture,
        0.0,
        0.0,
        Color::new(1.0, 1.0, 1.0, opacity),
        DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        },
    );
}

/// Met à jour les positions des astéroïdes, du vaisseau et des missiles.
/// 
/// - `asteroids`: Référence mutable vers la liste des astéroïdes.
/// - `spaceship`: Référence mutable vers le vaisseau spatial.
/// - `missiles`: Référence mutable vers la liste des missiles.

fn update_model(
    asteroids: &mut [Asteroid],
    spaceship: &mut Spaceship,
    missiles: &mut [Missile],
) {
    for asteroid in asteroids.iter_mut() {
        asteroid.update_position();
    }

    spaceship.update_position();

    for missile in missiles.iter_mut() {
        missile.update_position();
    }
}

/// Vérifie les collisions entre les objets du jeu (vaisseau, astéroïdes, missiles).
/// 
/// - `spaceship`: Référence mutable vers le vaisseau spatial.
/// - `asteroids`: Référence mutable vers la liste des astéroïdes.
/// - `missiles`: Référence mutable vers la liste des missiles.
/// - `score`: Référence mutable vers le score actuel.
/// 
/// Retourne `true` si le vaisseau entre en collision avec un astéroïde.


fn check_collision_game(
    spaceship: &mut Spaceship,
    asteroids: &mut Vec<Asteroid>,
    missiles: &mut Vec<Missile>,
    score: &mut i32,
) -> bool {
    // Vérification des collisions entre le vaisseau et les astéroïdes
    let mut split_asteroids = Vec::new();
    let mut spaceship_collision = false;

    // Détecte les collisions entre le vaisseau et les astéroïdes.
    asteroids.retain(|asteroid| {
        if asteroid.check_collision(spaceship) {
            split_asteroids = asteroid.split();
            if !spaceship.invincible {
                *score -= 10; // Mise à jour du score si collision vaisseau/astéroide (-)
                spaceship_collision = true;
            }
            false // Supprimer l'astéroïde touché
        } else { 
            true // Conserver l'astéroïde
        }
    });
    // Vérification des collisions entre missiles et astéroïdes
    missiles.retain(|missile| {
        let mut hit = false;
        asteroids.retain(|asteroid| {
            if missile.position().distance(asteroid.position()) < asteroid.radius() {
                hit = true;
                *score += 5; // Mise à jour du score (+)
                split_asteroids = asteroid.split(); // Ajouter les astéroïdes créés par la division
                false // Supprimer l'astéroïde touché
            } else {
                true // Conserver l'astéroïde
            }
        });

        !hit && !missile.is_out_of_bounds()
    });

    // Ajouter les nouveaux astéroïdes créés par la division
    asteroids.append(&mut split_asteroids); // Ajoute les astéroïdes créés par la division.

    spaceship_collision // Renvoie la valeur true ou false en fonction de si il y a un collision vaisseau/astéroide 
}


/// Dessine les objets du jeu (fond, astéroïdes, missiles, vaisseau, score).
/// 
/// - `background`: Texture de l'arrière-plan.
/// - `asteroids`: Liste des astéroïdes à dessiner.
/// - `missiles`: Liste des missiles à dessiner.
/// - `spaceship`: Référence mutable vers le vaisseau spatial.
/// - `score`: Score actuel du joueur.

fn draw_game(
    background: &Texture2D,
    asteroids: &mut Vec<Asteroid>,
    missiles: &mut Vec<Missile>,
    spaceship: &mut Spaceship,
    score: i32,
) {
    draw_texture_ex(
        background,
        0.0,
        0.0,
        Color::new(1.0, 1.0, 1.0, 1.0),
        DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        },
    );

    for asteroid in asteroids {
        asteroid.draw();
    }
    for missile in missiles {
        missile.draw();
    }
    spaceship.draw();

     // Affiche les informations sur l'écran
    let text_shield = format!("Vies : {}", spaceship.shield + 1);
    let text_score = format!("Score : {}", score);

    draw_text(
        &text_shield,
        screen_width() / 2.0 - 200.0,
        screen_height() * 0.05,
        50.0,
        GREEN,
    );
    draw_text(
        &text_score,
        screen_width() / 2.0 + 100.0,
        screen_height() * 0.05,
        50.0,
        GREEN,
    );
}


/// Point d'entrée principal du jeu.
///
/// Charge les ressources, initialise les objets du jeu, et contrôle
/// les différentes étapes de la boucle de jeu.

#[macroquad::main(window_conf)]
async fn main() {
    // Chargement des textures nécessaires au jeu.
    let background_texture: Texture2D = load_texture("src/space_bg.png").await.unwrap();
    let spaceship_texture: Texture2D = load_texture("src/spaceship.png").await.unwrap();
    let shield_texture: Texture2D = load_texture("src/shield.png").await.unwrap();
    let asteroid_texture: Texture2D = load_texture("src/asteroid.png").await.unwrap();

    // Initialisation des objets du jeu.
    let mut spaceship: Spaceship = Spaceship::new(&spaceship_texture, &shield_texture);
    let mut nbr_asteroids: usize = 10;
    let mut asteroids: Vec<Asteroid> =
        Asteroid::generate_asteroid(nbr_asteroids, &asteroid_texture);
    let mut missiles: Vec<Missile> = Vec::new();
    let mut score: i32 = 0;
    let mut game_status: bool = true;
    let mut invincibility_time: f64 = 0.0;
    let mut start_screen: bool = true;

    // Boucle principale.
    loop {
        if start_screen {
            draw_background(&background_texture, 0.5);
            draw_text(
                "Mini-projet Asteroids",
                screen_width() / 2.0 - 700.0,
                screen_height() / 2.0 - 100.0,
                150.0,
                WHITE,
            );                                                      // Affichage de l'écran de démarrage.
            draw_text(
                "Réalisé par LALMASSI Ilyan & BRULU Thomas",
                screen_width() / 2.0 - 500.0,
                screen_height() / 2.0 + 100.0,
                50.0,
                WHITE,
            );

            let button_x = screen_width() / 2.0 - 100.0;
            let button_y = screen_height() / 2.0 + 200.0;
            let button_width = 200.0;
            let button_height = 75.0;
            draw_rectangle(button_x, button_y, button_width, button_height, GREEN);
            draw_text(" Jouer", button_x + 20.0, button_y + 50.0, 50.0, BLACK);
            draw_rectangle(button_x, button_y + 100.0, button_width, button_height, RED);
            draw_text("Quitter", button_x + 20.0, button_y + 150.0, 50.0, BLACK);

            if is_mouse_button_pressed(MouseButton::Left) {
                let mouse_position = mouse_position();
                if mouse_position.0 >= button_x
                    && mouse_position.0 <= button_x + button_width
                    && mouse_position.1 >= button_y
                    && mouse_position.1 <= button_y + button_height
                {
                    start_screen = false;
                    game_status = true;
                } else if mouse_position.0 >= button_x
                    && mouse_position.0 <= button_x + button_width
                    && mouse_position.1 >= button_y + 100.0
                    && mouse_position.1 <= button_y + 100.0 + button_height
                {
                    break;
                }
            }
        } else if game_status {
            if asteroids.is_empty() {
                game_status = false;          // Arrêt du jeu si plus d'astéroides
            }

            draw_game(
                &background_texture,
                &mut asteroids,
                &mut missiles,
                &mut spaceship,
                score,
            );

            if spaceship.invincible && get_time() - invincibility_time >= 1.0 {spaceship.invincible = false;}

            if check_collision_game(&mut spaceship, &mut asteroids, &mut missiles, &mut score) {
                if spaceship.shield > 0 {
                    spaceship.invincible = true;
                    spaceship.shield -= 1;                          // Mise à jour du bouclier et arrêt du jeu si plus de vie.
                    invincibility_time = get_time();
                } else {
                    game_status = false;
                }
            }

            if handle_input(&mut spaceship, &mut missiles) {
                break;
            }

            update_model(&mut asteroids, &mut spaceship, &mut missiles);
        } else {
            draw_background(&background_texture, 0.5);
            draw_text(
                "Game Over!",
                screen_width() / 2.0 - 650.0,
                screen_height() / 2.0 - 50.0,
                300.0,
                RED,
            );
            draw_text(                                              // Ecran de fin de jeu.
                &format!("Score final : {}", score),
                screen_width() / 2.0 - 200.0,
                screen_height() / 2.0 + 100.0,
                50.0,
                WHITE,
            );

            let button_x = screen_width() / 2.0 - 100.0;
            let button_y = screen_height() / 2.0 + 200.0;
            let button_width = 200.0;
            let button_height = 75.0;
            draw_rectangle(button_x, button_y, button_width, button_height, GREEN);
            draw_text("Rejouer", button_x + 10.0, button_y + 50.0, 50.0, BLACK);
            draw_rectangle(button_x, button_y + 100.0, button_width, button_height, RED);
            draw_text("Quitter", button_x + 20.0, button_y + 150.0, 50.0, BLACK);

            if is_mouse_button_pressed(MouseButton::Left) {
                let mouse_position = mouse_position();
                if mouse_position.0 >= button_x
                    && mouse_position.0 <= button_x + button_width
                    && mouse_position.1 >= button_y
                    && mouse_position.1 <= button_y + button_height
                {
                    spaceship = Spaceship::new(&spaceship_texture, &shield_texture);
                    nbr_asteroids += 5;
                    asteroids = Asteroid::generate_asteroid(nbr_asteroids, &asteroid_texture);
                    missiles.clear();
                    score = 0;
                    game_status = true;
                } else if mouse_position.0 >= button_x
                    && mouse_position.0 <= button_x + button_width
                    && mouse_position.1 >= button_y + 100.0
                    && mouse_position.1 <= button_y + 100.0 + button_height
                {
                    break;
                }
            }
        }
        next_frame().await;
    }
}


/// Teste la méthode `check_collision` du trait `StellarObject`.
/// 
/// Ce test vérifie si la méthode `check_collision` fonctionne correctement pour deux objets stellaires.
/// On utilise 3 astéroides qu'on simule avec une nouvelle structure `TestAsteroid` pour vérifier les différents scénarios de collision:
/// 
/// - Un premier astéroïde (`asteroid1`) avec un autre astéroïde (`asteroid2`) qui est en collision avec lui.
/// - Un troisième astéroïde (`asteroid3`) placé à une distance trop grande pour entrer en collision avec les 2 autres.
/// 
/// Le test valide que la méthode `check_collision` renvoie `true` lorsqu'il y a collision et `false` lorsqu'il n'y en a pas.
/// 
/// # Scénarios vérifiés :
/// 
/// 1. Vérification que deux astéroïdes avec des positions proches et des rayons suffisants détectent une collision.
/// 2. Vérification que deux astéroïdes distants n'entrent pas en collision.
/// 
/// # Retour attendu :
/// 
/// Le test doit réussir si :
/// - `asteroid1` et `asteroid2` sont détectés comme en collision (car leur distance est inférieure à la somme de leurs rayons).
/// - `asteroid1` et `asteroid3` ne sont pas en collision (car leur distance est supérieure à la somme de leurs rayons).

#[cfg(test)]
mod tests {
    use super::*; // Accéder aux éléments du module `stellarobject` afin de vérifier la fonction 

    struct TestAsteroid {
        position: Vec2,
        radius: f32,
    }

    impl StellarObject for TestAsteroid {
        fn position(&self) -> Vec2 {
            self.position
        }

        fn update_position(&mut self) {
            // Ne fait rien pour le test
        }

        fn radius(&self) -> f32 {
            self.radius
        }
    }

    #[test]
    fn test_check_collision() {
        // Créer deux astéroïdes de test avec des positions et rayons différents

        // Création de l'astéroïde 1 avec une position à (0, 0) et un rayon de 1.0
        let asteroid1 = TestAsteroid {
            position: vec2(0.0, 0.0),
            radius: 1.0,
        };
        // Création de l'astéroïde 2 avec une position à (1, 1) et un rayon de 1.0
        let asteroid2 = TestAsteroid {
            position: vec2(1.0, 1.0),
            radius: 1.0,
        };
        // Création de l'astéroïde 3 avec une position à (5, 5) et un rayon de 1.0
        let asteroid3 = TestAsteroid {
            position: vec2(5.0, 5.0),
            radius: 1.0,
        };

        // Tester les collisions
        // Vérification que `asteroid1` et `asteroid2` sont en collision
        assert!(asteroid1.check_collision(&asteroid2)); // Collision attendue
        // Vérification que `asteroid1` et `asteroid3` ne sont pas en collision
        assert!(!asteroid1.check_collision(&asteroid3)); // Pas de collision attendue
    }
}
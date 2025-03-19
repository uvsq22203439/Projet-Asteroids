//! Module `asteroid.rs`
//!
//! Ce module définit la structure `Asteroid`, représentant un astéroïde dans le jeu.

use crate::stellarobject::StellarObject;
use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use std::f32::consts::PI;

/// Structure `Asteroid` représentant un astéroïde dans le jeu.
///
/// L'astéroïde possède une position, une vitesse, un rayon et une texture. Il se déplace à une vitesse
/// constante et, lorsqu'il dépasse les limites de l'écran, réapparaît du côté opposé.
pub struct Asteroid<'a> {
    radius: f32,            // Rayon de l'astéroïde
    position: Vec2,         // Position de l'astéroïde
    speed: Vec2,            // Vitesse et direction du déplacement de l'astéroïde
    texture: &'a Texture2D, // Texture de l'astéroïde
}

impl<'a> Asteroid<'a> {
    // Taille de l'astéroïde par défaut
    pub const ASTEROID_TAILLE: f32 = 60.0;

    /// Crée un nouvel astéroïde avec une position, une vitesse et un rayon aléatoires.
    ///
    /// # Arguments
    ///
    /// * `texture` - La texture à utiliser pour dessiner l'astéroïde.
    ///
    /// # Retourne
    ///
    /// Un nouvel astéroïde avec des propriétés générées aléatoirement.
    pub fn new(texture: &'a Texture2D) -> Self {
        Self {
            radius: Self::new_alea_radius(),
            position: Self::new_alea_pos(),
            speed: Self::new_alea_speed(),
            texture,
        }
    }

    /// Crée un vecteur d'astéroïdes, tous avec des propriétés aléatoires.
    ///
    /// # Arguments
    ///
    /// * `n` - Le nombre d'astéroïdes à générer.
    /// * `texture` - La texture à utiliser pour chaque astéroïde.
    ///
    /// # Retourne
    ///
    /// * `Vec<Asteroid<'a>>` - Un vecteur contenant `n` astéroïdes.
    pub fn generate_asteroid(n: usize, texture: &'a Texture2D) -> Vec<Asteroid<'a>> {
        (0..n).map(|_| Asteroid::new(texture)).collect()
    }

    /// Divise l'astéroïde en deux plus petits astéroïdes si sa taille le permet.
    ///
    /// # Retourne
    ///
    /// * `Vec<Asteroid<'a>>` - Un vecteur contenant les deux nouveaux astéroïdes créés, ou un vecteur vide si l'astéroïde
    ///     ne peut pas être divisé car trop petit.
    pub fn split(&self) -> Vec<Asteroid<'a>> {
        let current_radius = self.radius();

        if current_radius > Self::ASTEROID_TAILLE / 2.0 / 2.0 {
            let new_radius = current_radius / 2.0;

            let mut rng = thread_rng();
            let angle_offset: f32 = rng.gen_range(0.0..(2.0 * PI));

            let asteroid1 = Asteroid {
                radius: new_radius,
                position: self.position + vec2(new_radius / 2.0, 0.0),
                speed: Vec2::from_angle(angle_offset).normalize() * self.speed.length(),
                texture: self.texture,
            };

            let asteroid2 = Asteroid {
                radius: new_radius,
                position: self.position - vec2(new_radius / 2.0, 0.0),
                speed: Vec2::from_angle(angle_offset + PI / 2.0).normalize() * self.speed.length(),
                texture: self.texture,
            };

            vec![asteroid1, asteroid2]
        } else {
            vec![]
        }
    }

    /// Génère un rayon aléatoire pour un nouvel astéroïde.
    ///
    /// # Retourne
    ///
    /// * `f32` - Un flottant représentant un rayon aléatoire parmi trois tailles possibles.
    fn new_alea_radius() -> f32 {
        let mut rng = thread_rng();
        let num: f32 = rng.gen_range(1..=3) as f32;
        match num {
            1.0 => Self::ASTEROID_TAILLE / 2.0 / 2.0,
            2.0 => Self::ASTEROID_TAILLE / 2.0,
            _ => Self::ASTEROID_TAILLE,
        }
    }

    /// Génère une position aléatoire pour un nouvel astéroïde à la périphérie de l'écran.
    ///
    /// # Retourne
    ///
    /// * `Vec2` - Une position aléatoire sur l'écran, choisie parmi les bords représentée par un vecteur à 2 dimensions.
    fn new_alea_pos() -> Vec2 {
        let mut rng = thread_rng();

        let nearpos: f32 = rng.gen_range(Self::ASTEROID_TAILLE / 2.0..=Self::ASTEROID_TAILLE);
        let nearside = rng.gen_range(1..=4);
        let xpos: f32 = match nearside {
            2 => screen_width() - nearpos,
            4 => nearpos,
            _ => rng.gen_range(0.0..=screen_width()),
        };
        let ypos: f32 = match nearside {
            1 => nearpos,
            3 => screen_height() - nearpos,
            _ => rng.gen_range(0.0..=screen_height()),
        };
        vec2(xpos, ypos)
    }

    /// Génère une vitesse aléatoire pour un astéroïde, avec une direction aléatoire.
    ///
    /// # Retourne
    ///
    /// * `Vec2` - Une vitesse aléatoire, représentée par un vecteur à 2 dimensions.
    fn new_alea_speed() -> Vec2 {
        let mut rng = thread_rng();
        let angle: f32 = rng.gen_range(0.0..=(2.0 * PI));
        Vec2::from_angle(angle)
    }

    /// Dessine l'astéroïde à l'écran en utilisant sa texture et sa position.
    pub fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.position.x - self.radius,
            self.position.y - self.radius,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.radius * 2.0, self.radius * 2.0)),
                ..Default::default()
            },
        );
    }

    /// Contraint la position de l'astéroïde pour qu'il reste à l'intérieur des limites de l'écran.
    ///
    /// # Arguments
    ///
    /// * `pos` - La position actuelle de l'astéroïde.
    ///
    /// # Retourne
    ///
    /// * `Vec2` - Un vecteur à 2 dimensions représentant la position ajustée de l'astéroïde, qui sera toujours dans les limites de l'écran.
    fn bound_pos(mut pos: Vec2) -> Vec2 {
        pos.x = Self::bound_to(pos.x, screen_width());
        pos.y = Self::bound_to(pos.y, screen_height());
        pos
    }

    /// Ajuste une coordonnée pour qu'elle reste à l'intérieur des limites spécifiées.
    ///
    /// # Arguments
    ///
    /// * `coord` - La coordonnée à ajuster.
    /// * `max` - La valeur maximale (largeur ou hauteur de l'écran).
    ///
    /// # Retourne
    ///
    /// * `f32` - Un flottant qui définie la coordonnée ajustée.
    fn bound_to(coord: f32, max: f32) -> f32 {
        if coord < 0.0 {
            max - coord
        } else if coord > max {
            coord - max
        } else {
            coord
        }
    }
}

impl<'a> StellarObject for Asteroid<'a> {
    /// Retourne la position actuelle de l'astéroïde.
    ///
    /// # Retourne
    ///
    /// * `Vec2` - Les coordonnées actuelles de l'astéroide dans l'espace à 2 dimensions.
    fn position(&self) -> Vec2 {
        self.position
    }

    /// Met à jour la position de l'astéroïde en fonction de sa vitesse.
    ///
    /// La position est ajustée pour rester à l'intérieur des limites de l'écran.
    fn update_position(&mut self) {
        self.position += self.speed;
        self.position = Self::bound_pos(self.position);
    }

    /// Retourne le rayon de l'astéroïde.
    ///
    /// # Retourne
    ///
    /// * `f32` - Un flottant qui définie le rayon.
    fn radius(&self) -> f32 {
        self.radius
    }
}

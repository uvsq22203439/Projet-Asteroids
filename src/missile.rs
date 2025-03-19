//! Module `missile.rs`
//!
//! Ce module définit la structure `Missile`.

use crate::stellarobject::StellarObject;
use macroquad::prelude::*;

/// Structure `Missile` qui représente les missiles du jeu.
///
/// Le missile a une position, une vitesse, et un rayon. Il peut être déplacé à une vitesse constante
/// et peut être dessiné à l'écran. Il vérifie également si son positionnement sort des limites de l'écran.
pub struct Missile {
    position: Vec2, // Position actuelle du missile
    velocity: Vec2, // Vitesse du missile (direction et intensité)
    radius: f32,    // Rayon du missile (utilisé pour le dessin et la détection des collisions)
}

impl Missile {
    /// Crée un nouveau missile avec une position et un angle de lancement.
    ///
    /// # Arguments
    ///
    /// * `position` - La position initiale du missile.
    /// * `angle` - L'angle de lancement en radians pour déterminer la direction du missile.
    ///
    /// # Retourne
    ///
    /// Un nouveau missile avec une vitesse calculée à partir de l'angle et une taille fixe.
    pub fn new(position: Vec2, angle: f32) -> Self {
        Self {
            position,
            velocity: vec2(angle.cos(), angle.sin()) * 5.0, // Vitesse initiale basée sur l'angle
            radius: 3.0,                                    // Rayon du missile
        }
    }

    /// Dessine le missile à l'écran s'il n'est pas hors des limites.
    ///
    /// Le missile est représenté par un petit cercle rempli de couleur rouge.
    pub fn draw(&self) {
        if !self.is_out_of_bounds() {
            draw_circle(self.position.x, self.position.y, self.radius, RED); // Dessin du missile
        }
    }

    /// Vérifie si le missile est hors des limites de l'écran.
    ///
    /// # Retourne
    ///
    /// `true` si le missile dépasse les bords de l'écran, `false` sinon.
    pub fn is_out_of_bounds(&self) -> bool {
        self.position.x < 0.0
            || self.position.x > screen_width()
            || self.position.y < 0.0
            || self.position.y > screen_height() // Vérifie les limites de l'écran
    }
}

impl StellarObject for Missile {
    /// Retourne la position actuelle du missile.
    ///
    /// # Retourne
    ///
    /// * `Vec2` - Les coordonnées actuelles du missile dans l'espace à 2 dimensions.
    fn position(&self) -> Vec2 {
        self.position
    }

    /// Met à jour la position du missile en fonction de sa vitesse.
    ///
    /// Cette méthode déplace le missile dans la direction de sa vitesse à chaque mise à jour du jeu.
    fn update_position(&mut self) {
        self.position += self.velocity; // Déplace le missile
    }

    /// Retourne le rayon du missile.
    ///
    /// # Retourne
    ///
    /// * `f32` - Un flottant qui définie le rayon.
    fn radius(&self) -> f32 {
        self.radius
    }
}

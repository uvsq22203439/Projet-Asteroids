//! Module `Spaceship.rs`
//!
//! Ce module définit la structure `Spaceship`.

use crate::stellarobject::StellarObject;
use macroquad::prelude::*;
use std::f32::consts::PI;

/// Structure `Spaceship` qui représente le vaisseau spatial du jeu.
///
/// Le vaisseau a une position, une vitesse, une orientation, une intensité de pousée, un bouclier, des textures...  Il se déplace selon une certaine vitesse
/// et il vérifie son positionnement; ce qui signifie que si il dépasse les dimensions de la fenêtre, il
/// réapparaît du côté opposé.
pub struct Spaceship {
    position: Vec2,            // Position actuelle du vaisseau
    speed: Vec2,               // Vitesse actuelle du vaisseau
    angle: f32,                // Orientation du vaisseau en radians
    thrust: f32,               // Intensité de la poussée
    radius: f32,               // Rayon du vaisseau
    pub shield: u32,           // Points de bouclier restants
    pub invincible: bool,      // Indique si le vaisseau est temporairement invincible
    texture_ship: Texture2D,   // Texture pour représenter le vaisseau
    texture_shield: Texture2D, // Texture pour représenter le bouclier
}

impl Spaceship {
    /// Crée un nouveau vaisseau spatial à une position centrale avec des textures données.
    ///
    /// # Arguments
    /// * `texture_ship` - Texture pour le vaisseau.
    /// * `texture_shield` - Texture pour le bouclier.
    pub fn new(texture_ship: &Texture2D, texture_shield: &Texture2D) -> Self {
        Self {
            position: vec2(screen_width() / 2.0, screen_height() / 2.0),
            speed: vec2(0.0, 0.0), // Initialement à l'arrêt
            angle: -PI / 2.0,      // Orienté vers le haut
            thrust: 0.0,           // Pas de poussée par défaut
            radius: 30.0,
            shield: 2,         // Bouclier initial avec 2 points
            invincible: false, // Non invincible par défaut
            texture_ship: texture_ship.clone(),
            texture_shield: texture_shield.clone(),
        }
    }

    /// Retourne l'angle actuel du vaisseau.
    pub fn angle(&self) -> f32 {
        self.angle
    }

    /// Applique une poussée au vaisseau dans la direction de son angle.
    ///
    /// # Arguments
    /// * `amount` - Intensité de la poussée à appliquer.
    pub fn apply_thrust(&mut self, amount: f32) {
        self.thrust = amount;
    }

    /// Tourne le vaisseau dans le sens des aiguilles d'une montre.
    pub fn turn_right(&mut self) {
        self.angle += 0.015;
    }

    /// Tourne le vaisseau dans le sens inverse des aiguilles d'une montre.
    pub fn turn_left(&mut self) {
        self.angle -= 0.015;
    }

    /// Dessine le vaisseau et, si applicable, son bouclier à sa position actuelle.
    pub fn draw(&self) {
        // Dessin du vaisseau
        draw_texture_ex(
            &self.texture_ship,
            self.position.x - self.radius,
            self.position.y - self.radius,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.radius * 2.0, self.radius * 2.0)),
                rotation: self.angle + PI / 2.0,
                ..Default::default()
            },
        );

        // Dessin du bouclier s'il est actif
        if self.shield > 0 {
            draw_texture_ex(
                &self.texture_shield,
                self.position.x - self.radius * 1.5,
                self.position.y - self.radius * 1.5,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.radius * 3.0, self.radius * 3.0)),
                    rotation: self.angle + PI / 2.0,
                    ..Default::default()
                },
            );
        }

        // Cercle jaune représentant l'invincibilité du vaisseau
        if self.invincible {
            draw_circle_lines(
                self.position.x,
                self.position.y,
                self.radius * 1.3,
                3.0,
                YELLOW,
            );
        }
    }

    /// Contraint le vaisseau à rester dans les limites de la fenêtre
    fn bound_pos(mut pos: Vec2) -> Vec2 {
        pos.x = Self::bound_to(pos.x, screen_width());
        pos.y = Self::bound_to(pos.y, screen_height());
        pos
    }

    /// Si le vaisseau dépasse les coordonnées, il réapparaît de l'autre
    /// côté de la fenêtre
    /// # Argument
    /// * `max` - Coordonée maximum.
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

impl StellarObject for Spaceship {
    /// # Retourne
    /// `Vec2` - La position actuelle du vaisseau.
    fn position(&self) -> Vec2 {
        self.position
    }

    /// Met à jour la position du vaisseau en fonction de sa vitesse et de la poussée.
    fn update_position(&mut self) {
        self.speed += vec2(
            self.thrust * self.angle.cos(),
            self.thrust * self.angle.sin(),
        );

        if self.speed.length() > 1.0 {
            self.speed = self.speed.normalize();
        }

        self.speed *= 0.999; // Réduction de la vitesse pour simuler la friction

        self.position += self.speed;
        self.position = Self::bound_pos(self.position); // Gestion des bords de l'écran
    }

    /// # Retourne
    /// * `f32` - Le rayon du vaisseau.
    fn radius(&self) -> f32 {
        self.radius
    }
}

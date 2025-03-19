//! Module `stellarobject.rs`
//!
//! Ce module définit le trait `StellarObject`, utilisé pour représenter les objets stellaires (vaisseau saptial, astéroides...).

use macroquad::prelude::*;

/// Le trait `StellarObject` représente un objet stellaire dans le jeu.

/// Ce trait fournit des méthodes communes pour gérer la position, les collisions et le rayon
/// des objets stellaires.
pub trait StellarObject {
    /// Retourne la position de l'objet.

    /// # Retourne
    /// * `Vec2` - Les coordonnées actuelles de l'objet dans l'espace à 2 dimensions.
    fn position(&self) -> Vec2;

    /// Met à jour la position de l'objet en fonction de sa vitesse.
    ///
    /// Cette méthode déplace l'objet stellaire en fonction de sa vitesse et de sa direction.
    fn update_position(&mut self);

    /// Vérifie si cet objet entre en collision avec un autre.
    ///
    /// # Arguments
    /// * `other` - Une référence à un autre objet implémentant `StellarObject`.
    ///
    /// # Retourne
    /// * un booléen - `true` si les deux objets sont en collision, `false` sinon.
    ///
    /// La collision est détectée en calculant la distance entre les deux objets
    /// et en la comparant à la somme de leurs rayons.
    fn check_collision(&self, other: &dyn StellarObject) -> bool {
        let distance = self.position().distance(other.position());
        let combined_radius = self.radius() + other.radius();
        distance < combined_radius
    }

    /// Retourne le rayon de l'objet.
    ///
    /// # Retourne
    /// * `f32` - Le rayon de l'objet utilisé pour les calculs de collision.
    fn radius(&self) -> f32;
}

extern crate nalgebra_glm;
use nalgebra_glm::Vec3;

pub struct Camera {
    pub eye: Vec3,    // Posición de la cámara en el espacio del mundo
    pub center: Vec3, // Punto que la cámara está mirando
    pub up: Vec3,     // Vector que representa "arriba" para la cámara
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Self { eye, center, up }
    }

    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();

        let rotated = vector.x * right + vector.y * up - vector.z * forward;
        rotated.normalize()
    }

    // Método para rotar la cámara utilizando yaw y pitch
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        // Calcular el vector desde el centro hasta el ojo (radio) y medir la distancia
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        // Calcular el yaw actual (rotación alrededor del eje Y)
        let current_yaw = radius_vector.z.atan2(radius_vector.x);

        // Calcular el pitch actual (rotación alrededor del eje X)
        let radius_xz =
            (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        // Aplicar las rotaciones delta
        // Mantener yaw en el rango [0, 2π] para consistencia
        let new_yaw = (current_yaw + delta_yaw) % (2.0 * std::f32::consts::PI);

        // Clamp el pitch para prevenir gimbal lock
        let new_pitch = (current_pitch + delta_pitch).clamp(
            -std::f32::consts::FRAC_PI_2 + 0.1,
            std::f32::consts::FRAC_PI_2 - 0.1,
        );

        // Calcular la nueva posición del ojo
        // Usamos coordenadas esféricas para la conversión a coordenadas cartesianas
        let new_eye = self.center
            + Vec3::new(
                radius * new_yaw.cos() * new_pitch.cos(),
                -radius * new_pitch.sin(),
                radius * new_yaw.sin() * new_pitch.cos(),
            );

        // Actualizar la posición de la cámara
        self.eye = new_eye;
    }

    pub fn zoom(&mut self, factor: f32) {
        // Calcular el vector de la dirección desde la cámara hacia el centro
        let direction = (self.center - self.eye).normalize();

        // Ajustar la posición de la cámara en la dirección del centro
        self.eye += direction * factor;
    }
}

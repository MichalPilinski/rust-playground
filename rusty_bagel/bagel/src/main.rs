use std::{thread, time::Duration};

struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn subtract(&self, subtractor: &Vec3) -> Vec3 {
       Vec3 { 
        x: self.x - subtractor.x, 
        y: self.y - subtractor.y, 
        z: self.z - subtractor.z }
    }

    fn modulo(&self, mod_val: f32) -> Vec3 {
        Vec3 {
            x: self.x % mod_val,
            y: self.y % mod_val,
            z: self.z % mod_val,
        }
    }
}

fn main() {
    const SCREEN_WIDTH: usize = 75;
    const SCREEN_HEIGHT: usize = 75;

    let mut pixels = vec![0.0; SCREEN_WIDTH * SCREEN_HEIGHT];
    let mut light_pos: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};

    let epsilon = 0.01;
    let max_dist = 200.0;
    let iter_limit = 100;

    // Loop over screen pseudo-pixels
    let mut tick: f32 = 0.0;
    loop {
        // Clears console each frame
        print!("{esc}c", esc = 27 as char);

        // Animations
        tick += 0.1;
        light_pos.x = 25.0 + 15.0 * tick.cos();
        light_pos.y = 25.0 + 15.0 * tick.sin();
        light_pos.z = -15.0;

        // Render loop
        for y_iter in 1..SCREEN_HEIGHT {
            for x_iter in 1..SCREEN_WIDTH {
                let mut dist = max_dist-epsilon;
                let mut ray_pos: Vec3 = Vec3 {
                    x: (x_iter as f32) + 0.5,
                    y: (y_iter as f32) + 0.5,
                    z: 0.0
                };

                let mut iter_number = 0;
                while dist > epsilon && dist < max_dist {
                    iter_number += 1;
                    if iter_number > iter_limit {
                        break;
                    }
                    // calculate distance estimator to object
                    dist = get_distance_estimation(&ray_pos);
                    ray_pos.z += dist;
                }
    
                if dist < epsilon {
                    let real_sensor_pos: Vec3 = Vec3 {
                        x: x_iter as f32,
                        y: y_iter as f32,
                        z: 0.0
                    };

                    let pixel_intensity = get_diffused_intensity(&real_sensor_pos, &ray_pos, &light_pos);
                    pixels[(y_iter * SCREEN_WIDTH) + x_iter] = pixel_intensity;
                } else {
                    pixels[(y_iter * SCREEN_WIDTH) + x_iter] = 0.0;
                }
            }
        }

        // Draw and scaling loop
        let char_size_ratio: f32 = 0.5;
        let mut canvas = String::from("");
        for y_iter in 1..SCREEN_HEIGHT {
            let mut position: f32 = 0.0;
            while (position as usize) < SCREEN_WIDTH {
                let intencity = pixels[(y_iter * SCREEN_WIDTH) + (position as usize)];
                let char_intensity = get_symbol(&intencity);

                canvas.push(char_intensity);
                position += char_size_ratio;
            } 

            canvas.push('\n');
        }
        print!("{}", canvas);

        println!("Light pos: {}, {}, {}", light_pos.x, light_pos.y, light_pos.z);
    }
}

fn get_distance_estimation(ray_pos: &Vec3) -> f32 {
    const SPHERE_POS: Vec3 = Vec3 {x: 5.0, y: 5.0, z: 5.0};
    const SPHERE_R: f32 = 5.0;

    // const SPHERE2_POS: Vec3 = Vec3 {x: 35.0, y: 25.0, z: 50.0};
    // const SPHERE2_R: f32 = 10.0;

    // Distance estimator of a sphere
    ray_pos.modulo(15.0).subtract(&SPHERE_POS).length() - SPHERE_R
    //let dist2 = ray_pos.subtract(&SPHERE2_POS).length() - SPHERE2_R;

    //smooth(dist1, dist2, 10.0)
}

fn smooth(dist1: f32, dist2: f32, k: f32) -> f32{
    let h: f32 = (k-(dist1 - dist2).abs()).max(0.0) / k;
    
    dist1.min(dist2) - h.powf(3.0) * k / 6.0
}

fn get_diffused_intensity(real_sensor_pos: &Vec3, intersect_pos: &Vec3, light_pos: &Vec3) -> f32 {
    const LIGHT_INTENS: f32 = 30.0;
    
    let intersect_sensor_vec = real_sensor_pos.subtract(intersect_pos);
    let intersect_light_vec = light_pos.subtract(&intersect_pos);

    let dot_product = 
        (intersect_light_vec.x * intersect_sensor_vec.x) +
        (intersect_light_vec.y * intersect_sensor_vec.y) +
        (intersect_light_vec.z * intersect_sensor_vec.z);

    let surf_intens = LIGHT_INTENS / intersect_light_vec.length();
    (surf_intens * dot_product) / (intersect_light_vec.length() * intersect_sensor_vec.length()) 
}

fn get_symbol(intensity: &f32) -> char {
    let luminecense_table: Vec<char> = " .,-~:;=!*#$@".chars().collect();

    let char_index = (13.0 * intensity - 0.01).floor() as usize;
    luminecense_table[char_index.min(12)]
}
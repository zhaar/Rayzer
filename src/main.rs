use std::marker::Copy;
use std::num;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::Add;

fn main() {
  let sphere = Shape::Sphere { center: Vec3D {x: 2.0, y: 2.0, z: 1.0}, radius: 1.0};
  let scene = Scene { elements: vec![sphere] };
  let ray = Ray { origin: Vec3D {x: 0.0, y: 0.0, z: 1.0},
                  direction: unit(Vec3D {x: 1.0, y: 1.0, z: 0.0})
                };
  let p = trace_ray(scene, ray);
  println!("intersect: {}", p);
  
}

fn trace_ray(scene: Scene, ray: Ray) -> i32 {
  let intersections = scene.elements.into_iter().filter(|ref e| ray.intersects(&e)).collect::<Vec<_>>();
  if intersections.len() > 0 {
    1
  } else {
    0
  }
}

#[derive(Copy, Clone)]
struct Ray {
  origin: Vec3D,
  direction: Vec3D
}

impl Ray {
  fn intersects(self, shape: &Shape) -> bool {
    match *shape {
      Shape::Sphere { center: c, radius: r } =>{
        let dist = self.origin - c;
        let a = self.direction * self.direction;
        let b = 2.0 * (self.direction * dist);
        let c = dist * dist - r * r;
        if let QuadraticSolution::None = solve_quadratic(a, b, c) {
          false
        } else {
          true 
        }
      }
      Shape::Plane { position: p, normal:n } => {
        false
      }
    }
  }
}

#[derive(Copy, Clone)]
struct Vec3D {
  x: f64,
  y: f64,
  z: f64
}

impl Vec3D {
  fn length(self) -> f64 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }
}

fn unit(vector: Vec3D) -> Vec3D {
  vector / vector.length()
}

impl Div<f64> for Vec3D {
  type Output = Vec3D;

  fn div(self, other: f64) -> Vec3D {
    Vec3D {x: self.x / other, y: self.y / other, z: self.z / other }
  }
}

impl Add for Vec3D {
  type Output = Vec3D;

  fn add(self, other: Vec3D) -> Vec3D {
    Vec3D {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
  }
}

impl Sub for Vec3D {
  type Output = Self;

  fn sub(self, other: Vec3D) -> Vec3D {
    Vec3D {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
  }
}

impl Mul for Vec3D {
  type Output = f64;

  fn mul(self, other: Vec3D) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
}

struct Camera {
  position: Vec3D
}

enum Shape {
  Sphere { center: Vec3D, radius: f64 },
  Plane { position: Vec3D, normal: Vec3D }
}

struct Light {

}
struct Scene {
  elements: Vec<Shape>,
}

enum QuadraticSolution {
  None,
  One(f64),
  Two(f64, f64)
}

// ax^2 + bx + c = 0
fn solve_quadratic(a: f64, b: f64, c: f64) -> QuadraticSolution {
  let delta = b * b - 4.0* a * c;
  if delta < 0.0 {
    QuadraticSolution::None
  } else if delta == 0.0 {
    QuadraticSolution::One(-b / (2.0* a))
  } else {
    QuadraticSolution::Two((-b + delta)/ (2.0 * a), (-b - delta) / (2.0 * a))
  }
}

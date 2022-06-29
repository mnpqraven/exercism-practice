// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
      // you can hardcode like Self { name, age, weight: 60 }
      Self { name, age, weight}
    }

    // no return nor semicolon at the end
    // attention to the -> lambda
    pub fn name(&self) -> &str {
      self.name.as_str()
    }

    pub fn age(&self) -> u32 {
      self.age
    }

    pub fn weight(&self) -> f32 {
      self.weight
    }

    // can assign to self cause of &mut
    pub fn set_age(&mut self, new_age: u32) {
      self.age = new_age;
    }

    pub fn set_weight(&mut self, new_weight: f32) {
      self.weight = new_weight;
    }
}

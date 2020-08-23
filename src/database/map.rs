#[derive(Debug)]
pub struct Map {
  index: Vec<Option<u32>>,
}

impl Map {
  pub fn new() -> Map {
    Map { index: Vec::new() }
  }

  pub fn set(&mut self, id: u32, row_number: u32) {
    if id as usize > self.index.len() {
      self.index.resize(id as usize, None);
    }

    if let Some(_) = self.index.get_mut(id as usize) {
      return;
    }

    self.index.insert(id as usize, Some(row_number));
    println!("{:?}", self);
  }

  pub fn get(&self, id: u32) -> Option<u32> {
    if let Some(option) = self.index.get(id as usize) {
      if let Some(number) = option {
        return Some(*number);
      }
    }

    None
  }
}

pub struct Map {
  index: Vec<Option<[u32; 1024]>>,
}

impl Map {
  pub fn new() -> Map {
    Map { index: Vec::new() }
  }

  pub fn set(&mut self, id: u32, row_number: u32) {
    let page_number = id / 1024;
    let index_number = id % 1024;

    while page_number as usize > self.index.len() {
      self.index.push(None);
    }

    if let Some(option) = self.index.get_mut(page_number as usize) {
      if let Some(slice) = option {
        slice[index_number as usize] = row_number;
        return;
      }
    }

    let mut new_slice = [0; 1024];
    new_slice[index_number as usize] = row_number;
    self.index.insert(page_number as usize, Some(new_slice));
  }

  pub fn get(&self, id: u32) -> Option<u32> {
    let page_number = id / 1024;
    let index_number = id % 1024;

    if let Some(option) = self.index.get(page_number as usize) {
      if let Some(slice) = option {
        return match slice[index_number as usize] {
          0 => None,
          x => Some(x),
        };
      }
    }

    None
  }
}

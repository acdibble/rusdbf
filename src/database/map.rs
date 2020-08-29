#[derive(Debug)]
pub struct Map {
    index: Vec<Option<Vec<u32>>>,
}

fn get_page_number(id: u32) -> usize {
    id as usize / 1024
}

fn get_index(id: u32) -> usize {
    id as usize % 1024
}

impl Map {
    pub fn new() -> Map {
        Map { index: Vec::new() }
    }

    pub fn set(&mut self, id: u32, row_number: u32) {
        let page_number = get_page_number(id);
        if page_number >= self.index.len() {
            self.index.resize(page_number + 1, None);
        }

        debug_assert!(self.index.len() > page_number);
        let page = self
            .index
            .get_mut(page_number)
            .expect("failed to retrieve option from index")
            .get_or_insert_with(|| vec![0; 1024]);

        let index = get_index(id);
        debug_assert!(page.len() > index);
        page[index] = row_number;
    }

    pub fn get(&self, id: u32) -> u32 {
        let page_number = get_page_number(id);
        match &self.index.get(page_number) {
            None | Some(None) => 0,
            Some(Some(vector)) => {
                let index = get_index(id);
                vector[index]
            }
        }
    }
}

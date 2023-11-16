const BIGGEST_NUMBER: isize = isize::MAX;

#[derive(Debug)]
pub struct UpdateableHeapElem<Payload: Default + Clone> {
    pub idx: usize,
    pub ordering_key: isize,
    pub payload: Payload,
}

impl<Payload: Default + Clone> UpdateableHeapElem<Payload> {
    pub fn new(idx: usize, ordering_key: isize, payload: Payload) -> Self {
        Self {
            idx,
            ordering_key,
            payload,
        }
    }
}

#[derive(Debug)]
pub struct UpdateableHeap<Payload: Default + Clone> {
    data: Vec<UpdateableHeapElem<Payload>>,
    idx_map: Vec<usize>,
    size: usize,
    capacity: usize,
}

impl<Payload: Default + Clone> UpdateableHeap<Payload> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            idx_map: vec![0; capacity],
            size: 0,
            capacity,
        }
    }

    pub fn fill(&mut self) {
        let capacity = self.capacity;
        for i in 0..capacity {
            self.insert(UpdateableHeapElem::new(
                i,
                BIGGEST_NUMBER,
                Payload::default(),
            ));
        }
    }

    pub fn insert(&mut self, elem: UpdateableHeapElem<Payload>) -> usize {
        let idx = elem.idx;
        self.data.push(elem);
        self.size += 1;

        if self.size == 1 {
            return 0;
        }

        self.idx_map[idx] = self.size - 1;

        self.up_fn(self.size - 1)
    }

    pub fn decrease_by_idx(&mut self, idx: usize, new_val: isize, payload: Payload) -> usize {
        let data_idx = self.idx_map[idx];
        if data_idx >= self.capacity {
            return idx;
        }

        let new_idx = if self.data[data_idx].ordering_key < new_val {
            idx
        } else {
            self.data[data_idx].ordering_key = new_val;
            self.data[data_idx].payload = payload;
            self.up_fn(data_idx)
        };
        new_idx
    }

    pub fn get_and_remove_min(&mut self) -> UpdateableHeapElem<Payload> {
        self.data.swap(0, self.size - 1);
        let result = self.data.pop().unwrap();
        self.size -= 1;

        if self.size > 0 {
            self.idx_map[result.idx] = self.capacity;
            let new_head = self.data[0].idx;
            self.idx_map[new_head] = 0;
            self.down_fn(0);
        }

        result
    }

    fn down_fn(&mut self, mut new_elem_idx: usize) -> usize {
        loop {
            let children = self.find_children(new_elem_idx);

            if children.len() == 0 {
                break;
            }

            let min_children_id = if children.len() == 1 {
                children[0]
            } else {
                if self.data[children[0]].ordering_key < self.data[children[1]].ordering_key {
                    children[0]
                } else {
                    children[1]
                }
            };

            if self.data[min_children_id].ordering_key >= self.data[new_elem_idx].ordering_key {
                break;
            }

            self.idx_map[self.data[min_children_id].idx] = new_elem_idx;
            self.idx_map[self.data[new_elem_idx].idx] = min_children_id;

            self.data.swap(new_elem_idx, min_children_id);

            new_elem_idx = min_children_id;
        }

        new_elem_idx
    }

    fn up_fn(&mut self, mut new_elem_idx: usize) -> usize {
        loop {
            if new_elem_idx == 0 {
                return 0;
            }
            let parent_idx = Self::find_parent(new_elem_idx);
            if self.data[parent_idx].ordering_key <= self.data[new_elem_idx].ordering_key {
                break;
            }
            self.idx_map[self.data[parent_idx].idx] = new_elem_idx;
            self.idx_map[self.data[new_elem_idx].idx] = parent_idx;

            self.data.swap(new_elem_idx, parent_idx);

            new_elem_idx = parent_idx;
        }

        new_elem_idx
    }

    fn find_parent(idx: usize) -> usize {
        (idx + 1) / 2 - 1
    }

    fn find_children(&self, idx: usize) -> Vec<usize> {
        let mut result = Vec::with_capacity(2);
        let first = (idx + 1) * 2 - 1;
        for i in 0..2 {
            if self.size > first + i {
                result.push(first + i)
            }
        }
        result
    }
}

use std::marker::PhantomData;


pub struct DrawPointer<T> { 
	idx: usize,
	_marker: PhantomData<T>,
}

pub struct DrawBuffer<T> {
	prefab_items: Vec<T>,
	render_queue: Vec<usize>,
}

impl<T> DrawBuffer<T> {
	pub fn new(capacity: usize) -> Self {
		DrawBuffer {
		    render_queue: Vec::with_capacity(capacity),
			prefab_items: Vec::new(),
		}
	}
	
	pub fn define(&mut self, draw_item: T) -> DrawPointer<T> {
		let pointer = DrawPointer { 
			idx: self.prefab_items.len(),
			_marker: PhantomData,
		};
		self.prefab_items.push(draw_item);
		pointer
	}
		
	pub fn stage(&mut self, draw_ptr: &DrawPointer<T>) {
		self.render_queue.push(draw_ptr.idx);
	}

	pub fn edit(&mut self, draw_ptr: &DrawPointer<T>) -> &mut T {
		&mut self.prefab_items[draw_ptr.idx]
	}

	pub fn read(&mut self, draw_ptr: &DrawPointer<T>) -> &T {
		&self.prefab_items[draw_ptr.idx]
	}

	pub fn by_index(&self, queue_idx: &usize) -> &T {
		&self.prefab_items[*queue_idx]
	}

	pub fn queue(&self) -> &Vec<usize> {
		&self.render_queue
	}

	pub fn clear_queue(&mut self) {
		self.render_queue.clear();
	}

	pub fn item_list(&self) -> &Vec<T> {
		&self.prefab_items
	}

	pub fn mut_item_list(&mut self) -> &mut Vec<T> {
		&mut self.prefab_items
	}
}

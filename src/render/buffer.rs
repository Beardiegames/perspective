use std::marker::PhantomData;
use super::*;


pub struct DrawPointer<T> { 
	idx: usize,
	_marker: PhantomData<T>,
}


pub struct DrawBuffer<T: Drawable> {
	pub base_material: Option<Material>,
	pub render_queue: Vec<usize>,
	pub prefab_items: Vec<T>,
}

impl<T> DrawBuffer<T> 
	where T: Drawable
{
	pub fn new(capacity: usize) -> Self {
		DrawBuffer {
			base_material: None,
		    render_queue: Vec::with_capacity(capacity),
			prefab_items: Vec::new(),
		}
	}
	
	pub fn define(&mut self, draw_item: T) -> DrawPointer<T> {
		let pointer = DrawPointer::<T> { 
			idx: self.prefab_items.len(),
			_marker: PhantomData,
		};
		self.prefab_items.push(draw_item);
		pointer
	}
		
	pub fn stage(&mut self, draw_ptr: &DrawPointer<T>) {
		self.render_queue.push(draw_ptr.idx);
	}

	pub fn stage_by_index(&mut self, idx: usize) {
		self.render_queue.push(idx);
	}
	

	pub fn edit_prefab(&mut self, draw_ptr: &DrawPointer<T>) -> &mut T {
		&mut self.prefab_items[draw_ptr.idx]
	}

	pub fn read_prefab(&mut self, draw_ptr: &DrawPointer<T>) -> &T {
		&self.prefab_items[draw_ptr.idx]
	}

	pub fn list_prefabs(&self) -> &Vec<T> {
		&self.prefab_items
	}

	pub fn list_prefabs_mut(&mut self) -> &mut Vec<T> {
		&mut self.prefab_items
	}

	pub fn read_prefab_at(&self, queue_idx: &usize) -> Option<&T> {
		match *queue_idx < self.prefab_items.len() {
			true => Some(&self.prefab_items[*queue_idx]),
			false => None,
		}
	}

	pub fn edit_prefab_at(&mut self, queue_idx: &usize) -> Option<&mut T> {
			match *queue_idx < self.prefab_items.len() {
				true => Some(&mut self.prefab_items[*queue_idx]),
				false => None,
			}
		}

	pub fn number_of_prefabs(&self) -> usize {
		self.prefab_items.len()
	}

	pub fn draw_queue(&self) {
		for i in &self.render_queue {
			self.prefab_items[*i].draw();
		}
	}
}

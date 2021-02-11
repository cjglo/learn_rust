// impl a vec from scratch
use std::ptr::NonNull;
use std::alloc;

pub struct MyVec<T> {

	ptr: NonNull<T>, // ptr to beginning of list
	len: usize,
	capacity: usize,

}


impl <T> MyVec<T> {

	pub fn new() -> Self {
		Self {
			ptr: NonNull::dangling(),
			len: 0,
			capacity: 0, 
		}

	}

	pub fn push(&mut self) {
		if self.capacity == 0 {
			// let layout = alloc::Layout::array<T>(4).expect("Could not allocate!");
			// Saefty: the layout is hardcoded to be non-zero (4 times size of T)
			// let ptr = unsafe { alloc::alloc(layout) } as *mut T;
		}
		// todo!() /// left off at 38 minutes into demo

	}




	pub fn capacity(&self) -> usize {
		self.capacity
	}

	pub fn len(&self) -> usize {
		self.len
	}

}










#[cfg(test)]
mod tests {
	use super::*;


    #[test]
    fn init_test() {

        let vec: MyVec<usize> = MyVec::new(); // could also remove <usize> and put MyVec::<usize>::new()

        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.len(), 0);

    }


}

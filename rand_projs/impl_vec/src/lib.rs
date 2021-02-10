// impl a vec from scratch
use std::ptr::NonNull;

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

        let vec: MyVec<usize> = MyVec::new();

        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.len(), 0);

    }


}

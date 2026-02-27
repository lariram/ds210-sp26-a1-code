use std::{fmt::{Display, Formatter}, ptr::{self, null_mut}};

use malloc::MALLOC;

pub struct FastVec<T> {
    ptr_to_data: *mut T,
    len: usize,
    capacity: usize,
}
impl<T> FastVec<T> {
    // Creating a new FastVec that is either empty or has capacity for some future elements.
    pub fn new() -> FastVec<T> {
        return FastVec::with_capacity(1);
    }
    pub fn with_capacity(capacity: usize) -> FastVec<T> {
        return FastVec {
            ptr_to_data: MALLOC.malloc(size_of::<T>() * capacity) as *mut T,
            len: 0,
            capacity: capacity,
        };
    }

    // Retrieve the FastVec's length and capacity
    pub fn len(&self) -> usize {
        return self.len;
    }
    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    // Transforms an instance of SlowVec to a regular vector.
    pub fn into_vec(mut self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len);
        for i in 0..self.len {
            unsafe {
                let ptr = self.ptr_to_data.add(i);
                let element = ptr::read(ptr);
                v.push(element);
            }
        }
        MALLOC.free(self.ptr_to_data as *mut u8);
        self.ptr_to_data = null_mut();
        self.len = 0;
        self.capacity = 0;
        return v;
    }

    // Transforms a vector to a SlowVec.
    pub fn from_vec(vec: Vec<T>) -> FastVec<T> {
        let mut fast_vec: FastVec<T> = FastVec::with_capacity(vec.len());
        for element in vec {
            unsafe {
                let ptr = fast_vec.ptr_to_data.add(fast_vec.len);
                ptr::write(ptr, element);
            }
            fast_vec.len = fast_vec.len + 1;
        }
        return fast_vec;
    }

    // Student 1 and Student 2 should implement this together
    // Use the project handout as a guide for this part!
    pub fn get(&self, i: usize) -> &T {
        // telling Rust that I know the codes are not so safe.
        unsafe {
        //determine if index i is larger than the maximum range of the vector
        if i >= self.len {
            panic!("FastVec: get out of bounds"); //show the error message if it does
            } else {
                // if it is in the range of the vector
                let ptr_i= self.ptr_to_data.add(i); // Change the position of the pointer to the index
                let val: &T = &*ptr_i; //read the index value without changing the original value
                return val //return the value being read
            }
        }   
    }

    // Student 2 should implement this.
    pub fn push(&mut self, t: T) {
        if self.len == self.capacity {
            unsafe {
            let ptr_to_data: *mut T = MALLOC.malloc(size_of::<T>() * self.len * 2) as *mut T; // allocate new memory of twice the size
            for i in 0..self.len { // loop over the original array
                let val = std::ptr::read(self.ptr_to_data.add(i)); 
                std::ptr::write(ptr_to_data.add(i), val); 
                // move over all the elements from the previous pointer to the new pointer
                }
            MALLOC.free(self.ptr_to_data as *mut u8); // free the old pointer memory
            self.capacity *= 2; // update capacity
            self.ptr_to_data = ptr_to_data; // update pointer
            std::ptr::write(self.ptr_to_data.add(self.len), t); // write the value in the new array 
            self.len += 1 // update length
            }

        } else {
            unsafe {
            std::ptr::write(self.ptr_to_data.add(self.len), t); // write the value in the new array
            self.len += 1 // update length
            }
        }
    }

    // Student 1 should implement this.
    pub fn remove(&mut self, i: usize) {
         // telling Rust that I know the codes are not so safe.
        unsafe {
        //determine if index i is larger than the maximum range of the vector
        if i >= self.len {
            panic!("FastVec: remove out of bounds"); //show the error message if it does
            } else {
                // if it is in the range of the vector
                // move the pointer to the index I want to remove.
                let ptr_i: *mut T = self.ptr_to_data.add(i);
                //read the value of the index and remove the original value
                let val: T = ptr::read(self.ptr_to_data);
                //if the index i is at the maximum end of the vector:
                if i != self.len -1 {
                    // loop through the indexes after index i till the end:
                    for j in (i+1)..self.len {
                        //move the pointer to the new position after index i at j
                        let ptr_j: *mut T = self.ptr_to_data.add(j);
                        // read the value of the index and remove the original value
                        let val: T = ptr::read(ptr_j);
                        //move the pointer to the one position ahead of j
                        let ptr_j_1: *mut T = self.ptr_to_data.add(j-1);
                        // write down the value for the new position
                        ptr::write(ptr_j_1, val)

                    }
                }
                // reduce the length of the vector by one
                self.len = self.len -1;
            }
        }  
    }

    // This appears correct but with further testing, you will notice it has a bug!
    // Student 1 and 2 should attempt to find and fix this bug.
    // Hint: check out case 2 in memory.rs, which you can run using
    //       cargo run --bin memory
    pub fn clear(&mut self) {
        MALLOC.free(self.ptr_to_data as *mut u8);
        self.ptr_to_data = null_mut();
        self.len = 0;
        self.capacity = 0;
    }
}

// Destructor should clear the fast_vec to avoid leaking memory.
impl<T> Drop for FastVec<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

// This allows printing FastVecs with println!.
impl<T: Display> Display for FastVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FastVec[")?;
        if self.len > 0 {
            for i in 0..self.len()-1 {
                write!(f, "{}, ", self.get(i))?;
            }
            write!(f, "{}", self.get(self.len - 1))?;
        }
        return write!(f, "]");
    }
}
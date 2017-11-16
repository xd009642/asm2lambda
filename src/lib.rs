extern crate libc;

use std::mem;

// Replace jumps with return true/false depending on condition
pub struct Functor {
    program_space: *mut libc::c_void,
    size: usize,
}

impl Functor {
    ///
    /// TODO no error checking on sysconf, memalign, mprotect etc
    ///
    unsafe fn new(size: usize) -> Functor {
        let mut result = Functor {
            program_space: mem::uninitialized(),
            size: size,
        };
        
        libc::posix_memalign(&mut result.program_space, 
                             libc::sysconf(libc::_SC_PAGESIZE) as usize, size);
        libc::mprotect(result.program_space, size, libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE);
        result
    }

    fn set_memory(&self, asm: &[u8]) {
        assert!(asm.len() <= self.size);
        unsafe {
            libc::memcpy(self.program_space, asm.as_ptr() as *const libc::c_void, asm.len());
        }
    }

    fn execute(&self) -> bool {
        false 
    }
}

pub fn compute_lambda(asm: &[u8]) {
    unimplemented!();
}



#[cfg(test)]
mod tests {

    fn simple_test() {
        // `testb $1, %al`
        let asm = [0x84u8, 0x04u8, 0x25u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8];
    }

}

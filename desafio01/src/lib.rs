// src/lib.rs


//unsafe
pub unsafe fn multiply_array(ptr: *const i32, len: usize) -> i32 {
    let mut product = 1;
    //valor do loop inicial alterado
    for i in 0..len {
        // Bloco unsafe para operações de offset e de referências
        unsafe {
            product *= *ptr.offset(i as isize);
        }
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let arr = [2, 3, 4];
        let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) };
        assert_eq!(product, 24);
    }
}

// Suprime warning de função não utilizada
#[allow(dead_code)]
fn main() {
    println!("Hello, world!");
}
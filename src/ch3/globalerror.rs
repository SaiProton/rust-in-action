use rand::random;

static mut ERROR: isize = 0;

struct File;

#[allow(unused_variables)]
fn read(f: &File, save_to: &mut [u8]) -> usize {
    if random() && random() && random() {
        unsafe {
            ERROR = 1;
        }
    }
    0
}

#[allow(unused_mut)]
pub fn main() {
    let mut f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);

    unsafe {
        assert!(ERROR == 0, "An error has occured!");
    }
}

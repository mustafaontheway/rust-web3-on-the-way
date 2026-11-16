fn main() {

}

/*
The Rust standard library does not include an `update` method for `u16`. Let’s create one ourselves!
*/

trait Vectors<T> {

    fn size(&self) -> usize;
    fn max(&self) -> T;
    fn min(&self) -> T;
    fn update(&mut self) -> T;
}

impl Vectors<u16> for u16 {
  
    fn size(&self) -> usize {
        
        todo!()
    }

    fn max(&self) -> u16 {
        
        todo!()
    }

    fn min(&self) -> u16 {
        
        todo!()
    }

    fn update(&mut self) -> u16 {
        
        todo!()
    }
}

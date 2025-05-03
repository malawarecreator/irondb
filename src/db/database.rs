use std::{fmt::Debug, vec};
use super::dblock::Dblock;

#[derive(Debug,PartialEq, Eq, Clone)]
pub struct Db<A, B> {
    pub datablocks: Box<Vec<Dblock<A, B>>>,
    pub id: &'static str,
}

impl<A: PartialEq + Debug + Clone, B: PartialEq + Debug+ Clone> Db<A, B> {
    pub fn new(id: &'static str) -> Self {
        Self {datablocks: Box::new(vec![]), id}
    }
    
    pub fn save(&mut self, block: Dblock<A, B>) {
        self.datablocks.push(block);
    }
    
    pub fn getbp(&self, key: A) -> i32 {
        for i in 0..self.datablocks.len() {
            if self.datablocks.get(i).unwrap().key == key {
                println!("{:?}", self.datablocks.get(i).unwrap());
                return 0;
            }
        }
        return 1;


    } 

    pub fn getb(&self, key: A) -> Result<Dblock<A, B>, &'static str>{
        for i in 0..self.datablocks.len() {
            if self.datablocks.get(i).unwrap().key == key {
                return Ok(Dblock::new(self.datablocks.get(i).unwrap().key.clone(), self.datablocks.get(i).unwrap().data.clone()));
            }
        }

        return Err("no such block");
    }

    pub fn remove(&mut self, key: A) -> i32 {
        for i in 0..self.datablocks.len() {
            if self.datablocks.get(i).unwrap().key == key {
                self.datablocks.remove(i);
                return 0;
            }
        }
        return 1;
    }

}



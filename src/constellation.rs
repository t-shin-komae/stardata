use crate::star::Star;
use crate::Name;

#[derive(PartialEq,Debug)]
pub struct Constellation {
    pub stars: Vec<Star>,
    pub graph: Vec<(usize,usize)>,
    name: Name,
}

impl Constellation {
    /// Create a empty constellation.
    pub fn new(name:Name) -> Self{
        Self{
            stars:Vec::new(),graph: Vec::new(),name:name,
        }
    }

    pub fn get_name(&self) -> Name {
        self.name.clone()
    }

    pub fn add(&mut self,start:&Star,end:&Star){
        let start_index = 
        match self.stars.iter().enumerate().find(|star| star.1 == start){
            None =>{
                let len = self.stars.len();
                self.stars.push(start.clone());
                len
                }
            Some(found) => {
                found.0
            }
        };

        let end_index = 
        match self.stars.iter().enumerate().find(|star| star.1 == end){
            None =>{
                let len = self.stars.len();
                self.stars.push(end.clone());
                len
                }
            Some(found) => {
                found.0
            }
        };
        self.graph.push((start_index,end_index));
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::star::*;
    #[test]
    fn test_constellation() {
        let betelgeuse = Star::new(27989, Equatorial::new(5, 55, 10.29, true, 7, 24, 25.3), 0.58, Name{english:Some("Betelgeuse".to_string()),japanese:Some("ベテルギウス".to_string())});
        let sample1 = Star::new(2799, Equatorial::new(5, 25, 10.29, true, 6, 24, 25.3), 0.58, Name{english:Some("sample1".to_string()),japanese:Some("サンプル1".to_string())});
        let sample2= Star::new(2798, Equatorial::new(5, 3, 10.29, true, 6, 0, 25.3), 0.58, Name{english:Some("sample2".to_string()),japanese:Some("サンプル2".to_string())});
        let mut constellation = Constellation::new(Name{english:None,japanese:None});
        constellation.add(&betelgeuse, &sample1);
        constellation.add(&betelgeuse, &sample2);
    }
}
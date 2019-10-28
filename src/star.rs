pub struct Equatorial{
    pub right_ascension: RightAscension,
    pub declination: Declination,
}

pub struct RightAscension{
    hour:usize,
    minuite:usize,
    second:f32,
}
pub struct Declination{
    north:bool,
    degree:usize,
    minuite: usize,
    second: f32,
}

impl RightAscension{
    pub fn new(hour:usize,minuite:usize,second:f32) -> Result<Self,(usize,usize,usize)>{
        let (overflow_hour, overflow_minuite, overflow_second)
            = (hour/12,minuite/60,second as usize /60);
        if overflow_hour==0 && overflow_minuite==0 && overflow_second==0{
            Ok(RightAscension{
                hour:hour,
                minuite:minuite,
                second:second,
            })
        }else{
            Err((overflow_hour,overflow_minuite,overflow_second))
        }
    }

    pub fn get_angle(&self) -> f32{
        self.hour as f32 * (360./24.) + self.minuite as f32 * (360./24./60.) + self.second * (360./24./60./60.)
    }
    
}
impl Declination{
    pub fn new(north:bool,degree:usize,minuite:usize,second:f32) -> Result<Self,(usize,usize,usize)>{
        let (overflow_degree,overflow_minuite, overflow_second)
            = (degree/90,minuite/60,second as usize /60);
        if overflow_degree==0 && overflow_minuite==0 && overflow_second==0{
            Ok(Self{
                north:north,
                degree:degree,
                minuite:minuite,
                second:second,
            })
        }else{
            Err((overflow_degree,overflow_minuite,overflow_second))
        }
    }

    pub fn get_angle(&self) -> f32{
        (self.degree as f32 + self.minuite as f32 * (1./60.) + self.second * (1./60./60.))
            * if self.north==true{
                1.
            }else{
                -1.
            }
    }
    
}

pub struct Name{
    pub english: Option<String>,
    pub japanese: Option<String>,
}

pub struct Star{
    hip: usize,
    pub location: Equatorial,
    pub magnitue: f32,
    pub name: Name,
}

impl Star {
    pub fn new(hip:usize,location:Equatorial,magnitue: f32, name:Name) -> Self{
        Self{
            hip:hip,location:location,magnitue:magnitue,name:name
        }
    }
    pub fn get_hip(&self) -> usize {
        self.hip
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn create_equatorial() {
        use super::*;
        let right_ascension = RightAscension::new(4, 32, 50.3).expect("Rignt_Ascension Overflowed");
        let right_ascension_angle = right_ascension.get_angle();
        assert!(68. < right_ascension_angle && right_ascension_angle < 68.5);
        let declination = Declination::new(true, 40, 32, 10.3).expect("Declination Overflowed");
        let declination_angle = declination.get_angle();
        assert!(40.5 < declination_angle && declination_angle < 40.6);
    }
}
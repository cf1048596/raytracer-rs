use std::ops::Neg;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::DivAssign;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;

impl Vec3 {

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn new_empty() -> Self {
        Self { e: [0.into(), 0.into(), 0.into()] }
    }

    pub fn get(&self, i: usize) -> f64 {
        self.e[i]
    }

    pub fn get_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }

    pub fn x(&self) -> f64  {
        self.e[0]
    }

    pub fn y(&self) -> f64  {
        self.e[1]
    }

    pub fn z(&self) -> f64  {
        self.e[2]
    }

    pub fn get_len_squared(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn get_len(&self) -> f64 {
        self.get_len_squared().sqrt()
    }
 
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
         Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
       self.e[0] += other.e[0];
       self.e[1] += other.e[1];
       self.e[2] += other.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        self.e[0] *= scalar;
        self.e[1] *= scalar;
        self.e[2] *= scalar;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        *self *= 1_f64/scalar
    }
}


impl Add for Vec3 {
    type Output =  Vec3;
    fn add(self, othervec: Vec3) -> Vec3 {
       Vec3 {
           e: [
                self.e[0] + othervec.e[0],
                self.e[1] + othervec.e[1],
                self.e[2] + othervec.e[2],
           ],
        }
    }
}

impl Sub for Vec3 {
    type Output =  Vec3;
    fn sub(self, othervec: Vec3) -> Vec3 {
       Vec3 {
           e: [
                self.e[0] - othervec.e[0],
                self.e[1] - othervec.e[1],
                self.e[2] - othervec.e[2],
           ],
        }
    }
}

impl Mul for Vec3 {
    type Output =  Vec3;
    fn mul(self, othervec: Vec3) -> Vec3 {
       Vec3 {
           e: [
                self.e[0] * othervec.e[0],
                self.e[1] * othervec.e[1],
                self.e[2] * othervec.e[2],
           ],
        }
    }
}


impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        (1_f64/self) * rhs
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Vec3 {
        (1_f64/scalar) * self
    }
}


pub fn dot(u : &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0]
    + u.e[1] * v.e[1]
    + u.e[2] * v.e[2]
}

pub fn cross(u : &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0])
}

pub fn unit_vector(u: &Vec3) -> Vec3 {
    *u / u.get_len()
}

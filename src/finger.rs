use rand::Rng;
pub struct Finger {
    pub target : (i64,i64),
    pub action : f64,
}
impl Finger {
    pub fn to_string(&self) -> String {
        format!("{:+03},{:+03},{:+020.15}", self.target.0, self.target.1, self.action)
    }   
}
pub fn finger_as_string(finger : &Finger) -> String {
    format!("{:+03},{:+03},{:+020.15}", finger.target.0, finger.target.1, finger.action)
}

pub fn finger(r_span : usize, c_span: usize,  pow : f64) -> Finger {
    let mut rng = rand::thread_rng();
    Finger {
        target : random_target(r_span,c_span),
        action : random_action(pow),
    }
}
pub fn box_finger(target : (i64,i64), pow : f64) -> Finger {
    let mut rng = rand::thread_rng();
    Finger {
        target,
        action : random_action(pow),
    }
}

pub fn finger_from_string(string : &str) -> Finger {
    let mut strings = string.split(",");
    let i = strings.next().unwrap().parse::<i64>().unwrap();
    let j = strings.next().unwrap().parse::<i64>().unwrap();
    let f = strings.next().unwrap().parse::<f64>().unwrap();
    Finger {
        target : (i,j),
        action : f,
    }
}
pub fn random_action( pow : f64) -> f64 {
    let mut rng = rand::thread_rng();
    (1.0 - 2.0*rand::random::<f64>())*pow
}

pub fn random_target(r : usize, c : usize) -> (i64,i64) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..r) as i64,rng.gen_range(0..c) as i64)
}



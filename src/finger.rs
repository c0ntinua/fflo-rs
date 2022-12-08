pub struct Finger {
    pub target : (i64,i64),
    pub action : f64,
}
impl Finger {
    pub fn to_string(&self) -> String {
        format!("{:+03},{:+03},{:+020.15}", self.target.0, self.target.1, self.action)
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



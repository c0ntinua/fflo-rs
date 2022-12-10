use crate::hand::*;
use crate::finger::*;

pub fn hand_from_string(string : &str) -> Hand {
    let mut fingers_as_strings  = string.split("\n").collect::<Vec<&str>>();
    println!("{:?}", fingers_as_strings);
    fingers_as_strings.iter().map(|x| finger_from_string(x)).collect()
}

pub fn hands_from_string(string : &str) -> Vec<Hand> {
    let mut hands_as_strings = string.split("\n\n").collect::<Vec<&str>>();
    hands_as_strings.iter().map(|x| hand_from_string(x)).collect()
}

pub fn string_from_hand(hand : &Hand) -> String {
    let mut string = "".to_owned();
    for finger in hand {
        string.push_str(&string_from_finger(&finger));
    }
    string
}

pub fn finger_as_string(finger : &Finger) -> String {
    format!("{:+03},{:+03},{:+020.15}", finger.target.0, finger.target.1, finger.action)
}

pub fn string_from_finger(finger : &Finger) -> String {
    format!("{:+03},{:+03},{:+020.15}\n", finger.target.0, finger.target.1, finger.action)
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
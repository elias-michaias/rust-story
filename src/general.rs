use std::io::{stdin,stdout,Write};
use rand::prelude::*;

//Wait for key press to proceed
pub fn pause() {
    println!("> ");
    let mut s = String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
}

//Random float
pub fn rand_f () -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    y
}

//Text input
pub fn input () -> String {
    println!("v");
    let mut s = String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
                s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
                s.pop();
    }
    return s;
}

//Randomly generate enemy appearance quote
pub fn enemyappear () -> String {
    let mut rng = rand::thread_rng();

    let mut lines: Vec<&str> = vec![
        "was looking to party, but found you instead!",
        "looked at you in a disturbed manner!",
        "doesn't seem pleased with you!",
        "was willing to strike up a conversation, until you frowned!",
        "was kind of hoping you'd be taller!",
        "said you're ugly!",
        "wanted to show you their rock collection, but you winced at their favorite stone!",
        "was wondering if you'd be interested in a double date!",
        "looks at you seductively!",
        "makes rude hand gestures at you!",
        "pointed at you and laughed!",
        "wants you to come visit the parents!",
        "seems to have vile plans for you!",
        "doesn't like the way you looked at their imaginary friend!",
        "is upset with the sight of you!",
        "would really rather be somewhere else right now!",
        "thinks you're pretty neat on the inside! (Woah!)",
        "wants to show you a magic trick!",
        "is really really hungry! (Seriously!)",
        "is technically vegetarian, but thinks you'll do for now!",
        "is just a wandering soul looking for affection to fill the void inside of them!",
        "was planning a dinner party with their boss, until you showed up!",
        "is upset that you interrupted them in the middle of something special!",
        "feels violated that you looked at them so strongly!",
        "is self-conscious about their ability to defeat player characters!",
        "is pretty livid about you walking on the pages of his manifesto! (Yikes!)",
        "doesn't appreciate the corporate odor that you strut around with!",
        "is a programmer with no social life, and he's taking it out on you!",
        "wants to break your kneecaps with macaroni!",
        "is curious about the last time you were beaten up!",
        "doesn't have time for losers like you!",
        "is fighting for a future free from smelly people like you!",
        "feels a little guilty about the philosophical considerations, but still would like to murder you!",
        "was rubbed the wrong way by your low-wage attitude!",
    ];
    lines.shuffle(&mut rng);
    lines[0].to_string()
}

//! # yn
//! 
//! Want to check if a value is yes, no, maybe both? Maybe you want to check if a value is somewhat yes? Somewhat no?
//! 
//! We've got you covered! This is state of the art.. EHEM.. Natural language processing. With yn, you will be the one who can safely say NO!
//! 
//! ## Examples
//! 
//! If you want to use this very advanced natural language processing technology you simply need to do the following:
//! 
//! To check if something is yes:
//! 
//! ```rust
//! extern crate yn;
//! 
//! assert!(yn::yes("yes")); // this is yes, you can be 100% sure
//! ```
//! 
//! If you don't have time to write yes:
//! 
//! ```rust
//! extern crate yn;
//! 
//! assert!(yn::yes("y")); // this is also yes
//! ```
//! 
//! If you're a bleepity blooping machine:
//! 
//! ```rust
//! extern crate yn;
//! 
//! assert!(yn::yes("1"));
//! ```
//! 
//! Of course, this is not yes:
//! 
//! ```rust
//! extern crate yn;
//! 
//! assert!(!yn::yes("no"));
//! ```
//! 
//! You can also check if something is somewhat yes:
//! 
//! ```rust
//! extern crate yn;
//! 
//! assert!(yn::is_somewhat_yes("this has a y so it is the word"));
//! ```
//! Or if you want to check if something just contains y:
//! 
//! ```rust
//! extern crate yn;
//! 
//! assert!(yn::is_kinda_yes("very much so"));
//! ```
//! 
//! The options are endless. Your imagination is the limit.
//! 
//! ## License
//! 
//! Licensed under MIT

use std::fmt::Display;

const YES: [&str; 4] = ["y", "yes", "true", "1"];
const NO: [&str; 4] = ["n", "no", "false", "0"];

/// Determines if a value is a form of yes.
/// 
/// Returns true when the value is and/or contains (see examples for clarification) `"y"`, `"yes"`, `true` or `1`.
/// 
/// # Examples
/// 
/// yes should be yes:
/// 
/// ```rust
/// assert!(yn::yes("y"));
/// assert!(yn::yes("yes"));
/// assert!(yn::yes("true true true really true"));
/// assert!(yn::yes("double up to make sure, yes yes y y true true 1 1"));
/// assert!(yn::yes("definitely yes"));
/// ```
/// 
/// should definitely not be yes:
/// 
/// ```rust
/// assert!(!yn::yes("no"));
/// assert!(!yn::yes("this is definitely a no."));
/// assert!(!yn::yes("false"));
/// ```
pub fn yes(string: impl Display) -> bool {
    let string = convert_to_something_that_may_be_yes_or_no(string);

    definitely_yes(&string) || string.to_lowercase().split_whitespace().any(|w| YES[1..].contains(&w))
}

/// Determines if a value is somewhat yes.
/// 
/// Returns true when the value is something in the trend of `"really y"`, `"wow so y"`, `very true`, `0 but is actually 1!`, `contains y here`.
/// 
/// # Examples
/// 
/// input should be somewhat yes:
/// 
/// ```rust
/// assert!(yn::is_somewhat_yes("yes")); // yes is still somewhat yes
/// assert!(yn::is_somewhat_yes("really y"));
/// assert!(yn::is_somewhat_yes("wow so y"));
/// assert!(yn::is_somewhat_yes("very true"));
/// assert!(yn::is_somewhat_yes("0 but is actually 1!"));
/// assert!(yn::is_somewhat_yes("contains y here"));
/// ```
/// 
/// should definitely not be somewhat yes:
/// 
/// ```rust
/// assert!(!yn::is_somewhat_yes("no no noooo"));
/// assert!(!yn::is_somewhat_yes("many no's"));
/// assert!(!yn::is_somewhat_yes("very very false"));
/// ```
pub fn is_somewhat_yes(string: impl Display) -> bool {
    let string = convert_to_something_that_may_be_yes_or_no(string);
    definitely_yes(&string) || string.to_lowercase().split_whitespace().any(|w| YES.contains(&w))
}

/// Determines if a value is kinda yes.
/// 
/// Returns true when the value is kinda yes `"yes"`, `"many no is still yes because many is yes"`.
/// 
/// # Examples
/// 
/// input should be kinda yes:
/// 
/// ```rust
/// assert!(yn::is_kinda_yes("yes")); // always make sure
/// assert!(yn::is_kinda_yes("many no is still yes because many is yes"));
/// ```
/// 
/// should definitely not be kinda yes:
/// 
/// ```rust
/// assert!(!yn::is_kinda_yes("no no noooo"));
/// assert!(!yn::is_kinda_yes("veri meni no's"));
/// assert!(!yn::is_kinda_yes("should write veri false now instead of ver..- ALMOST.. VER.. AAARGHHH!"));
/// ```
pub fn is_kinda_yes(string: impl Display) -> bool {
    convert_to_something_that_may_be_yes_or_no(string).contains(YES[0])
}

/// Determines if a value is a form of no.
/// 
/// Returns true when the value is and/or contains (see examples for clarification) `"n"`, `"no"`, `false` or `0`.
/// 
/// # Examples
/// 
/// no should be no:
/// 
/// ```rust
/// assert!(yn::no("n"));
/// assert!(yn::no("no"));
/// assert!(yn::no("false false false VERY false"));
/// assert!(yn::no("double up to make sure, no no n n false FALSE!! 0 0 !! AAARGGHH"));
/// assert!(yn::no("definitely no hhhrrrghhh *barks loudly*"));
/// ```
/// 
/// should definitely not be no:
/// 
/// ```rust
/// assert!(!yn::no("yes"));
/// assert!(!yn::no("this is definitely a yes."));
/// assert!(!yn::no("true"));
/// ```
pub fn no(string: impl Display) -> bool {
    let string = convert_to_something_that_may_be_yes_or_no(string);

    definitely_no(&string) || string.to_lowercase().split_whitespace().any(|w| NO[1..].contains(&w))
}

/// Determines if a value is somewhat no.
/// 
/// Returns true when the value is something in the trend of `"really n"`, `"wow so n"`, `very false`, `rawrr~ no ;3`, `contains n here`.
/// 
/// # Examples
/// 
/// input should be somewhat no:
/// 
/// ```rust
/// assert!(yn::is_somewhat_no("no")); // no is still somewhat no
/// assert!(yn::is_somewhat_no("really n"));
/// assert!(yn::is_somewhat_no("wow so n"));
/// assert!(yn::is_somewhat_no("very false"));
/// assert!(yn::is_somewhat_no("1 but is actually 0!"));
/// assert!(yn::is_somewhat_no("contains n here"));
/// ```
/// 
/// should definitely not be somewhat no:
/// 
/// ```rust
/// assert!(!yn::is_somewhat_no("yes"));
/// assert!(!yn::is_somewhat_no("very yes"));
/// ```
pub fn is_somewhat_no(string: impl Display) -> bool {
    let string = convert_to_something_that_may_be_yes_or_no(string);
    definitely_no(&string) || string.to_lowercase().split_whitespace().any(|w| NO.contains(&w))
}




/// Determines if a value is kinda no.
/// 
/// Returns true when the value is kinda no `"no"`, `"if it has a n somewhere"`.
/// 
/// # Examples
/// 
/// input should be kinda no:
/// 
/// ```rust
/// assert!(yn::is_kinda_no("no")); // always make sure
/// assert!(yn::is_kinda_no("n"));
/// ```
/// 
/// should definitely not be kinda no:
/// 
/// ```rust
/// assert!(!yn::is_somewhat_no("yes YES YEEEEEEEEEEEEEESSS"));
/// assert!(!yn::is_somewhat_no("oooh i ca.. i almost typed the character! :("));
/// assert!(!yn::is_somewhat_no("uuuuuuuuuuuuuu"));
/// ```
pub fn is_kinda_no(string: impl Display) -> bool {
    convert_to_something_that_may_be_yes_or_no(string).contains(NO[0])
}

fn convert_to_something_that_may_be_yes_or_no(string: impl Display) -> String {
    string.to_string().chars().filter(|c| c.is_alphabetic() || ['0', '1', ' '].contains(c)).collect::<String>()
}

fn definitely_yes(string: &str) -> bool {
    string == YES[0] || string == YES[1]
}

fn definitely_no(string: &str) -> bool {
    string == NO[0] || string == NO[1]
}

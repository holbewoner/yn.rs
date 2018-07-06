extern crate yn;

#[test]
fn test_yes() {
  assert!(yn::yes("y"));
  assert!(yn::yes("yes"));
  assert!(yn::yes("yes"));
  assert!(yn::yes("yes"));
  assert!(yn::yes("this contains a yes"));
  assert!(yn::yes("true"));
  assert!(yn::yes("1"));
  assert!(yn::yes("NO. Stop it. NOOO! haha jk it's actually yes"));
  assert!(yn::yes("yeah"));
  assert!(yn::yes("ya"));
  assert!(yn::yes("ye"));
  assert!(yn::yes("affirmative"));
  assert!(yn::yes("10-4"));
  assert!(yn::yes("aye aye captain"));
  assert!(yn::yes("yea"));
  assert!(yn::yes("yep yep yep"));
  assert!(yn::yes("I will never let you go, never let you down never gonna turn around and ye you"));
  assert!(yn::yes("Is this getting redicoulous? yes!"));
  assert!(yn::yes("Is TheUnitedStatesOfAmerica the greatest country known to mankind?, affirmative."));
  assert!(yn::yes("roger roger beep boop 10-4 roger"));
  assert!(yn::yes("is this some good tests, 1 2 3, nah or maybe they are"));
  assert!(yn::yes("1, 2, 3 Poof... Ferris have evolved"));
  assert!(!yn::yes("this contains a y"));
  assert!(!yn::yes("this contains a y here"));
  assert!(!yn::yes("this has many"));
  assert!(!yn::yes("no"));
  assert!(!yn::yes("this is definitely a no."));
  assert!(!yn::yes("rawrr~ x3"));
  assert!(!yn::yes("rawrr~ XD"));
  assert!(!yn::yes("yahoo"));
  assert!(!yn::yes("maybe"));
  assert!(!yn::yes("nearly"));
  assert!(!yn::yes("close enough"));
  assert!(!yn::yes("is this a no?"));
  assert!(!yn::yes("👀"));
  assert!(!yn::yes("aha"));
  assert!(!yn::yes("Will you marry me?"));
  assert!(!yn::yes("Yippee ki-yay"));
}

#[test]
fn test_is_somewhat_yes() {
  assert!(yn::is_somewhat_yes("y"));
  assert!(yn::is_somewhat_yes("yes"));
  assert!(yn::is_somewhat_yes("yes.."));
  assert!(yn::is_somewhat_yes("yes!!!"));
  assert!(yn::is_somewhat_yes("yeah"));
  assert!(yn::is_somewhat_yes("ya"));
  assert!(yn::is_somewhat_yes("this contains a yes"));
  assert!(yn::is_somewhat_yes("true"));
  assert!(yn::is_somewhat_yes("1"));
  assert!(yn::is_somewhat_yes("NO. Stop it. NOOO! haha jk it's actually yes"));
  assert!(yn::is_somewhat_yes("this contains a y"));
  assert!(yn::is_somewhat_yes("this contains a y here"));
  assert!(yn::yes("ye olde taven"));
  assert!(yn::yes("affirmative sir"));
  assert!(yn::yes("*radio scatter* 10-4 roger *radio scatter*"));
  assert!(yn::yes("aye aye captain, whoooo lives in a pineapple under the sea"));
  assert!(yn::yes("yea"));
  assert!(yn::yes("yep yep yep"));
  assert!(!yn::is_somewhat_yes("this has many"));
  assert!(!yn::is_somewhat_yes("no"));
  assert!(!yn::is_somewhat_yes("this is definitely a no."));
}

#[test]
fn test_is_kinda_yes() {
  assert!(yn::is_kinda_yes("yes"));
  assert!(yn::is_kinda_yes("y"));
  assert!(yn::is_kinda_yes("this has many"));
  assert!(yn::is_kinda_yes("not really"));
  assert!(!yn::is_kinda_yes("no"));
}

#[test]
fn test_no() {
  assert!(yn::no("n"));
  assert!(yn::no("no"));
  assert!(yn::no("no.."));
  assert!(yn::no("oh no"));
  assert!(yn::no("aaah no!"));
  assert!(yn::no("this contains a no"));
  assert!(yn::no("false"));
  assert!(yn::no("0"));
  assert!(yn::no("Yes! That's it! YES! haha jk it's actually no"));
  assert!(yn::no("nah"));
  assert!(yn::no("nope"));
  assert!(yn::no("nay"));
  assert!(yn::no("nix"));
  assert!(yn::no("not true"));
  assert!(yn::no("negative smiley"));
  assert!(yn::no("10-10"));
  assert!(yn::no("It does contain a y but it also contains a NO"));
  assert!(!yn::no("this contains a n"));
  assert!(!yn::no("this contains a n here"));
  assert!(!yn::no("this is down"));
  assert!(!yn::no("yes"));
  assert!(!yn::no("this is definitely a yes."));
  assert!(!yn::no("Pennope"));
  assert!(!yn::no("unix"));
  assert!(!yn::no("yep yep yep"));
  assert!(!yn::no("I will never let you go, never let you down never gonna turn around and ye you"));
  assert!(!yn::no("Is this getting redicoulous? yes!"));
  assert!(!yn::no("Is TheUnitedStatesOfAmerica the greatest country known to mankind?, affirmative."));
  assert!(!yn::no("roger roger beep boop 10-4 roger"));
  assert!(yn::no("is this some good tests, 1 2 3, nah or maybe they are"));
  assert!(!yn::no("1, 2, 3 Poof... Ferris have evolved"));

}

#[test]
fn test_is_somewhat_no() {
  assert!(yn::is_somewhat_no("n"));
  assert!(yn::is_somewhat_no("no"));
  assert!(yn::is_somewhat_no("no..")); // just checking again
  assert!(yn::is_somewhat_no("no!")); // really gotta make sure its yes, you never know
  assert!(yn::is_somewhat_no("this contains a no"));
  assert!(yn::is_somewhat_no("false"));
  assert!(yn::is_somewhat_no("0"));
  assert!(yn::is_somewhat_no("Yes! That's it! YES! haha jk it's actually no"));
  assert!(yn::is_somewhat_no("this contains a n"));
  assert!(yn::is_somewhat_no("this contains a n here"));
  assert!(yn::is_somewhat_no("nah nah"));
  assert!(yn::is_somewhat_no("nope lol"));
  assert!(yn::is_somewhat_no("nay nay"));
  assert!(yn::is_somewhat_no("*nix?"));
  assert!(yn::is_somewhat_no("not true"));
  assert!(yn::is_somewhat_no("negative smiley"));
  assert!(yn::is_somewhat_no("10-10"));
  assert!(yn::is_somewhat_no("0")); // As you know 10-10=0
  assert!(!yn::is_somewhat_no("is this yes?"));
  assert!(!yn::is_somewhat_no("this is down"));
  assert!(!yn::is_somewhat_no("yes"));
  assert!(!yn::is_somewhat_no("this is definitely a yes."));

}

#[test]
fn test_is_kinda_no() {
  assert!(yn::is_kinda_no("no"));
  assert!(yn::is_kinda_no("n"));
  assert!(yn::is_kinda_no("go down"));
  assert!(yn::is_kinda_no("affirnative"));
  assert!(!yn::is_kinda_no("yes"));
}

use crate::{Flag, flag_new};
use crate::macros;

#[test]
fn from_existing() {
    let flag = Flag::new_from_existing(136);
    assert_eq!(flag.fvec, vec![false, false, false, true, false, false, false, true]);
    assert_eq!(flag.get_flag(0).unwrap(), false);
}

#[test]
fn get_all() {
    let flag = Flag::new_from_existing(136);
    assert_eq!(flag.get_all_flags(), vec![3, 7]);
    let flag2 = Flag::new_from_existing(36);
    assert_eq!(flag2.get_all_flags(), vec![2, 5])
}

#[test]
fn padtest() {
    let mut flag = Flag::new_from_existing(136);
    flag.pad(10).unwrap();
    assert_eq!(flag.fvec, vec![false, false, false, true, false, false, false, true, false, false, false])
}

#[test]
fn settest() {
    let mut flag = Flag::new_from_existing(136);
    flag.set_flag(10, true);
    assert_eq!(flag.fvec, vec![false, false, false, true, false, false, false, true, false, false, true]);
    assert_eq!(true, flag.get_flag(10).unwrap());
    assert_eq!(true, flag.exists(10));
}

#[test]
fn flagmacro() {
    let flagm = flag_new![3, 7];
    let flagn = Flag::new_from_existing(136);
    assert_eq!(flagm.fvec, flagn.fvec);
}

#[test]
fn gettest() {
    let flag = flag_new![3, 7];
    assert_eq!(flag.get(), 136);
}
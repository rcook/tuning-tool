#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::str::FromStr;
    use tuning_tool_lib::U7;
    use tuning_tool_macros::U7;

    #[derive(Clone, Copy, Debug, PartialEq, U7)]
    struct MyU7(u8);

    #[test]
    fn basics() {
        assert_eq!(0, MyU7::ZERO.to_u8());
        assert_eq!(1, MyU7::ONE.to_u8());
        assert_eq!(0, MyU7::MIN.to_u8());
        assert_eq!(127, MyU7::MAX.to_u8());
    }

    #[test]
    #[should_panic]
    fn constant_panic() {
        MyU7::constant::<128>();
    }

    #[test]
    fn constant_no_panic() {
        MyU7::constant::<127>();
    }

    #[test]
    fn from_str() {
        let value: MyU7 = "123".parse().expect("Should succeed");
        assert_eq!(123, value.to_u8());

        let value: MyU7 = MyU7::from_str("123").expect("Should succeed");
        assert_eq!(123, value.to_u8());
    }

    #[test]
    fn from_str_failure() {
        assert!(MyU7::from_str("1234").is_err());
    }

    #[test]
    fn widening_succ() {
        assert_eq!(1, MyU7::ZERO.widening_succ());
        assert_eq!(2, MyU7::ONE.widening_succ());
        assert_eq!(1, MyU7::MIN.widening_succ());
        assert_eq!(128, MyU7::MAX.widening_succ());
    }

    #[test]
    fn widening_pred() {
        assert_eq!(-1, MyU7::ZERO.widening_pred());
        assert_eq!(0, MyU7::ONE.widening_pred());
        assert_eq!(-1, MyU7::MIN.widening_pred());
        assert_eq!(126, MyU7::MAX.widening_pred());
    }

    #[test]
    fn checked_succ() {
        let mut result = Vec::new();
        let mut cursor = Some(MyU7::ZERO);
        while let Some(note_number) = cursor {
            result.push(note_number.0);
            cursor = note_number.checked_succ();
        }

        assert_eq!(128, result.len());
        for (i, value) in result.iter().enumerate() {
            assert_eq!(i, *value as usize);
        }
    }

    #[test]
    fn checked_pred() {
        let mut result = Vec::new();
        let mut cursor = Some(MyU7::MAX);
        while let Some(note_number) = cursor {
            result.push(note_number.0);
            cursor = note_number.checked_pred();
        }

        assert_eq!(128, result.len());
        result.reverse();
        for (i, value) in result.iter().enumerate() {
            assert_eq!(i, *value as usize);
        }
    }

    #[test]
    fn widening_add() {
        assert_eq!(0, MyU7(0).widening_add(MyU7(0)));
        assert_eq!(150, MyU7(100).widening_add(MyU7(50)));
    }

    #[test]
    fn widening_sub() {
        assert_eq!(0, MyU7(0).widening_sub(MyU7(0)));
        assert_eq!(50, MyU7(100).widening_sub(MyU7(50)));
        assert_eq!(-50, MyU7(50).widening_sub(MyU7(100)));
    }

    #[test]
    fn checked_add() {
        assert_eq!(Some(MyU7(0)), MyU7(0).checked_add(MyU7(0)));
        assert_eq!(None, MyU7(100).checked_add(MyU7(50)));
    }

    #[test]
    fn checked_sub() {
        assert_eq!(Some(MyU7(0)), MyU7(0).checked_sub(MyU7(0)));
        assert_eq!(Some(MyU7(50)), MyU7(100).checked_sub(MyU7(50)));
        assert_eq!(None, MyU7(50).checked_sub(MyU7(100)));
    }

    #[test]
    fn all() {
        let mut result = Vec::new();
        for (i, value) in MyU7::all().enumerate() {
            assert_eq!(i, value.to_u8() as usize);
            result.push(value);
        }
        assert_eq!(128, result.len());
    }

    #[test]
    fn up_to() {
        let mut result = Vec::new();
        for (i, value) in MyU7::from_u8_lossy(10)
            .up_to(MyU7::from_u8_lossy(15))
            .expect("Must be valid")
            .enumerate()
        {
            assert_eq!(i + 10, value.to_u8() as usize);
            result.push(value.to_u8());
        }
        assert_eq!(6, result.len())
    }

    #[test]
    fn up_to_invalid() {
        assert!(MyU7::from_u8_lossy(20)
            .up_to(MyU7::from_u8_lossy(15))
            .is_none());
    }

    #[test]
    fn u7_trait() {
        /*
        fn checked_add(self, rhs: Self) -> Option<Self>;
        fn checked_sub(self, rhs: Self) -> Option<Self>;
        fn up_to(self, end: Self) -> Option<Self::Iter>;
             */
        fn test<U: Debug + PartialEq + U7>() {
            assert_eq!(0, U::ZERO.to_u8());
            assert_eq!(1, U::ONE.to_u8());
            assert_eq!(0, U::MIN.to_u8());
            assert_eq!(127, U::MAX.to_u8());
            assert_eq!(128, U::all().collect::<Vec<_>>().len());
            assert_eq!(128, U::MAX.widening_succ());
            assert_eq!(-1, U::MIN.widening_pred());
            assert!(U::MAX.checked_succ().is_none());
            assert_eq!(Some(U::ONE), U::ZERO.checked_succ());
            assert!(U::MIN.checked_pred().is_none());
            assert_eq!(Some(U::ZERO), U::ONE.checked_pred());
            assert_eq!(128, U::MAX.widening_add(U::ONE));
            assert_eq!(-1, U::MIN.widening_sub(U::ONE));
            assert!(U::MAX.checked_add(U::ONE).is_none());
            assert_eq!(Some(U::ONE), U::ZERO.checked_add(U::ONE));
            assert!(U::MIN.checked_sub(U::ONE).is_none());
            assert_eq!(Some(U::ZERO), U::ONE.checked_sub(U::ONE));
            assert_eq!(
                2,
                U::ZERO
                    .up_to(U::ONE)
                    .expect("Must succeed")
                    .collect::<Vec<_>>()
                    .len()
            );
        }
        test::<MyU7>();
    }
}

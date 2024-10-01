#[cfg(test)]
mod tests {
    use tuning_tool_derive::U7;
    use tuning_tool_lib::u7::U7;

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
        for (i, value) in MyU7::panicking_new(10)
            .up_to(MyU7::panicking_new(15))
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
        assert!(MyU7::panicking_new(20)
            .up_to(MyU7::panicking_new(15))
            .is_none());
    }

    #[test]
    #[should_panic]
    fn panicking_new() {
        MyU7::panicking_new(128);
    }
}

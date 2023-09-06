use deku::prelude::*;
use std::convert::{TryFrom, TryInto};

mod test_slice {
    use super::*;

    #[test]
    fn test_count_static() {
        #[derive(PartialEq, Debug, DekuRead, DekuWrite)]
        struct TestStruct<'a> {
            #[deku(count = "2")]
            data: &'a [u8],
        }

        let test_data: Vec<u8> = [0xAA, 0xBB].to_vec();

        let ret_read = TestStruct::try_from(test_data.as_ref()).unwrap();
        assert_eq!(
            TestStruct {
                data: test_data.as_ref()
            },
            ret_read
        );

        let ret_write: Vec<u8> = ret_read.try_into().unwrap();
        assert_eq!(test_data, ret_write);
    }

    #[test]
    fn test_count_from_field() {
        #[derive(PartialEq, Debug, DekuRead, DekuWrite)]
        struct TestStruct<'a> {
            count: u8,
            #[deku(count = "count")]
            data: &'a [u8],
        }

        let test_data: Vec<u8> = [0x02, 0xAA, 0xBB].to_vec();

        let ret_read = TestStruct::try_from(test_data.as_ref()).unwrap();
        assert_eq!(
            TestStruct {
                count: 0x02,
                data: &test_data[1..]
            },
            ret_read
        );

        let ret_write: Vec<u8> = ret_read.try_into().unwrap();
        assert_eq!(test_data, ret_write);
    }

    #[test]
    fn test_count_zero() {
        #[derive(PartialEq, Debug, DekuRead, DekuWrite)]
        struct TestStruct<'a> {
            #[deku(count = "0")]
            data: &'a [u8],
        }

        let test_data: Vec<u8> = [].to_vec();

        let ret_read = TestStruct::try_from(test_data.as_ref()).unwrap();
        assert_eq!(
            TestStruct {
                data: test_data.as_ref()
            },
            ret_read
        );

        let ret_write: Vec<u8> = ret_read.try_into().unwrap();
        assert_eq!(test_data, ret_write);
    }

    #[test]
    #[should_panic(expected = "Incomplete(NeedSize { bits: 8 })")]
    fn test_count_error() {
        #[derive(PartialEq, Debug, DekuRead, DekuWrite)]
        struct TestStruct<'a> {
            count: u8,
            #[deku(count = "count")]
            data: &'a [u8],
        }

        let test_data: Vec<u8> = [0x03, 0xAA, 0xBB].to_vec();

        let _ret_read = TestStruct::try_from(test_data.as_ref()).unwrap();
    }
}

mod test_vec {
    use super::*;

    #[test]
    fn test_count_static() {
        #[derive(PartialEq, Debug, DekuRead, DekuWrite)]
        struct TestStruct {
            #[deku(count = "2")]
            data: Vec<u8>,
        }

        let test_data: Vec<u8> = [0xAA, 0xBB].to_vec();

        let ret_read = TestStruct::try_from(test_data.as_ref()).unwrap();
        assert_eq!(
            TestStruct {
                data: vec![0xAA, 0xBB]
            },
            ret_read
        );

        let ret_write: Vec<u8> = ret_read.try_into().unwrap();
        assert_eq!(test_data, ret_write);
    }

    #[test]
    fn test_count_from_field() {
        #[derive(PartialEq, Debug, DekuRead, DekuWrite)]
        struct TestStruct {
            count: u8,
            #[deku(count = "count")]
            data: Vec<u8>,
        }

        let test_data: Vec<u8> = [0x02, 0xAA, 0xBB].to_vec();

        let ret_read = TestStruct::try_from(test_data.as_ref()).unwrap();
        assert_eq!(
            TestStruct {
                count: 0x02,
                data: vec![0xAA, 0xBB]
            },
            ret_read
        );

        let ret_write: Vec<u8> = ret_read.try_into().unwrap();
        assert_eq!(test_data, ret_write);
    }

    #[test]
    fn test_count_zero() {
        #[derive(PartialEq, Debug, DekuRead, DekuWrite)]
        struct TestStruct {
            #[deku(count = "0")]
            data: Vec<u8>,
        }

        let test_data: Vec<u8> = [].to_vec();

        let ret_read = TestStruct::try_from(test_data.as_ref()).unwrap();
        assert_eq!(TestStruct { data: vec![] }, ret_read);

        let ret_write: Vec<u8> = ret_read.try_into().unwrap();
        assert_eq!(test_data, ret_write);
    }

    #[test]
    #[should_panic(expected = "Incomplete(NeedSize { bits: 8 })")]
    fn test_count_error() {
        #[derive(PartialEq, Debug, DekuRead, DekuWrite)]
        struct TestStruct {
            count: u8,
            #[deku(count = "count")]
            data: Vec<u8>,
        }

        let test_data: Vec<u8> = [0x03, 0xAA, 0xBB].to_vec();

        let _ret_read = TestStruct::try_from(test_data.as_ref()).unwrap();
    }
}

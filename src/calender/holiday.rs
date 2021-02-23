pub mod holiday {
    use chrono::{NaiveDate, Datelike};
    use std::{fs, io};
    use std::str::FromStr;
    use std::path::Path;

    type Holiday = NaiveDate;

    #[derive(Default, Debug, Eq, PartialEq, Clone)]
    pub struct Holidays {
        holiday_list: Vec<Holiday>
    }

    impl Holidays {
        fn contain(self, date: NaiveDate) -> bool {
            self.holiday_list.iter().any(|&i| i == date)
        }

        pub fn get_from_holiday_jp(path: &Path) -> Result<Holidays, io::Error> {
            let buf = fs::read_to_string(path)?;
            let lines: Vec<&str> = buf.split('\n').collect();
            Holidays::from_str_list(lines)
        }

        fn from_str_list(str_list: Vec<&str>) -> Result<Holidays, io::Error> {
            let mut list = Vec::new();
            for line in str_list {
                // 文字列から日付だけ取り出してlistにつめる
                let date_result = extract_date(line);
                if date_result.is_ok() {
                    list.push(date_result.unwrap());
                }
            }
            Ok(Holidays { holiday_list: list })
        }
    }

    pub fn is_holiday(date: NaiveDate, holidays: Holidays) -> bool {
        is_weekend(date) || holidays.contain(date)
    }

    /// YYYY-mm-dd: hoge 形式の文字列を日付型にして取り出す。
    /// 日付がないなら取得しない
    ///
    /// ```
    /// let date:NaiveDate = extract_date("2021-01-01: 元旦")?
    /// ```
    fn extract_date(str: &str) -> Result<NaiveDate, chrono::ParseError> {
        let p: Vec<&str> = str.split(": ").collect();
        NaiveDate::from_str(p[0])
    }

    /// Check Weekend.
    ///
    /// Return true if date is Saturday or Sunday
    ///
    /// ```
    /// let local_date: NaiveDate = NaiveDate::from_str("2021-02-22").unwrap();
    /// let is_weekend = is_weekend(local_date);
    /// ```
    fn is_weekend(date: NaiveDate) -> bool {
        match date.weekday() {
            chrono::Weekday::Sat |
            chrono::Weekday::Sun => true,
            _ => false
        }
    }

    #[cfg(test)]
    mod tests {
        use chrono::{NaiveDate};
        use std::str::FromStr;
        use super::*;

        const TEST_DATA: &'static [&'static str] = &[
            "---",
            "1970-01-01: 元日",
            "1970-01-15: 成人の日",
            "1970-02-11: 建国記念の日",
        ];

        #[test]
        fn is_weekend() {
            // 土曜日
            let date: NaiveDate = NaiveDate::from_str("2021-02-20").unwrap();
            assert_eq!(super::is_weekend(date), true);

            // 日曜日
            let date: NaiveDate = NaiveDate::from_str("2021-02-21").unwrap();
            assert_eq!(super::is_weekend(date), true);

            // 火曜日
            let date: NaiveDate = NaiveDate::from_str("2021-02-23").unwrap();
            assert_eq!(super::is_weekend(date), false);
        }

        #[test]
        fn extract_date() {
            // 正常
            let actual = super::extract_date("1970-01-01: 元日").unwrap();
            let expect = NaiveDate::from_str("1970-01-01").unwrap();
            assert_eq!(actual, expect);

            // ---
            assert_eq!(super::extract_date("---").is_err(), true);
        }

        #[test]
        fn from_str_list() {
            let holidays = Holidays {
                holiday_list: vec![
                    NaiveDate::from_str("1970-01-01").unwrap(),
                    NaiveDate::from_str("1970-01-15").unwrap(),
                    NaiveDate::from_str("1970-02-11").unwrap()
                ]
            };

            assert_eq!(Holidays::from_str_list(TEST_DATA.to_vec()).unwrap(), holidays);
        }

        #[test]
        fn contain() {
            let holidays = Holidays {
                holiday_list: vec![
                    NaiveDate::from_str("1970-01-01").unwrap(),
                    NaiveDate::from_str("1970-01-15").unwrap(),
                    NaiveDate::from_str("1970-02-11").unwrap()
                ]
            };

            fn test(date: &str, holidays: Holidays) -> bool {
                holidays.contain(NaiveDate::from_str(date).unwrap())
            }

            // 含まれている
            assert_eq!(test("1970-01-01", holidays.clone()), true);

            // 含まれていない
            assert_eq!(test("1970-01-02", holidays.clone()), false);
        }

        #[test]
        fn is_holiday() {
            let holidays = Holidays {
                holiday_list: vec![
                    NaiveDate::from_str("1970-01-01").unwrap(),
                    NaiveDate::from_str("1970-01-15").unwrap(),
                    NaiveDate::from_str("1970-02-11").unwrap()
                ]
            };

            fn test(date: &str, holidays: Holidays) -> bool {
                super::is_holiday(NaiveDate::from_str(date).unwrap(), holidays)
            }

            // 祝日 -> true
            assert_eq!(test("1970-01-01", holidays.clone()), true);

            // 平日 -> false
            assert_eq!(test("1970-01-02", holidays.clone()), false);

            // 土曜日 -> true
            assert_eq!(test("1970-01-03", holidays.clone()), true);

            // 日曜日 -> true
            assert_eq!(test("1970-01-04", holidays.clone()), true);
        }

        #[test]
        fn get_from_holiday_jp() {
            let holidays = Holidays::get_from_holiday_jp(Path::new("./res/holidays.yml")).unwrap();

            fn test(date: &str, holidays: Holidays) -> bool {
                super::is_holiday(NaiveDate::from_str(date).unwrap(), holidays)
            }

            // 祝日 -> true
            assert_eq!(test("1970-01-01", holidays.clone()), true);

            // 平日 -> false
            assert_eq!(test("1970-01-02", holidays.clone()), false);

            // 土曜日 -> true
            assert_eq!(test("1970-01-03", holidays.clone()), true);

            // 日曜日 -> true
            assert_eq!(test("1970-01-04", holidays.clone()), true);
        }
    }
}
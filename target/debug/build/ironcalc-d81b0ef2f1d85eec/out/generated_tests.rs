#[allow(non_snake_case)]
#[test]
fn test_calc_DATABASE_DAVERAGE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATABASE/DAVERAGE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATABASE/DAVERAGE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATABASE_DCOUNT_DCOUNTA_DGET_DPRODUCT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATABASE/DCOUNT_DCOUNTA_DGET_DPRODUCT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATABASE/DCOUNT_DCOUNTA_DGET_DPRODUCT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATABASE_DMIN_DMAX_DAVERAGE_DSUM() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATABASE/DMIN_DMAX_DAVERAGE_DSUM.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATABASE/DMIN_DMAX_DAVERAGE_DSUM.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATABASE_DVAR_DVARP_DSTDEV_DSTDEVP() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATABASE/DVAR_DVARP_DSTDEV_DSTDEVP.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATABASE/DVAR_DVARP_DSTDEV_DSTDEVP.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_DATE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/DATE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/DATE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_DATEVALUE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/DATEVALUE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/DATEVALUE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_DATEVALUE_issue_1379() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/DATEVALUE_issue_1379.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/DATEVALUE_issue_1379.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_DATEVALUE_issue_1380() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/DATEVALUE_issue_1380.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/DATEVALUE_issue_1380.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_DATE_DAY_MONTH_YEAR() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/DATE_DAY_MONTH_YEAR.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/DATE_DAY_MONTH_YEAR.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_DAYS_DAYS360() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/DAYS_DAYS360.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/DAYS_DAYS360.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_DAY_MONTH_YEAR() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/DAY_MONTH_YEAR.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/DAY_MONTH_YEAR.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_EDATE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/EDATE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/EDATE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_EOMONTH() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/EOMONTH.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/EOMONTH.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_EOMONTH_issue_1388() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/EOMONTH_issue_1388.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/EOMONTH_issue_1388.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_NETWORKDAYS_NETWORKDAYS_INTL() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/NETWORKDAYS_NETWORKDAYS.INTL.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/NETWORKDAYS_NETWORKDAYS.INTL.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_NOW_TODAY() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/NOW_TODAY.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/NOW_TODAY.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_TIMEVALUE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/TIMEVALUE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/TIMEVALUE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_TIMEVALUE_issue_1402() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/TIMEVALUE_issue_1402.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/TIMEVALUE_issue_1402.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_TIME_HOUR_MINUTE_SECOND() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/TIME_HOUR_MINUTE_SECOND.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/TIME_HOUR_MINUTE_SECOND.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_WEEKDAY_WEEKNUM_ISOWEEKNUM() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/WEEKDAY_WEEKNUM_ISOWEEKNUM.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/WEEKDAY_WEEKNUM_ISOWEEKNUM.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_WEEKDAY_WEEKNUM_ISOWEEKNUM_issue_1502() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/WEEKDAY_WEEKNUM_ISOWEEKNUM_issue_1502.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/WEEKDAY_WEEKNUM_ISOWEEKNUM_issue_1502.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_WORKDAY_WORKDAY_INTL() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/WORKDAY_WORKDAY.INTL.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/WORKDAY_WORKDAY.INTL.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_DATE_AND_TIME_YEARFRAC_DATEDIF() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/DATE_AND_TIME/YEARFRAC_DATEDIF.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/DATE_AND_TIME/YEARFRAC_DATEDIF.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_ENGINEERING_BESSEL() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/ENGINEERING/BESSEL.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/ENGINEERING/BESSEL.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_ENGINEERING_BITs() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/ENGINEERING/BITs.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/ENGINEERING/BITs.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_ENGINEERING_COMPLEXs() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/ENGINEERING/COMPLEXs.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/ENGINEERING/COMPLEXs.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_ENGINEERING_CONVERT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/ENGINEERING/CONVERT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/ENGINEERING/CONVERT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_ENGINEERING_ERFs() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/ENGINEERING/ERFs.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/ENGINEERING/ERFs.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_ENGINEERING_GESTEP_DELTA() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/ENGINEERING/GESTEP_DELTA.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/ENGINEERING/GESTEP_DELTA.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_ENGINEERING_NUMBER_SYSTEMS() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/ENGINEERING/NUMBER_SYSTEMS.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/ENGINEERING/NUMBER_SYSTEMS.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_ACCRINT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/ACCRINT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/ACCRINT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_ACCRINTM() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/ACCRINTM.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/ACCRINTM.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_ACCRINT_ACCRINTM() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/ACCRINT_ACCRINTM.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/ACCRINT_ACCRINTM.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_ACCRINT_extended() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/ACCRINT_extended.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/ACCRINT_extended.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_COUPDAYS() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/COUPDAYS.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/COUPDAYS.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_CUMPRINC_CUMIPMT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/CUMPRINC_CUMIPMT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/CUMPRINC_CUMIPMT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_DB_DDB() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/DB_DDB.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/DB_DDB.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_DISC() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/DISC.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/DISC.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_FV() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/FV.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/FV.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_FVSCHEDULE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/FVSCHEDULE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/FVSCHEDULE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_INTRATE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/INTRATE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/INTRATE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_IPMT_PPMT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/IPMT_PPMT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/IPMT_PPMT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_IRR() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/IRR.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/IRR.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_ISPMT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/ISPMT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/ISPMT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_MIRR() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/MIRR.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/MIRR.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_NOMINAL_EFFECT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/NOMINAL_EFFECT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/NOMINAL_EFFECT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_NPER() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/NPER.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/NPER.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_NPV() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/NPV.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/NPV.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_PDURATION() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/PDURATION.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/PDURATION.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_PMT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/PMT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/PMT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_PRICE_YIELD() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/PRICE_YIELD.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/PRICE_YIELD.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_PV() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/PV.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/PV.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_RATE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/RATE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/RATE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_RRI() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/RRI.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/RRI.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_SLN_SYD() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/SLN_SYD.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/SLN_SYD.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_TBILLs() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/TBILLs.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/TBILLs.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_XIRR() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/XIRR.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/XIRR.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_XNPV() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/XNPV.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/XNPV.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_FINANCIAL_accrint_cases() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/FINANCIAL/accrint_cases.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/FINANCIAL/accrint_cases.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_INFORMATION_CELL() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/INFORMATION/CELL.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/INFORMATION/CELL.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_INFORMATION_ERROR_TYPE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/INFORMATION/ERROR.TYPE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/INFORMATION/ERROR.TYPE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_INFORMATION_FORMULATEXT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/INFORMATION/FORMULATEXT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/INFORMATION/FORMULATEXT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_INFORMATION_ISOMITTED() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/INFORMATION/ISOMITTED.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/INFORMATION/ISOMITTED.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_INFORMATION_ISREF() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/INFORMATION/ISREF.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/INFORMATION/ISREF.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_INFORMATION_ISREF_ISFORMULA_ISODD_ISEVEN() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/INFORMATION/ISREF_ISFORMULA_ISODD_ISEVEN.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/INFORMATION/ISREF_ISFORMULA_ISODD_ISEVEN.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_INFORMATION_IS_INFORMATION() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/INFORMATION/IS_INFORMATION.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/INFORMATION/IS_INFORMATION.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_INFORMATION_N() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/INFORMATION/N.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/INFORMATION/N.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_INFORMATION_SHEET_SHEETS() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/INFORMATION/SHEET_SHEETS.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/INFORMATION/SHEET_SHEETS.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_INFORMATION_TYPE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/INFORMATION/TYPE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/INFORMATION/TYPE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_INFORMATION_error_broadcast() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/INFORMATION/error_broadcast.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/INFORMATION/error_broadcast.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOGICAL_AND_OR_XOR() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOGICAL/AND_OR_XOR.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOGICAL/AND_OR_XOR.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOGICAL_BYCOL_BYROW() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOGICAL/BYCOL_BYROW.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOGICAL/BYCOL_BYROW.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOGICAL_IFERROR() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOGICAL/IFERROR.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOGICAL/IFERROR.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOGICAL_IFNA() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOGICAL/IFNA.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOGICAL/IFNA.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOGICAL_IFS() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOGICAL/IFS.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOGICAL/IFS.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOGICAL_IF_ARRAY() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOGICAL/IF_ARRAY.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOGICAL/IF_ARRAY.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOGICAL_LAMBDA() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOGICAL/LAMBDA.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOGICAL/LAMBDA.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOGICAL_LET() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOGICAL/LET.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOGICAL/LET.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOGICAL_MAKEARRAY() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOGICAL/MAKEARRAY.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOGICAL/MAKEARRAY.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOGICAL_MAP_REDUCE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOGICAL/MAP_REDUCE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOGICAL/MAP_REDUCE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOGICAL_SCAN() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOGICAL/SCAN.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOGICAL/SCAN.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_ADDRESS() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/ADDRESS.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/ADDRESS.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_CHOOSE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/CHOOSE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/CHOOSE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_CHOOSECOLS_CHOOSEROWS() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/CHOOSECOLS_CHOOSEROWS.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/CHOOSECOLS_CHOOSEROWS.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_DROP_TAKE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/DROP_TAKE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/DROP_TAKE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_EXPAND() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/EXPAND.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/EXPAND.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_FILTER_ERRORS() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/FILTER_ERRORS.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/FILTER_ERRORS.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_HSTACK_VSTACK() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/HSTACK_VSTACK.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/HSTACK_VSTACK.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_HVLOOKUP() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/HVLOOKUP.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/HVLOOKUP.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_INDEX() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/INDEX.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/INDEX.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_INDIRECT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/INDIRECT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/INDIRECT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_MATCH() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/MATCH.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/MATCH.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_MATCH_LOOKUP_arrays() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/MATCH_LOOKUP_arrays.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/MATCH_LOOKUP_arrays.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_ROW_COLUM() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/ROW_COLUM.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/ROW_COLUM.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_SORT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/SORT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/SORT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_TOCOL() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/TOCOL.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/TOCOL.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_TRANSPOSE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/TRANSPOSE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/TRANSPOSE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_TRIMRANGE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/TRIMRANGE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/TRIMRANGE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_WRAPCOLS_ROWS() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/WRAPCOLS_ROWS.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/WRAPCOLS_ROWS.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_XLOOKUP() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/XLOOKUP.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/XLOOKUP.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_XMATCH() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/XMATCH.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/XMATCH.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_LOOKUP_AND_REFERENCE_XMATCH_arrays() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/LOOKUP_AND_REFERENCE/XMATCH_arrays.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/LOOKUP_AND_REFERENCE/XMATCH_arrays.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_ABS() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/ABS.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/ABS.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_ARABIC_ROMAN() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/ARABIC_ROMAN.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/ARABIC_ROMAN.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_ATAN2_POWER() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/ATAN2_POWER.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/ATAN2_POWER.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_BASE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/BASE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/BASE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_COMBIN_COMBINA() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/COMBIN_COMBINA.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/COMBIN_COMBINA.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_DEGREES_RADIANS() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/DEGREES_RADIANS.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/DEGREES_RADIANS.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_EVEN_ODD() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/EVEN_ODD.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/EVEN_ODD.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_EXP_SIGN() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/EXP_SIGN.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/EXP_SIGN.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_FACT_DOUBLEFACT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/FACT_DOUBLEFACT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/FACT_DOUBLEFACT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_FLOOR_CEILING() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/FLOOR_CEILING.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/FLOOR_CEILING.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_GCD_LCM() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/GCD_LCM.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/GCD_LCM.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_LCM_GCD() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/LCM_GCD.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/LCM_GCD.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_LOG_LOG10_LN() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/LOG_LOG10_LN.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/LOG_LOG10_LN.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_MAX_MIN() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/MAX_MIN.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/MAX_MIN.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_MIN_MAX_extras() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/MIN_MAX_extras.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/MIN_MAX_extras.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_MMULT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/MMULT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/MMULT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_MOD_QUOTIENT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/MOD_QUOTIENT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/MOD_QUOTIENT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_MROUND_TRUNC_INT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/MROUND_TRUNC_INT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/MROUND_TRUNC_INT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_MULTINOMIAL() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/MULTINOMIAL.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/MULTINOMIAL.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_MUNIT_MDETERM_MINVERSE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/MUNIT_MDETERM_MINVERSE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/MUNIT_MDETERM_MINVERSE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_POWER() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/POWER.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/POWER.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_PRODUCT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/PRODUCT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/PRODUCT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_PRODUCT_SUM() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/PRODUCT_SUM.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/PRODUCT_SUM.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_ROUND() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/ROUND.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/ROUND.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_SEQUENCE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/SEQUENCE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/SEQUENCE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_SERIESSUM() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/SERIESSUM.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/SERIESSUM.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_SQRT_SQRTPI() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/SQRT_SQRTPI.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/SQRT_SQRTPI.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_SUMIFS() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/SUMIFS.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/SUMIFS.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_SUMIF_AVERAGE_IF() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/SUMIF_AVERAGE_IF.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/SUMIF_AVERAGE_IF.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_SUMIF_array() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/SUMIF_array.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/SUMIF_array.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_SUMPRODUCT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/SUMPRODUCT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/SUMPRODUCT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_SUMSQ() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/SUMSQ.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/SUMSQ.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_SUMX2MY2_SUMX2PY2_SUMXMY2() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/SUMX2MY2_SUMX2PY2_SUMXMY2.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/SUMX2MY2_SUMX2PY2_SUMXMY2.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_MATH_AND_TRIGONOMETRY_TRIGONOMETRIC() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/MATH_AND_TRIGONOMETRY/TRIGONOMETRIC.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/MATH_AND_TRIGONOMETRY/TRIGONOMETRIC.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_AVEDEV() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/AVEDEV.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/AVEDEV.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_AVERAGE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/AVERAGE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/AVERAGE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_BETADIST_BETAINV() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/BETADIST_BETAINV.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/BETADIST_BETAINV.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_BETA_GAMMA() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/BETA_GAMMA.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/BETA_GAMMA.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_BINOM() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/BINOM.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/BINOM.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_CHISQ() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/CHISQ.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/CHISQ.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_CHISQ_TEST() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/CHISQ_TEST.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/CHISQ_TEST.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_CONFIDENCE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/CONFIDENCE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/CONFIDENCE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_CORREL_SLOPE_INTERCEPT_RSQ_STEYX() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/CORREL_SLOPE_INTERCEPT_RSQ_STEYX.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/CORREL_SLOPE_INTERCEPT_RSQ_STEYX.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_COUNT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/COUNT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/COUNT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_COUNTIF() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/COUNTIF.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/COUNTIF.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_COUNTIFS() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/COUNTIFS.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/COUNTIFS.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_COVARIANCE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/COVARIANCE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/COVARIANCE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_DEVSQ() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/DEVSQ.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/DEVSQ.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_EXPONDIST() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/EXPONDIST.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/EXPONDIST.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_EXPON_DIST() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/EXPON_DIST.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/EXPON_DIST.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_FISHER() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/FISHER.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/FISHER.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_FISHER_FISHERINV() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/FISHER_FISHERINV.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/FISHER_FISHERINV.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_FREQUENCY() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/FREQUENCY.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/FREQUENCY.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_F_DIST() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/F_DIST.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/F_DIST.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_F_TEST() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/F_TEST.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/F_TEST.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_GAMMA() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/GAMMA.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/GAMMA.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_GAMMALN_GAMMALNPRECISE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/GAMMALN_GAMMALNPRECISE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/GAMMALN_GAMMALNPRECISE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_GAUSS() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/GAUSS.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/GAUSS.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_GEOMEAN() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/GEOMEAN.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/GEOMEAN.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_HYPERGEOM_DIST() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/HYPERGEOM_DIST.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/HYPERGEOM_DIST.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_LINEST_LOGEST() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/LINEST_LOGEST.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/LINEST_LOGEST.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_LOGNORM() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/LOGNORM.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/LOGNORM.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_MEADIAN_KURT_SKEW_HARMEAN() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/MEADIAN_KURT_SKEW_HARMEAN.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/MEADIAN_KURT_SKEW_HARMEAN.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_MINA_MAXA() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/MINA_MAXA.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/MINA_MAXA.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_MIN_MAX() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/MIN_MAX.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/MIN_MAX.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_MODE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/MODE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/MODE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_NORM_DIST() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/NORM_DIST.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/NORM_DIST.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_PEARSON() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/PEARSON.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/PEARSON.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_PERCENTIL_QUARTIL() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/PERCENTIL_QUARTIL.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/PERCENTIL_QUARTIL.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_PERMUT_PERMUTATIONA() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/PERMUT_PERMUTATIONA.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/PERMUT_PERMUTATIONA.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_PHI() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/PHI.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/PHI.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_POISSON() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/POISSON.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/POISSON.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_PROB() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/PROB.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/PROB.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_RANK_EQ_RANK_AVG() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/RANK_EQ_RANK_AVG.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/RANK_EQ_RANK_AVG.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_SMALL_LARGE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/SMALL_LARGE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/SMALL_LARGE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_STANDARIZE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/STANDARIZE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/STANDARIZE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_STDEV() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/STDEV.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/STDEV.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_TREND_GROWTH() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/TREND_GROWTH.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/TREND_GROWTH.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_TRIMMEAN() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/TRIMMEAN.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/TRIMMEAN.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_T_DIST() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/T_DIST.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/T_DIST.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_T_TEST() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/T_TEST.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/T_TEST.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_VARIANCE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/VARIANCE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/VARIANCE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_WEIBULL_DIST() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/WEIBULL_DIST.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/WEIBULL_DIST.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_STATISTICAL_Z_TEST() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/STATISTICAL/Z_TEST.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/STATISTICAL/Z_TEST.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_ARRAYTOTEXT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/ARRAYTOTEXT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/ARRAYTOTEXT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_CHAR() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/CHAR.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/CHAR.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_CONCAT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/CONCAT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/CONCAT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_CONCATENATE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/CONCATENATE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/CONCATENATE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_DOLLARs() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/DOLLARs.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/DOLLARs.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_EXACT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/EXACT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/EXACT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_FIND_SEARCH() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/FIND_SEARCH.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/FIND_SEARCH.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_MID_LEFT_RIGHT_arrays() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/MID_LEFT_RIGHT_arrays.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/MID_LEFT_RIGHT_arrays.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_MINUS_MINUS() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/MINUS_MINUS.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/MINUS_MINUS.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_PROPER() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/PROPER.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/PROPER.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_REGEX() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/REGEX.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/REGEX.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_REPT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/REPT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/REPT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_STRING_HANDLING() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/STRING_HANDLING.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/STRING_HANDLING.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_SUBSTITUTE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/SUBSTITUTE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/SUBSTITUTE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_TEXT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/TEXT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/TEXT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_TEXTBEFORE_TEXTAFTER() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/TEXTBEFORE_TEXTAFTER.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/TEXTBEFORE_TEXTAFTER.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_TEXTJOIN() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/TEXTJOIN.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/TEXTJOIN.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_TEXTSPLIT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/TEXTSPLIT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/TEXTSPLIT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_T_VALUE_VALUETOTEXT() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/T_VALUE_VALUETOTEXT.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/T_VALUE_VALUETOTEXT.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_UNICODE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/UNICODE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/UNICODE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_TEXT_UPPER_LOWER() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/TEXT/UPPER_LOWER.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/TEXT/UPPER_LOWER.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_arithmetic() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/arithmetic.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/arithmetic.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_array_in_scalar_repro() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/array_in_scalar_repro.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/array_in_scalar_repro.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_consume_string() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/consume_string.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/consume_string.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_defined_names() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/defined_names.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/defined_names.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_defined_names_for_unit_test() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/defined_names_for_unit_test.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/defined_names_for_unit_test.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_escape_strings() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/escape_strings.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/escape_strings.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_issue_341() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/issue_341.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/issue_341.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_logical() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/logical.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/logical.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_percentage() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/percentage.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/percentage.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_quotes() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/quotes.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/quotes.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_range_operator() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/range_operator.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/range_operator.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_simple_arrays() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/simple_arrays.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/simple_arrays.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_simple_functions() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/simple_functions.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/simple_functions.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_calc_spill_operator() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/calc_tests/spill_operator.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/calc_tests/spill_operator.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_no_export_tables() {
    ironcalc::compare::test_file("tests/calc_test_no_export/tables.xlsx").unwrap_or_else(|e| panic!("{}", e));
}

#[allow(non_snake_case)]
#[test]
fn test_templates_crossword() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/templates/crossword.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/templates/crossword.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_templates_event_calendar() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/templates/event_calendar.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/templates/event_calendar.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_templates_gantt_project_tracker() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/templates/gantt_project_tracker.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/templates/gantt_project_tracker.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_templates_invoice() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/templates/invoice.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/templates/invoice.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_templates_travel_expenses_tracker() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/templates/travel_expenses_tracker.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/templates/travel_expenses_tracker.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_templates_weekly_timesheet() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/templates/weekly_timesheet.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/templates/weekly_timesheet.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_templates_wordle() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/templates/wordle.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/templates/wordle.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_templates_yearly_calendar() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/templates/yearly_calendar.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/templates/yearly_calendar.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_docs_CHOOSE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/docs/CHOOSE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/docs/CHOOSE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_docs_COS() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/docs/COS.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/docs/COS.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_docs_ERF_PRECISE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/docs/ERF-PRECISE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/docs/ERF-PRECISE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_docs_ERF() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/docs/ERF.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/docs/ERF.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_docs_ERFC_PRECISE() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/docs/ERFC-PRECISE.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/docs/ERFC-PRECISE.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_docs_ERFC() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/docs/ERFC.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/docs/ERFC.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_docs_FV() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/docs/FV.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/docs/FV.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_docs_PV() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/docs/PV.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/docs/PV.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}

#[allow(non_snake_case)]
#[test]
fn test_docs_SIN() {
    let temp_folder = std::env::temp_dir();
    let dir = temp_folder.join(format!("{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let result = std::panic::catch_unwind(|| {
        ironcalc::compare::test_file("tests/docs/SIN.xlsx").unwrap_or_else(|e| panic!("{}", e));
        ironcalc::compare::test_load_and_saving("tests/docs/SIN.xlsx", &dir)
            .unwrap_or_else(|e| panic!("{}", e));
    });
    std::fs::remove_dir_all(&dir).unwrap();
    result.unwrap();
}


use std::{io, path::Path};

use calamine::{open_workbook, Reader, Xls, Xlsx};

// ! # let path = format!("{}/tests/issue3.xlsm", env!("CARGO_MANIFEST_DIR"));
// ! let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");

//ugly return string as errmsg
pub fn open_1(file: &Path) -> Result<(), String> {
    let mut path = format!("{}/", env!("CARGO_MANIFEST_DIR"));
    let f = file.to_str().unwrap();
    path.push_str(f);
    let mut err_msg = "".to_string();

    if file.exists() {
        let aa: Result<calamine::Xlsx<_>, calamine::XlsxError> = open_workbook(path);
        if let Ok(mut wb) = aa {
            let mut sht = wb.worksheet_range("sheet1");
            if let Some(sht_rst) = sht {
                if let Ok(sheet) = sht_rst {
                    let start = sheet.start();
                    let end = sheet.end();
                    match (start, end) {
                        (None, None) => todo!(),
                        (None, Some(_)) => todo!(),
                        (Some(_), None) => todo!(),
                        (Some((s1, s2)), Some((e1, e2))) => {
                            for i in s1..s2 {
                                for j in e1..e2 {
                                    let cell = sheet.get_value((i, j));
                                }
                            }
                        }
                    }
                } else {
                    err_msg = "sheet get range failed".to_string();
                    return Err(err_msg);
                    //error miss, can not get XlsxError,
                }
            } else {
                err_msg = "sheet is None".to_string();
                return Err(err_msg);
            }
            //    let mut sheet = workbook.worksheet_range("Sheet1")
        }
        let mut err_msg_1 = "sheet not exist".to_string();
        let mut err_msg_2 = "cell empty".to_string();
        Err(err_msg_1)
    } else {
        let mut err_msg = "".to_string();
        let file = file.to_str();
        if let Some(path) = file {
            err_msg = path.to_owned();
            err_msg.push_str(" hard disk io");
        } else {
            err_msg = "file path is empty".to_string();
        }
        Err(err_msg)
    }
}
//what open_workbook return
// ugly, error miss, error char match

//return my Error Type
#[derive(Debug)]
pub enum FileError {
    FileNone(String),
    SheetError,
    RangeError,
    IOError(io::Error),
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileError::FileNone(s1) => {
                write!(f, "error msg display file none Error:{},", s1)
            }
            FileError::SheetError => {
                write!(f, "error msg display sheet open error")
            }
            FileError::RangeError => {
                write!(f, "error msg display Range found Error")
            }
            FileError::IOError(e) => {
                write!(f, "error msg display IO Error:{}", e)
            }
        }
    }
}

pub fn open_2(file: &Path) -> Result<(), FileError> {
    let mut path = format!("{}/file", env!("CARGO_MANIFEST_DIR"));
    let f = file.to_str().unwrap();
    path.push_str(f);
    let mut err_msg = "".to_string();

    if file.exists() {
        let aa: Result<calamine::Xlsx<_>, calamine::XlsxError> = open_workbook(path.clone());
        if let Ok(mut wb) = aa {
            let mut sht = wb.worksheet_range("sheet1");
            if let Some(sht_rst) = sht {
                if let Ok(sheet) = sht_rst {
                    let start = sheet.start();
                    let end = sheet.end();
                    match (start, end) {
                        (None, None) => todo!(),
                        (None, Some(_)) => todo!(),
                        (Some(_), None) => todo!(),
                        (Some((s1, s2)), Some((e1, e2))) => {
                            for i in s1..s2 {
                                for j in e1..e2 {
                                    let cell = sheet.get_value((i, j));
                                }
                            }
                        }
                    }
                } else {
                    return Err(FileError::SheetError);
                    //error miss, can not get XlsxError,
                }
            } else {
                err_msg = "sheet is None".to_string();
                return Err(FileError::FileNone(path.clone()));
            }
            //    let mut sheet = workbook.worksheet_range("Sheet1")
        }
        Err(FileError::FileNone(path))
    } else {
        let mut err_msg = "".to_string();
        let file = file.to_str();
        if let Some(path) = file {
            err_msg = path.to_owned();
            err_msg.push_str(" not exists");
        } else {
            let not_found = io::ErrorKind::NotFound;
            let error = io::Error::from(not_found);
            return Err(FileError::IOError(error));
        }
        Err(FileError::FileNone(path))
    }
}
//need transfrom ,need Display

//return trait obj,error miss
pub fn open_3(file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut path = format!("{}/file", env!("CARGO_MANIFEST_DIR"));
    let f = file.to_str().unwrap();
    path.push_str(f);
    let mut err_msg = "".to_string();
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let mut sheet = workbook
        .worksheet_range("sheet1")
        .ok_or(calamine::Error::Msg("Cannot find 'Sheet1'"))??;
    //double ??

    let start = sheet.start().ok_or(calamine::Error::Msg("start null"))?;
    let end = sheet.start().ok_or(calamine::Error::Msg("end null"))?;
    Ok(())
}

// 方式	优点	缺点
// 自定义错误类型	可以统一错误类型，方便上层用户对不同的错误类型采取不同的措施	需要进行各式的类型转换，较为繁琐
// Box<dyn Error>	Error可以直接透传，不需要在乎具体的类型	丢失了结构体类型信息，但是也可以通过downcast把trait object转换回具体的结构体
// fn string_error() -> Result<(), String> {
//     Ok(())
// }

// fn io_error() -> Result<(), std::io::Error> {
//     Ok(())
// }

// fn any_error() -> Result<(), Box<dyn Error>> {
//     string_error()?;
//     io_error()?;
//     Ok(())
// }

use std::{
    io,
    path::{Path, PathBuf},
};

use anyhow::Result;
use calamine::{open_workbook, Reader, Xlsx};
use thiserror::Error;

//return my Error Type
#[derive(Error, Debug)]
pub enum FileError {
    #[error("my error is file:{0} is none")]
    FileNone(String),
    #[error("my error is sheet error")]
    SheetError,
    #[error("my error is range error")]
    RangeExitError,
    #[error("my error is sheet start null")]
    RangeStartNull,
    #[error("my error is sheet end null")]
    RangeEndNull,
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
// pub fn open_2(file: &Path) -> Result<(), FileError>

pub fn open_23(file: &Path) -> Result<()> {
    let mut path = format!("{}/", env!("CARGO_MANIFEST_DIR"));

    let mut pb = PathBuf::new();
    pb.push(path);
    pb.push(file);
    println!("path buffer: {:?}", pb);
    let mut workbook: Xlsx<_> = open_workbook(pb)?;

    let mut sheet = workbook
        .worksheet_range("Sheet2")
        .ok_or(FileError::RangeExitError)??;
    let start = sheet.start().ok_or(FileError::RangeStartNull)?;
    let end = sheet.end().ok_or(FileError::RangeEndNull)?;

    Ok(())
}

//     let mut sheet = workbook.worksheet_range("Sheet1")
//         .ok_or(Error::Msg("Cannot find 'Sheet1'"))??;
// let mut sheet = workbook.worksheet_range("sheet1");
// match sheet {
//     Some(_) => println!("{}", "aaa"),
//     None => Err::<(), Error>(FileError::RangeError.into())?,
// }
//double ??

// return Err(ChannelError::SerialPortParamError(p1.to_string()).into());
// 方式	优点	缺点
// 自定义错误类型	可以统一错误类型，方便上层用户对不同的错误类型采取不同的措施	需要进行各式的类型转换，较为繁琐
// Box<dyn Error>	Error可以直接透传，不需要在乎具体的类型	丢失了结构体类型信息，但是也可以通过downcast把trait object转换回具体的结构体

//expect send msg to panic!
//type  system error handler
// fut
//   .await?
//   .process()?
//   .next()
//   .await?;
// thiserror 可以看作是定义 Error 的一个工具，它只帮你生成一些定义 Error 的代码，别的什么都不做，相当纯粹。
//而 anyhow 则为你定义好了一个 Error 类型，基本可以看作是一个 Box<dyn Error> ，同时还提供了一些如 context 等扩展功能，用起来更加无脑
// use std::error::Error;
// pub enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// 通常情况下，E 是实现 std::error::Error 的错误类型：

// pub trait Error: Debug + Display {
//     fn source(&self) -> Option<&(dyn Error + 'static)> { ... }
//     fn backtrace(&self) -> Option<&Backtrace> { ... }
//     fn description(&self) -> &str { ... }
//     fn cause(&self) -> Option<&dyn Error> { ... }
// }

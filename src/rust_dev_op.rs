use anyhow::{anyhow, Error, Ok, Result};
use dashmap::{DashMap, Map};
use once_cell::sync::OnceCell;
use std::{process::id, sync::Arc};
use thiserror::Error;

//return my Error Type
#[derive(Error, Debug)]
pub enum DevError {
    #[error("add dev failed, dev:{0}  missed")]
    DriverMiss(String),
    #[error("add dev failed, dev:{0} already exists")]
    DevExists(String),
    #[error("add dev failed, dev:{0} cerete failed")]
    DevCreateFailed(String),
    #[error("add dev failed, dev cerete failed")]
    DevDouble(),
}

pub fn get_device_manager() -> &'static DeviceManager {
    static INSTANCE: OnceCell<DeviceManager> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = DeviceManager {
            devices: DashMap::new(),
        };
        m
    })
}

pub struct Device {
    pub id: String,
    pub val: String,
}

impl Device {
    pub fn new(idp: String) -> Device {
        Device {
            id: idp,
            val: "dummy".to_string(),
        }
    }
}

pub struct DeviceManager {
    devices: dashmap::DashMap<String, Arc<Device>>,
}

impl DeviceManager {
    //only test Result, so don't care T of Result
    pub fn add_device(&self, idx: String) -> Result<()> {
        if self.devices.contains_key(&idx) {
            return Err(anyhow!(DevError::DevExists(idx)));
        } else {
            self.devices.insert(
                idx.clone(),
                Arc::new(Device {
                    id: idx.clone(),
                    val: "dummy".to_string(),
                }),
            );
        }

        // self.devices
        //     .insert(
        //         idx.clone(),
        //         Arc::new(Device {
        //             id: idx.clone(),
        //             val: "dummy".to_string(),
        //         }),
        //     )
        //     .ok_or(DevError::DevExists(idx.clone()))?;

        Ok(())
    }
}
// pub fn open_2(file: &Path) -> Result<(), FileError>

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
//anyhow
//你可以使用anyhow!宏创建一个新的错误。
//anyhow允许从任何实现了std::error::Error接口的类型创建一个anyhow::Error
//pub type Result<T, E = Error> = core::result::Result<T, E>;
// anyhow Result wrap core::Result, lagger than Result

//****** Obj-01
// //Trait std::error::Error
// pub trait Error: Debug + Display {
//     // Provided methods
//     fn source(&self) -> Option<&(dyn Error + 'static)> { ... }
//     fn description(&self) -> &str { ... }
//     fn cause(&self) -> Option<&dyn Error> { ... }
//     fn provide<'a>(&'a self, request: &mut Request<'a>) { ... }
// }

//****** Obj-02
//Result<T, E>. E is traig obj

//****** Obj-03
//anyhow::Result<T, E = anyhow::Error>.
//所有实现了std::Error trait的结构体，统一转换成它定义的anyhow::Error

// 在需要返回Result的地方，使用Result<T, anyhow::Error>或者等价的anyhow::Result<T>，就可以利用？抛出任何类型实现了std::error::Error的错误
//impl std::error::Error for MyError {} ，只有为自定义错误类型实现 Error 特征后，才能转换成相应的特征对象。

//****** Obj-03 this error
// #[derive(Error, Debug)]
// pub enum DevError {

1\mqtt
 命名。工作原理（发布订阅），遗嘱机制（client断开一会儿），服务质量等级 QoS。
mqtt特性：
一次发布，多个client消费。
topic 即用即建。
        https://developer.aliyun.com/article/786001
topic分隔符及通配符
        https://www.cnblogs.com/cxuanBlog/p/14917187.html
mqtt在协议栈中的位置
        https://www.eet-china.com/mp/a61746.html

2、rust mqtt库
1、找库    https://blessed.rs/crates
            https://lib.rs/
12-27
1、rumqttd，config.add_source().build()? 这写法就rust风格。
mqttd server start() fn
tx rx分离
2、rumqttc
sub。比较简单。
client::new() ,返回 client 和 connection 。 主要关注connection的结构，client是connect的半调路。
 client sub， 
     pub，按照道理说，只需要不断的client（requests_tx）发送即可。
但实际应用中，就bug了，没有recv线程，pub都出不来。
所以还是要理解这么一套的概念。
3、async异步 
tokio异步库
    https://rust-book.junmajinlong.com/ch100/01_understand_tokio_runtime.html
runtime future await、
tokio中的一套异步库。包括time
    https://blog.ediri.io/how-to-asyncawait-in-rust-an-introduction

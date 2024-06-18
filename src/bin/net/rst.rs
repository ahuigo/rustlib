use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, RawFd};
use std::time::Duration;
use std::{io, thread};


// curl m:9999 ;
// failure: Connection reset by peer
/*
网关可能在多种情况下关闭连接，并返回 RST 包给客户端，导致客户端出现 "Connection reset by peer" 的错误。以下是一些可能的情景：
安全策略违规：如果网关检测到流量违反了安全策略（如不允许的端口通信、协议或特定的数据模式），它可能会重置连接。
资源限制：网关可能因资源限制（如内存不足、CPU 负载过高、连接数达到上限）而无法处理新的或现有的连接，进而发送 RST 包主动关闭连接。
无效的数据包：如果收到的数据包不符合预期的格式或者细节有误，网关可能会放弃当前的连接并发送 RST 包。
超时：连接在一定时间内没有活动，或者 TCP 握手未在配置的超时时间内完成，网关可能会发送 RST 包重置连接。
路由问题：当网关不能找到前往目标地址的有效路由时，可能会通过发送 RST 包来终止连接。
负载均衡器的决策：如果网关同时也是一个负载均衡器，它可能根据设定的规则（例如健康检查失败）来重置某些连接。
防火墙拦截：网络中的防火墙可能会识别到不正常或恶意的通信尝试，而指示网关发送 RST 包以阻断这些连接。
后端服务响应：如果网关作为代理服务器或反向代理服务器，它会将客户端的请求转发到后端服务，如果后端服务关闭了连接或发送了 RST 包，网关可能也会将 RST 包转发给客户端。

 */
fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9999")?;
    println!("Listening on 127.0.0.1:9999");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        };
    }

    Ok(())
}

fn set_linger(socket_fd: RawFd, linger_duration: Option<Duration>) -> io::Result<()> {
    use libc::{linger, setsockopt, SOL_SOCKET, SO_LINGER};
    use std::mem;

    let linger = linger_duration.map_or(libc::linger { l_onoff: 0, l_linger: 0 }, |duration| {
        libc::linger {
            l_onoff: 1,
            l_linger: duration.as_secs() as _,
        }
    });

    // 尝试在套接字上设置 SO_LINGER 选项，并将延迟时间设置为 0。这通常会告诉操作系统，在调用 close 之后立即放弃任何未发送的数据并发送一个 RST 包给对方
    let ret = unsafe {
        setsockopt(
            socket_fd,
            SOL_SOCKET,
            SO_LINGER,
            &linger as *const libc::linger as *const libc::c_void,
            mem::size_of::<linger>() as libc::socklen_t,
        )
    };

    if ret == 0 {
        println!("SO_LINGER set successfully");
        Ok(())
    } else {
        println!("Failed to set SO_LINGER");
        Err(io::Error::last_os_error())
    }
}
fn handle_client(mut stream: TcpStream) {
    let socket_fd = stream.as_raw_fd();

    // 读取数据 (这里只是示例，并没有实际处理)
    let mut buf = [0; 1024];
    let nbytes = stream.read(&mut buf).unwrap();
    println!("Received data: {:?}", &buf[..nbytes]);

    // 设置 SO_LINGER 使得关闭连接时立即发送 RST 包
    set_linger(socket_fd, Some(Duration::from_secs(0))).expect("Failed to set SO_LINGER");

    // 关闭 TCP 流，这里可能触发发送 RST 包
    drop(stream);
}
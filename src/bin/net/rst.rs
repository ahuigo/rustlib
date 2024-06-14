use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, RawFd};
use std::time::Duration;
use std::{io, thread};


// echo m:9999 ;
// failure: Connection reset by peer
fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9999")?;
    println!("Listening on 127.0.0.1:9999");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 在新线程中处理连接
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                // 输出连接错误信息
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
        Ok(())
    } else {
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
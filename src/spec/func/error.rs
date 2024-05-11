/*
rust 有两种error：
1. 不可恢复panic: 例如数组越界，除数为0　或主动panic
2. 可恢复的error: Result<T,E>, 例如文件不存在，网络连接失败等
*/
mod err_panic {
    #[test]
    // 1. panic默认栈展开: unwinding，可以被上层catch
    // 如果想打印栈：RUST_BACKTRACE=1 cargo test -- spec::func::error::err_panic::panic_unwinding
    fn panic_unwinding() {
        //panic!("crash and burn");
        let v = vec![1, 2, 3];
        v[99];
    }

    /* 2. 改成panic 直接终止: abort
    配置修改 Cargo.toml 文件，实现在 release 模式下遇到 panic 直接终止：
    #![allow(unused)]
    fn main() {
        [profile.release]
        panic = 'abort'
    }
    */

    #[test]
    /*
    当调用 panic! 宏时，它会
    1. 格式化 panic 信息，然后使用该信息作为参数，调用 std::panic::panic_any() 函数
    2. panic_any 会检查应用是否使用了 panic hook: -- 如果使用了，该 hook 函数就会被调用（hook 是一个钩子函数，是外部代码设置的，用于在
       panic 触发时，执行外部代码所需的功能）
    3. 当 hook 函数返回后，当前的线程就开始进行栈展开：-- 从 panic_any 开始，如果寄存器或者栈因为某些原因信息错乱了，那很可能该展开会发生
       异常，最终线程会直接停止，展开也无法继续进行
    4. 展开的过程是一帧一帧的去回溯整个栈，在展开过程中，你可能会遇到被用户标记为 catching 的帧（通过
       std::panic::catch_unwind() 函数标记）
       4.1 此时用户提供的 catch 函数会被调用，展开也随之停止：
       4.2 当然，如果 catch 选择在内部调用 std::panic::resume_unwind() 函数，则展开还会继续。
        */
    fn panic_inner() {
        panic!("crash and burn");
    }
}

#[test]
fn err_result() {
    use std::net::IpAddr;
    // let _home: IpAddr = "127.0.0.1".parse().unwrap();
    let rst = "127.0.0.1".parse(); // Result<F, F::Err>
    let _home: IpAddr = rst.unwrap(); //失败时直接unwrap_failed:panic
}

/**
golang 避免多次if err != nil　方法1(缺点是要临时定义write和err)：
    var err error
    write := func(buf []byte) {
        if err != nil {
            return
        }
        _, err = w.Write(buf)
    }
    write(p0[a:b])
    write(p1[c:d])
    write(p2[e:f])
    // and so on
    if err != nil {
        return err
    }

方法2: 缺点是只适合多次访问同一对象的方法：

    func (ew *errWriter) write(buf []byte) {
        if ew.err != nil {
            return
        }
        _, ew.err = ew.w.Write(buf)
    }
    ew := &errWriter{w: fd}
    ew.write(p0[a:b])
    ew.write(p1[c:d])
    ew.write(p2[e:f])
    // and so on
    if ew.err != nil {
        return ew.err
    }

 **/
#[test] // write_all()? 如果遇到错误，直接返回错误
fn if_err() {
    use std::io::Write;
    #[allow(unused)]
    fn write_all(
        fd: &mut dyn Write,
        p0: &[u8],
        a: usize,
        b: usize,
        p1: &[u8],
        c: usize,
        d: usize,
        p2: &[u8],
        e: usize,
        f: usize,
    ) -> std::io::Result<()> {
        fd.write_all(&p0[a..b])?;
        fd.write_all(&p1[c..d])?;
        fd.write_all(&p2[e..f])?;
        Ok(())
    }
}

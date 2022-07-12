use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};

use nix::sys::socket::{
    accept, bind, listen, recv, send, socket, AddressFamily, MsgFlags, SockFlag, SockType, UnixAddr,
};

use nix::errno::Errno;

use std::os::unix::io::RawFd;

use std::process::{Command, Stdio};

use serde_json::{json, Value};

pub fn connect() -> (RawFd, RawFd) {
    let main_addr = Alphanumeric.sample_string(&mut thread_rng(), 50);
    let event_addr = Alphanumeric.sample_string(&mut thread_rng(), 50);

    let main_sock_addr = UnixAddr::new_abstract(&main_addr.as_bytes()).unwrap();
    let event_sock_addr = UnixAddr::new_abstract(&event_addr.as_bytes()).unwrap();

    let main_sock = socket(
        AddressFamily::Unix,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .unwrap();

    let event_sock = socket(
        AddressFamily::Unix,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .unwrap();

    //TODO: Handle Error
    for i in 0..=10 {
        if i == 10 {
            println!("Error Establishing connection with socket");
        }
        match bind(main_sock, &main_sock_addr) {
            Ok(_) => break,
            Err(err) => {
                if let Errno::EBADF | Errno::EINVAL | Errno::ENOTSOCK = err {
                    panic!("Failed creating main Socket");
                }
            }
        }
    }

    for i in 0..=10 {
        if i == 10 {
            println!("Error Establishing connection with socket");
        }

        match bind(event_sock, &event_sock_addr) {
            Ok(_) => break,
            Err(err) => {
                if let Errno::EBADF | Errno::EINVAL | Errno::ENOTSOCK = err {
                    panic!("Failed creating event Socket");
                }
            }
        }
    }

    listen(main_sock, 1).unwrap();
    listen(event_sock, 1).unwrap();

    Command::new("am")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args([
            "broadcast",
            "-n",
            "com.termux.gui/.GUIReceiver",
            "--es",
            "mainSocket",
            &main_addr,
            "--es",
            "eventSocket",
            &event_addr,
        ])
        .output()
        .unwrap();

    let main = accept(main_sock).unwrap();
    let event = accept(event_sock).unwrap();

    let mut protocol = [1u8];
    while send(main, &protocol, MsgFlags::empty()).unwrap() == 0 {}
    protocol = [0u8];
    while recv(main, &mut protocol, MsgFlags::empty()).unwrap() == 0 {}
    (main, event)
}

pub fn transmit_buffer(fd: RawFd, msg: &[u8]) {
    let mut len = msg.len();
    let mut start = 0;
    while len > 0 {
        let ret = send(fd, &msg[start..], MsgFlags::empty()).unwrap();
        len = len.saturating_sub(ret);
        start += ret;
    }
}

pub fn recv_msg(fd: RawFd) -> Value {
    let mut size = [0u8; 4];
    let mut togo = 4usize;
    let mut start = 0;

    while togo > 0 {
        let ret = recv(fd, &mut size[start..], MsgFlags::empty()).unwrap();
        togo = togo.saturating_sub(ret);
        start += ret;
    }

    togo = u32::from_be_bytes(size) as usize;
    let mut msg = vec![0u8; 1024 * 64];
    while togo > 0 {
        let ret = recv(fd, &mut msg[start..], MsgFlags::empty()).unwrap();
        togo = togo.saturating_sub(ret);
        start += ret;
    }

    let msg: Vec<u8> = msg.iter().map(|&v| v).filter(|&val| val != b'\0').collect();
    match serde_json::from_slice(&msg) {
        Ok(val) => val,
        Err(_) => json!(null),
    }
}

pub fn send_msg(fd: RawFd, msg: Value) {
    let msg = msg.to_string();
    let msg_bytes = msg.as_bytes();
    let msg_len = u32::to_be_bytes(msg_bytes.len() as u32);

    transmit_buffer(fd, &msg_len);
    transmit_buffer(fd, &msg_bytes);
}

pub fn send_recv_msg(fd: RawFd, msg: Value) -> Value {
    send_msg(fd, msg);
    recv_msg(fd)
}

pub fn construct_message(method: &str, args: &Value) -> Value {
    json!({"method": method, "params": args})
}

use std::{time, thread};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use nix::unistd;
use nix::unistd::ForkResult;

fn main() {
    let s = Arc::new(Mutex::new(String::new()));
    let s1 = s.clone();

    /* スレッド: 5秒間ロックを持ったままにしたあとwittenと書き込む */
    thread::spawn(move || { 
        let mut s = s1.lock().unwrap();
        thread::sleep(time::Duration::from_secs(5));
        s.push_str("written");
    });

    let t = Instant::now();
    /* フォークして親子が1秒ごとに文字列を読む試みをする */
    match unsafe{unistd::fork()} {
        Ok(child_or_parent) => { //親も子もここにくる
            let s2 = s.clone();
            loop {
                thread::sleep(time::Duration::from_secs(1));
                let s = s2.lock().unwrap(); //ロックの取得
                println!("{}秒後 {:?}: {:?}", t.elapsed().as_secs(), child_or_parent, s);
            }
        },
        Err(_) => panic!("!"),
    }
}

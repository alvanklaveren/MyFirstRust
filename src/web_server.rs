#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub mod single_threaded {

    use std::fs;
    use std::io::{self,prelude::*,BufRead};
    use std::net::{TcpStream, TcpListener };
    use std::collections::HashMap;
    use std::ptr::null;

    enum RedirectPage {
        Root,
        Home,
        _404,
    }

    impl RedirectPage {
        fn filename(&self) -> &str {
            match *self {
                RedirectPage::Root => "./src/index.html",
                RedirectPage::Home => "./src/index_alt.html",
                RedirectPage::_404 => "./src/404.html",
            }
        }

        fn status_line(&self) -> &str {
            match *self {
                RedirectPage::Root => "HTTP/1.1 200 OK\r\n\r\n",
                RedirectPage::Home => "HTTP/1.1 200 OK\r\n\r\n",
                RedirectPage::_404 => "HTTP/1.1 404 NOT FOUND\r\n\r\n",
            }
        }

        fn page_id(&self) -> &str {
            match *self {
                RedirectPage::Root => "GET / HTTP/1.1\r\n",
                RedirectPage::Home => "GET /home HTTP/1.1\r\n",
                RedirectPage::_404 => "",
            }
        }
    }

    pub fn listen(){
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            handle_connection(stream);
        }
    }

    fn handle_connection(mut stream: TcpStream) {

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        //println!("{}",String::from_utf8_lossy(&buffer[..]));

        let mut pages = Vec::new();
        pages.push(RedirectPage::Root);
        pages.push(RedirectPage::Home);

        let find_page = pages.iter().find(|p| buffer.starts_with(p.page_id().as_bytes()));
        let page = match find_page {
            Some(p) => p,
            None => &RedirectPage::_404,
        };

        let contents = fs::read_to_string(page.filename()).unwrap();
        let response = format!("{}{}", page.status_line(), contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

pub mod multi_threaded {


}
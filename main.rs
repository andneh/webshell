use std::net::{TcpListener, TcpStream};
use std::io::{Read, BufReader, BufRead, BufWriter, Write};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

        for result in reader.lines(){
            match result {
                Ok(line) => {
                    println!("{}", line);
                    let a  = "HTTP/1.1 404 Not Found
Content-Type: text/html; charset=UTF-8
Referrer-Policy: no-referrer
Content-Length: 1561
Date: Mon, 24 Apr 2023 19:50:23 GMT

<!DOCTYPE html>
<html lang=en>
  <meta charset=utf-8>
  <meta name=viewport content=\"initial-scale=1, minimum-scale=1, width=device-width\">
  <title>Error 404 (Not Found)!!1</title>
  <style>
    *{margin:0;padding:0}html,code{font:15px/22px arial,sans-serif}html{background:#fff;color:#222;padding:15px}body{margin:7% auto 0;max-width:390px;min-height:180px;padding:30px 0 15px}* > body{background:url(//www.google.com/images/errors/robot.png) 100% 5px no-repeat;padding-right:205px}p{margin:11px 0 22px;overflow:hidden}ins{color:#777;text-decoration:none}a img{border:0}@media screen and (max-width:772px){body{background:none;margin-top:0;max-width:none;padding-right:0}}#logo{background:url(//www.google.com/images/branding/googlelogo/1x/googlelogo_color_150x54dp.png) no-repeat;margin-left:-5px}@media only screen and (min-resolution:192dpi){#logo{background:url(//www.google.com/images/branding/googlelogo/2x/googlelogo_color_150x54dp.png) no-repeat 0% 0%/100% 100%;-moz-border-image:url(//www.google.com/images/branding/googlelogo/2x/googlelogo_color_150x54dp.png) 0}}@media only screen and (-webkit-min-device-pixel-ratio:2){#logo{background:url(//www.google.com/images/branding/googlelogo/2x/googlelogo_color_150x54dp.png) no-repeat;-webkit-background-size:100% 100%}}#logo{display:inline-block;height:54px;width:150px}
  </style>
  <a href=//www.google.com/><span id=logo aria-label=Google></span></a>
  <p><b>404.</b> <ins>That’s an error.</ins>
  <p>The requested URL <code>/</code> was not found on this server.  <ins>That’s all we know.</ins>
";
                    write!(writer,"{a}\n").unwrap();
                    writer.flush();
                    if line == "close"{
                        break;
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }

        //
        // let mut buffer = [0; 1024];

}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;

    use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddr};

    #[test]
    fn socket() {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        assert_eq!("127.0.0.1:8080".parse(), Ok(socket));
        assert_eq!(socket.port(), 8080);
        assert_eq!(socket.is_ipv4(), true);

    }

    #[test]
    #[should_panic(expected = "uh")]
    fn panic(){
        panic!("uh");
    }

    #[test]
    #[should_panic]
    fn zero(){
        let a = 1/1;
        assert_eq!(a, 1);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(1+2, 3);
    }
}
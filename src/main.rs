use clap::{Arg, Command};
use csv::{ReaderBuilder, Trim};
use std::net::TcpStream;
use std::io::Read;
use ssh2::Session;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let matches = Command::new("magnetosphere")
                    .arg(Arg::with_name("config"))
                    .get_matches();

    let configpath = Path::new(matches.get_one::<String>("config").unwrap());
    let csv = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'=')
        .comment(Some(b'#'))
        .trim(Trim::All)
        .from_path(configpath).unwrap()
        .into_records();
    let mut config = HashMap::new();

    for record in csv {
        println!("{} = {}", &record.as_ref().unwrap()[0], &record.as_ref().unwrap()[1]);
        config.insert(String::from(&record.as_ref().unwrap()[0]),
            String::from(&record.as_ref().unwrap()[1]));
    }

    let tcp = TcpStream::connect(format!("{}:{}", 
                                         config.get("ip").unwrap(),
                                         config.get("port").unwrap()))
                                 .unwrap();
    let mut sess = Session::new().unwrap();

    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(config.get("username").unwrap(),
                              None,
                              Path::new(config.get("privatekey").unwrap()),
                              None)
        .unwrap();
    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut output = Vec::new();
    channel.read_to_end(&mut output).unwrap();

    println!("{}", String::from_utf8(output).unwrap());

}

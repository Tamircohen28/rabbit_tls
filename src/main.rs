use amiquip::{Auth, Connection, ConnectionOptions, ConnectionTuning, Exchange, Publish, Result};
use log::LevelFilter;
use mio::net::TcpStream;
use native_tls::{Certificate, TlsConnector};
use simple_logger::SimpleLogger;
use std::{
    fs,
    net::{SocketAddr, SocketAddrV4},
    path::Path,
};

struct RabbitTLS<P, U, S> {
    certificate: P,
    url: U,
    addr: S,
}

impl<P, U, S> RabbitTLS<P, U, S>
where
    P: AsRef<Path>,
    U: AsRef<str>,
    S: Into<SocketAddr>,
{
    fn new(secure: bool, auth: Auth, host: &str, port: usize, ca_file: P) -> Self {}
}
//     fn get_url(&self) -> String {
//         format!(
//             "{protocol}://{username}:{password}@{host}:{port}",
//             protocol = if self.secure { "amqps" } else { "amqp" },
//             username = self.username,
//             password = self.password,
//             host = self.host,
//             port = self.port,
//         )
//     }

//     fn get_stream(&self) -> TcpStream {
//         let address = format!("{host}:{port}", host = self.host, port = self.port,);
//         TcpStream::connect(&address.parse().unwrap()).unwrap()
//     }

//     fn get_cert(&self) -> Certificate {
//         let cert_byte_vec = fs::read(self.cafile.clone()).expect("failed to load cert");
//         Certificate::from_pem(&cert_byte_vec).expect("failed to parse cert")
//     }
// }

fn main() {
    // SimpleLogger::new()
    //     .with_level(LevelFilter::Debug)
    //     .init()
    //     .unwrap();

    // let rabbit = Rabbit {
    //     host: "localhost".to_owned(),
    //     port: 5671,
    //     username: "guest".to_owned(),
    //     password: "guest".to_owned(),
    //     secure: true,
    //     cafile: r"C:\Users\Tamir Cohen\Desktop\rabbit_tls\certificate\client_certificate.pem"
    //         .to_owned(),
    // };
    // let connector = TlsConnector::builder()
    //     .add_root_certificate(rabbit.get_cert())
    //     .build()
    //     .unwrap();

    // // Open connection.
    // let mut connection = Connection::open_tls_stream::<Auth, TlsConnector, TcpStream>(
    //     connector,
    //     &rabbit.host,
    //     rabbit.get_stream(),
    //     ConnectionOptions::default(),
    //     ConnectionTuning::default(),
    // )
    // .expect("Connection failed");

    // // Open a channel - None says let the library choose the channel ID.
    // let channel = connection
    //     .open_channel(None)
    //     .expect("Opening channel failed");

    // // Get a handle to the direct exchange on our channel.
    // let exchange = Exchange::direct(&channel);

    // // Publish a message to the "hello" queue.
    // exchange
    //     .publish(Publish::new(b"hello there", "hello"))
    //     .expect("Publish had failed");

    // connection.close().expect("Failed closing connection");
}

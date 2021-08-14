use amiquip::{
    Auth, Connection, ConnectionOptions, ConnectionTuning, ExchangeDeclareOptions, ExchangeType,
    FieldTable, Publish, QueueDeclareOptions,
};
use log::{info, LevelFilter};
use mio::net::TcpStream;
use native_tls::{Certificate, Identity, TlsConnector};
use simple_logger::SimpleLogger;
use std::{fs, path::Path};

// certificate 'Common Name' was set on cration, do not touch this!
const C_CA_CN: &str = "rabbit_tls_srv";
const C_RABBIT_IP: &str = "127.0.0.1";
const C_RABBIT_PORT: u16 = 5671;
const C_CERT_DIR: &str = r"certificate";

// Rabbit configuration struct
#[derive(Debug)]
struct Rabbit<P: AsRef<Path>> {
    username: String,
    password: String,
    host: String,
    port: u16,
    client_cert: P,
    client_p12: P,
}

impl<P: AsRef<Path>> Rabbit<P> {
    fn new(
        username: &str,
        password: &str,
        host: &str,
        port: u16,
        client_cert: P,
        client_p12: P,
    ) -> Self {
        Self {
            username: username.to_owned(),
            password: password.to_owned(),
            host: host.to_owned(),
            port,
            client_cert,
            client_p12,
        }
    }

    // get connection options
    fn get_conn_options(&self) -> ConnectionOptions<Auth> {
        ConnectionOptions::default().auth(Auth::Plain {
            username: self.username.clone(),
            password: self.password.clone(),
        })
    }

    // get TLS connector configurad with needed certifications
    fn get_connector(&self) -> TlsConnector {
        let cert_byte_vec = fs::read(&self.client_cert).expect("failed to load cert");
        let cert = Certificate::from_pem(&cert_byte_vec).expect("failed to parse cert");
        let identity_byte_vec = fs::read(&self.client_p12).expect("failed to load client_p12");
        let identity =
            Identity::from_pkcs12(&identity_byte_vec, "").expect("failed to parse client_p12");

        TlsConnector::builder()
            .identity(identity)
            .add_root_certificate(cert)
            .build()
            .unwrap()
    }

    // open tcp stream on the rabbit port
    fn get_stream(&self) -> TcpStream {
        TcpStream::connect(&format!("{}:{}", self.host, self.port).parse().unwrap()).unwrap()
    }
}

fn client_demo(index: usize) {
    let rabbit = Rabbit::new(
        "guest",
        "guest",
        C_RABBIT_IP,
        C_RABBIT_PORT,
        Path::new(C_CERT_DIR).join(format!("client_{}_certificate.pem", index)),
        Path::new(C_CERT_DIR).join(format!("client_{}_key.p12", index)),
    );

    // client-side TLS connection
    let connector = rabbit.get_connector();
    let stream = rabbit.get_stream();
    info!("Connected to TCP stream");

    // Open connection.
    let mut connection = Connection::open_tls_stream::<Auth, TlsConnector, TcpStream>(
        connector,
        C_CA_CN,
        stream,
        rabbit.get_conn_options(),
        ConnectionTuning::default(),
    )
    .expect("Rabbit Connection failed");
    info!("TLS connection successful");

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection
        .open_channel(None)
        .expect("Opening channel failed");

    // Get a handle to the direct exchange on our channel.
    let exchange = channel
        .exchange_declare(
            ExchangeType::Direct,
            "tls_exchange".to_owned(),
            ExchangeDeclareOptions::default(),
        )
        .expect("Failed declaring exchange");

    channel
        .queue_declare("tls_queue", QueueDeclareOptions::default())
        .expect("Failed declaring queue");

    channel
        .queue_bind("tls_queue", "tls_exchange", "hello", FieldTable::default())
        .expect("Failed binding queue to exchange");

    // publish 10 messages
    for _ in 0..10 {
        exchange
            .publish(Publish::new(
                format!("hello there from client {}", index).as_bytes(),
                "hello",
            ))
            .expect("Publish had failed");
    }
    info!("All messages sent seccessfuly");
}

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    client_demo(1);
    client_demo(2);
    client_demo(3);
}

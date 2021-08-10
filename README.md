# Rabbit TLS
using tls communication for rabbit service.  
Enables Encrypted communication with peer verification, reed more at [Rabbit TLS](https://www.rabbitmq.com/ssl.html).

## Requirements
### 1. Rust develop env
[Download Rust toolchain](https://www.rust-lang.org/tools/install)  
On windows you may need to install the [Visual Studio C++ Build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

### 2. Docker
#### **windows**
Win 10 docker installation [guide](https://www.ntweekly.com/2018/04/28/install-docker-windows-10/)

#### **Linux**
```shell
sudo apt install docker.io
```

### 3. Add CA
you need to add the generated CA certificate to the considered trusted certificates.
#### **windows**
On Windows trusted certificates are managed using tools such as [certmgr](https://docs.microsoft.com/en-us/dotnet/framework/tools/certmgr-exe-certificate-manager-tool).  
The Certificate Manager is automatically installed with Visual Studio.
```shell
cd "C:\Program Files (x86)\Windows Kits\10\bin\10.0. 16299.0\x64"
certmgr -add -all "<repo path>\certificate\ca_certificate.pem" -s Root
```
#### **Linux**
```shell
# Linux with Mono
certmgr -add -c Trust <repo path>/certificate/ca_certificate.pem
```

## Rabbit server setup
```shell
make run-docker
```
This command runs the Rabbit server with TLS support, also enables managment interface on port 8080.  
You can then go to http://localhost:8080 with the default username and password of  ```guest/guest```.

For more capabilities see [rabbitmq on dockerhub](https://hub.docker.com/_/rabbitmq)

## Using tls-gen's Basic Profile
Below is an example that generates a CA and uses it to produce two certificate/key pairs, one for the server and another for clients. This is the setup for the certificate directory.  
**Do not** genarate certificate with password, the client can't read them.
```shell
git clone https://github.com/michaelklishin/tls-gen tls-gen
cd tls-gen/basic
make PYTHON=python
```

## Use OpenSSL Tools to Test TLS Connection
```shell
openssl s_client -connect localhost:5671 -cert client_certificate.pem -key client_key.pem -CAfile ca_certificate.pem
```
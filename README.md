# Rabbit TLS
using tls communication for rabbit service

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

## Rabbit server setup
```shell
make run-docker
```
This command runs the Rabbit server management plugin, and enables managment interface on port 8080. You can then go to http://localhost:8080 with the default username and password of  ```guest/guest```.

For more capabilities see [rabbitmq on dockerhub](https://hub.docker.com/_/rabbitmq)




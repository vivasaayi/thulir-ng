### Install Git
```sh
sudo yum install git
```

### Clone Repo
```sh
https://github.com/vivasaayi/thulir-ng.git
```

### Install Rust
```sh
sudo su
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install Dependencies
```sh
sudo yum install gcc-c++
sudo yum install -y openssl-devel
```

### Install NVM
```sh
sudo curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash

nvm install 20

# Have to use an older version because of GLIBC dependencies
nvm install 16
```

### Compile Web
```sh
nvm use node 16
cd thulir-ng/web/

npm install
npm run build
```

### Compile Rust
```sh
export ROCKET_ADDRESS=0.0.0.0
export ROCKET_PORT=80
cargo build
cargo run
```
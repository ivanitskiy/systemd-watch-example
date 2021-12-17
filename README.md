This is an example how to work systemd logs with rust on linux


Install  the following packages to allow `systemd` crate able to load libsystemd:

    sudo apt install pkg-config libsystemd-dev


Refer to [rust-systemd](https://github.com/jmesmon/rust-systemd) documentation for more details.

How to test:

you can run the program via `cargo run`, open another terminal and watch events. e.g. run `docker ps`

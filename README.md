# Hosts File Swapper

This Rust program allows you to swap the /etc/hosts file on your machine with a prepared hosts file. The prepared hosts file can be used to redirect the URLs of popular social media sites to localhost, or to block access to them altogether.

### Prerequisites

- Rust 1.46 or higher

### Usage:

1. Clone this repository:
```bash
    git clone https://github.com/user/hosts-file-swapper
    cd hosts-file-swapper
```
    
2. Edit the main.rs file to specify the path to the prepared hosts file:

```rust
let prepared_hosts_path = Path::new("/path/to/prepared/hosts/file");
```
3. Build and run the program:
```bash
cargo build --release
sudo target/release/hosts-file-swapper
```

Note that modifying the `/etc/hosts` file requires administrative privileges, so you may need to run the program with sudo or as the root user.

### Example

Here is an example of an `/etc/hosts` file that redirects the URLs of some popular social media sites to localhost:

```
127.0.0.1 www.facebook.com
127.0.0.1 www.instagram.com
127.0.0.1 www.twitter.com
127.0.0.1 www.linkedin.com
127.0.0.1 www.pinterest.com
127.0.0.1 www.reddit.com
127.0.0.1 www.youtube.com
```

### Testing

To run the unit tests, use the following command:

```bash
cargo test
```

### License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/git/git-scm.com/blob/main/MIT-LICENSE.txt) file for details.
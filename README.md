# Hosts File Swapper

This Rust program allows you to swap the /etc/hosts file on your machine with a prepared hosts file. The prepared hosts file can be used to redirect the URLs of popular social media sites to localhost, or to block access to them altogether.

### Prerequisites

- Rust 1.64 or higher

### Usage:

1. Clone this repository:
```bash
    git clone https://github.com/kubabialy/website_blocker
    cd website-blocker
```
2. Build and run the program:
```bash
cargo build --release
```

3. Run it with either prepared `./my-hosts` file or provide path to one of your choice using `-s` or `--source-file` param.

```bash
// This one uses default value -> ./my-hosts
sudo target/release/website-blocker
sudo target/release/website-blocker -s path/to/my/file
sudo target/release/website-blocker --source-file path/to/my/file
```

Note that modifying the `/etc/hosts` file requires administrative privileges, so you may need to run the program with sudo or as the root user.

In case of need to rollback to previous config this program prepares a backup file with the same name affixed with `-backup`. So in case of `hosts` file you should expect `hosts-backup` file to be in your `etc` directory.

4. In case revert of the change is needed:
```bash
sudo target/release/website-blocker -r
sudo target/release/website-blocker --reverse
```

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

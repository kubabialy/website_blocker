use std::fs;
use std::io;
use std::path::Path;

fn swap_hosts_file(prepared_path: &Path, current_path: &Path) -> io::Result<()> {
    // Set the path to the prepared hosts file
    let prepared_hosts_path = Path::new(&prepared_path);

    // Read the prepared hosts file into a string
    let prepared_hosts = fs::read_to_string(prepared_hosts_path)?;

    // Set the path to the current hosts file
    let current_hosts_path = Path::new(&current_path);

    // Create a backup file
    prepare_backup_file(&String::from(current_path.to_str().unwrap()));

    // Write the prepared hosts file to the current hosts file
    fs::write(current_hosts_path, prepared_hosts).expect("File couldn't be replaced");

    Ok(())
}

fn prepare_backup_file(current_path: &String) {
    let mut backup_path = String::from(current_path);
    let file_affix = "-backup";

    backup_path.push_str(file_affix);
    fs::write(backup_path, current_path).expect("Could not create a backup file");
}

/**
Remaining stuff to add:
- Backup [ ]
- Add file parameter [ ]
*/
fn main() -> io::Result<()> {
    swap_hosts_file(Path::new("./my-hosts"), Path::new("/etc/hosts"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_backup_creation() {
        let dir = TempDir::new().unwrap();

        // Set the path to prepared file
        let prepared_file = dir.path().join("prepared_file");
        let prepared_file_path = prepared_file.as_path();

        // Write some content to the prepared file
        let content = "Foo bar baz";
        fs::write(&prepared_file, content).unwrap();

        // Prepare the backup file
        prepare_backup_file(&prepared_file_path.to_str().unwrap().to_string());
        // Read the content of the current hosts file
        let swapped_file = fs::read_to_string(prepared_file_path).unwrap();

        // Ensure that the current hosts file has the content of the prepared hosts file
        assert_eq!(swapped_file, content.to_string());
    }

    #[test]
    fn test_swap_hosts_file() {
        // Create a temporary directory to store the test files
        let dir = TempDir::new().unwrap();

        // Set the paths to the prepared and current hosts files
        let prepared_hosts_path = dir.path().join("prepared_hosts");
        let current_hosts_path = dir.path().join("current_hosts");

        // Write some content to the prepared hosts file
        let prepared_hosts = "127.0.0.1 example.com";
        fs::write(&prepared_hosts_path, prepared_hosts).unwrap();

        // Write some different content to the current hosts file
        let current_hosts = "127.0.0.1 some-other-example.org";
        fs::write(&current_hosts_path, current_hosts).unwrap();

        // Set the paths for the test
        let test_prepared_hosts_path = prepared_hosts_path.as_path();
        let test_current_hosts_path = current_hosts_path.as_path();

        // Swap the hosts files
        swap_hosts_file(test_prepared_hosts_path, test_current_hosts_path).unwrap();

        // Read the content of the current hosts file
        let swapped_hosts = fs::read_to_string(current_hosts_path).unwrap();

        // Ensure that the current hosts file has the content of the prepared hosts file
        assert_eq!(swapped_hosts, prepared_hosts);
    }
}


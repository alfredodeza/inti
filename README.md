# inti

This project is purely exploratory and not meant for production

Easy Rust-based configuration management for setting up dotfiles, packages, and getting things configured.


Run it by default in a directory where your dotfiles are, it should work like `Makefile`, no need for any arguments.

Similar to Ansible or GitHub Actions with a reasonable `inti.yaml` file needed. By default, it will look into the directory and start doing the work, either installing packages or placing files around. Fully idempotent with the ability to check packages before trying to install them again.

```yaml
# inti.yaml - config embedded optionally
config:
  parallel: false
  verbose: true

tasks:
  - name: "Install Vim"
    apt:
      update: true  # Will skip if update has ran today
      package: vim

  - name: "Setup project"
    file:
      path: src
      state: directory

  - name: Install all other system packages
    apt:
      update: true  # Will skip if update has ran today
      packages:
        - build-essentials
        - curl
        - wget

  - name: Make sure that update is done on repos
    apt:
      update: force # forces an update regardless of when it was last done

  - name: Install Rust
    command: curl -sSf https://sh.rustup.rs | sh -s -- -y
```

# Rust Gist Magic

A command-line application written in Rust that allows you to perform CRUD operations for GitHub Gist for a file directly from the CLI using API calls to the Github API.


### Commands

---

#### List

```
Usage: gist_magic list [OPTIONS]

Description: List all gists associated with the provided username. If no username is provided, it lists all gists available to the authenticated user.

Options:
  -o, --username <USERNAME>    Username of the owner of gists to list
```

---

#### Create

```
Usage: gist_magic create [OPTIONS]

Description: Create a new gist either from a file or by manually entering content.

Options:
  -f, --filepath <FILEPATH>    File to create a gist from
  -s, --start <START>          Starting line of the file to create a gist from
  -e, --end <END>              Ending line of the file to create a gist from
```

---

#### Edit

```
Usage: gist_magic edit [OPTIONS]

Description: Edit an existing gist.

Options:
  -g, --gistid <GISTID>    ID of the gist to edit
```

---

#### Delete

```
Usage: gist_magic delete [OPTIONS]

Description: Delete an existing gist.

Options:
  -g, --gistid <GISTID>    ID of the gist to be deleted
```

---

#### View

```
Usage: gist_magic view [OPTIONS]

Description: View details of a specific gist.

Options:
  -g, --gistid <GISTID>    ID of the gist to view
```

---

#### Star

```
Usage: gist_magic star [OPTIONS]

Description: Star a specific gist.

Options:
  -g, --gistid <GISTID>    ID of the gist to star
```

---

#### Unstar

```
Usage: gist_magic unstar [OPTIONS]

Description: Unstar a specific gist.

Options:
  -g, --gistid <GISTID>    ID of the gist to unstar
```

---


#### GitHub OAuth Token Configuration

To configure your GitHub OAuth token with Gist Magic, follow these steps:

1. **Access GitHub OAuth Token**: First, you need to obtain your GitHub OAuth token from the GitHub Developer Settings. This token acts as your authentication key when interacting with GitHub's APIs.

2. **Setting the OAuth Token**:
   - If you already have your GitHub OAuth token, you can set it directly in the Gist Magic configuration file.
   - If you don't have a token or haven't set it yet, Gist Magic prompts you to input your GitHub OAuth token the first time you run the tool.

3. **Saving the Configuration**:
   - After entering your GitHub OAuth token, Gist Magic automatically saves it to a configuration file named `.gist_magic.json` located in your home directory.

4. **Persisting Configuration**:
   - Gist Magic persists your GitHub OAuth token in the configuration file so that you don't need to re-enter it each time you use the tool.

#### Accessing the Configuration File

You can manually access and modify the configuration file if needed. The file is located at the following path:

- **Location**: `$HOME/.gist_magic.json`
- **Content**: The configuration file contains your GitHub OAuth token in JSON format.

#### Updating the OAuth Token

If you ever need to update or change your GitHub OAuth token, you can do so by directly editing the configuration file or by running the Gist Magic tool, which will prompt you to enter the new token.

By configuring your GitHub OAuth token with Gist Magic, you ensure seamless integration with GitHub's services, allowing you to manage your gists effortlessly from the command line.


### Additional Notes
- **Authorization:** All commands except `list` require a GitHub token for authorization.
- **Error Handling:** Errors are logged and printed to the console for user visibility.
- **Output:** The output for commands like `list` and `view` includes gist details such as ID, description, owner, and gist files (if applicable).


## Installation

1. Clone the repository:

    ```shell
    git clone https://github.com/Akash-Singh04/rust-gist-magic.git
    ```

2. Build the application:

    ```shell
    cd rust-gist-magic
    cargo build --release
    ```

3. Run the application:

    ```shell
    cargo run --release -- <file-name> <file-content>
    ```


### License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

### Contributing Guide

We welcome contributions from the community to improve and expand Gist Magic. Here's how you can contribute:

1. Fork the repository to your GitHub account.
2. Clone the forked repository to your local machine.
3. Create a new branch for your feature or bug fix.
4. Make your changes and ensure they adhere to the project's coding style and guidelines.
5. Write tests for your changes if applicable.
6. Commit your changes with descriptive commit messages.
7. Push your changes to your fork on GitHub.
8. Open a pull request (PR) against the main repository's `main` branch.

Before opening a PR, ensure that you've tested your changes thoroughly. If your changes affect the project's behavior, update the documentation accordingly.

### Additional Information

For any questions or discussions related to this project, feel free to open an issue on the repository. We appreciate your interest and contributions!

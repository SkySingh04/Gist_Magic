# Rust Gist Magic

A command-line application written in Rust that allows you to create a GitHub Gist for a file directly from the CLI.


### Commands
1. **List**
    - **Description:** Lists all gists either for the authenticated user or for a specific user if provided.
    - **Usage:** `gist_magic list [OPTIONS]`
    - **Options:**
        - `-o, --username <username>`: Specify the username of the owner of the gists to list.

2. **Create**
    - **Description:** Creates a new gist.
    - **Usage:** `gist_magic create`

3. **Edit**
    - **Description:** Edits an existing gist.
    - **Usage:** `gist_magic edit [OPTIONS]`
    - **Options:**
        - `-g, --gistid <gistid>`: Specify the ID of the gist to edit.

4. **Delete**
    - **Description:** Deletes an existing gist.
    - **Usage:** `gist_magic delete`

5. **View**
    - **Description:** Views details of a specific gist.
    - **Usage:** `gist_magic view [OPTIONS]`
    - **Options:**
        - `-g, --gistid <gistid>`: Specify the ID of the gist to view.

6. **Star**
    - **Description:** Stars (favorites) a specific gist.
    - **Usage:** `gist_magic star [OPTIONS]`
    - **Options:** None

7. **Unstar**
    - **Description:** Unstars (removes from favorites) a specific gist.
    - **Usage:** `gist_magic unstar [OPTIONS]`
    - **Options:** None

### Additional Notes
- **Authorization:** All commands except `list` require a GitHub token for authorization.
- **Error Handling:** Errors are logged and printed to the console for user visibility.
- **Output:** The output for commands like `list` and `view` includes gist details such as ID, description, owner, and gist files (if applicable).


## Installation

1. Clone the repository:

    ```shell
    git clone https://github.com/your-username/rust-gist-magic.git
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


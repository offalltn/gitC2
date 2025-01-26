GitC2 is a simple C2 POC that leverages a GitHub repository for executing commands through issues.It is intended solely as a Proof-of-Concept to demonstrate how GitHub could be exploited for malicious purposes. Please note that this is not a polished or production-ready tool.
This project provides a Rust-based automation tool that interacts with GitHub repositories using the Octocrab library. It performs tasks such as managing issues, creating comments, and executing commands based on GitHub issue comments. It also supports file uploads, screenshots, and scheduled tasks.
Key Features

    Interact with GitHub repositories using a Personal Access Token (PAT).
    Create and manage GitHub issues dynamically.
    Respond to GitHub issue comments and execute predefined commands.
    Support for file upload to a GitHub repository.
    Capture screenshots and upload them to the repository.
    creation and scheduled tasks (well deteected)

Requirements
Prerequisites

    Rust installed on your system (Install Rust).
    A GitHub Personal Access Token (PAT) with repository permissions.
    A GitHub repository where the tool can create and manage issues.
    Windows operating system (for certain features like attrib +h and scheduled tasks).

Installation

    1-Clone the repository
    2-Configure your GitHub Personal Access Token: Replace TOKEN_HERE in the code with your GitHub PAT. 
    3-repo owner and repo name variables should be changed also.

build the application on linux:

    cargo build --release --target x86_64-pc-windows-gnu 

Usage
Commands Supported in GitHub Issue Comments. All interactions to the agents will be done from the issues dashboard

    Note: you can uncommnet the first line in the script to make the cmd window not visible to the user.
    cmd: Execute a system command.
        Example: cmd dir

    persist: Copy the executable to a hidden directory and create a scheduled task.

    screenshot: Capture a screenshot of the screen and upload it to the repository.

    download <file-path>: Upload a file from the specified local path to the GitHub repository.

Screenshots
![persist](https://github.com/user-attachments/assets/ae5ff193-e2ef-4a6d-b6d8-c028ce7b05f9)
![issues](https://github.com/user-attachments/assets/6b1b46a5-c367-4dc7-a50b-d175f573fe49)
![7310](https://github.com/user-attachments/assets/35d8e048-d725-45ef-b881-6f6543b3cc1e)


Project Workflow

    The application initializes by creating a hidden directory for configuration files.
    It checks if a configuration file exists:
        If not, it creates a new GitHub issue and stores the issue number locally.
        If it exists, the stored issue number is used to manage comments.
    It continuously monitors the GitHub issue for new comments.
    Based on the comment commands, it performs actions like:
        Executing system commands.
        Uploading files or screenshots to the repository.
        Scheduling tasks for persistent operations.

This tool should be used responsibly and ethically. Misuse of this application, especially in ways that violate GitHub's Terms of Service or local laws, is strictly prohibited. Always ensure you have the necessary permissions for any action performed.



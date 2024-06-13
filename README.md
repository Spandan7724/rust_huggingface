# Rust Chatbot with Hugging Face API
==================================

This Rust project implements a simple chatbot using the Hugging Face model inference API. It allows users to interact with the chatbot by typing queries, which are then sent to the API for processing and returning responses.


## Requirements

- Rust (version 1.51 or higher)
- Cargo (Rust's package manager)
- API token from Hugging Face (stored in `.env` file as `HF_TOKEN`)

## Installation and Usage

1. **Clone the Repository**:

    ```bash
    git clone https://github.com/your-username/rust-chatbot.git
    cd rust-chatbot
    ```

2. **Set Up Environment Variables**:

    - Add your Hugging Face API token in the `.env` file:

      ```plaintext
      HF_TOKEN=your_hugging_face_api_token_here
      ```

3. **Build and Run**:

    ```bash
    cargo run
    ```

4. **Interaction**:

    - Enter your query or prompt after the `>` prompt.
    - The chatbot will process the input, fetch a response from the API, and display it along with the previous chat history.

5. **Exiting**:

    - Press `Ctrl + C` to exit the application.

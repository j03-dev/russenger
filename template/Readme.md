# {{project-name}}

## Who to use

### Pls check the `.env` file and `Rocket.toml` file before

### Build
* run `cargo build`

### Migrate the database
* run `cargo run migrate`

### Run Russenger server
* run `cargo run runserver`

### Endpoints

- **GET `/webhook`:** Verify your chatbot with Facebook Messenger. Facebook will send a challenge, and your bot must
  respond correctly for verification.

- **POST `/webhook`:** This is where Facebook Messenger sends messages from users. Handle incoming messages and respond
  accordingly here.

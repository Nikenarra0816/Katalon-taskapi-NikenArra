# Katalon API

## Project Description
This project aims to test APIs using Katalon Studio. The testing covers various endpoints to ensure that responses and functionalities align with specifications.

## API Testing Scenarios
https://docs.google.com/spreadsheets/d/1f0HQPfkgtnqEXto722RvQ45b5BfNzpywTDwcUroPdJ4/edit?usp=sharing

## Project Structure
- **Test Cases**: Contains individual test scenarios for each endpoint
- **Object Repository**: Stores object definitions that interact with the API.
- **Test Suites**:A collection of test cases that can be executed together for comprehensive testing.

![image](https://github.com/user-attachments/assets/7721e89c-d5d7-484c-bd36-9b3b481ea363)


## Testing Explanation
- **POST Log In User**: In User: Verifies that the API correctly processes user credentials and returns a **200 OK** response . Endpoint ➡️ https://thinking-tester-contact-list.herokuapp.com/users/login

![image](https://github.com/user-attachments/assets/47e5bc5b-6c66-4cef-970c-4a7122e42f8a)

- **GET User Profile**: Retrieves the API data of the currently logged-in user with a **200 OK** status. Endpoint ➡️ https://thinking-tester-contact-list.herokuapp.com/users/me

![image](https://github.com/user-attachments/assets/3ce3d59f-9387-4a74-930f-829de75cf3e4)

- **PATCH Update User**: Updates partial API data of a registered user in the system with a **200 OK** status. Endpoint ➡️ https://thinking-tester-contact-list.herokuapp.com/users/me

![image](https://github.com/user-attachments/assets/0a58d829-ab4e-4e8e-8610-8d5f99fddd17)

- **POST Log Out User**: Verifies that the API successfully logs out the user from the system with a **200 OK** status. Endpoint ➡️ https://thinking-tester-contact-list.herokuapp.com/users/logout

![image](https://github.com/user-attachments/assets/fb44cd6e-c43a-4b21-91eb-3d156e8366cd)

- **DELETE Delete User**:Deletes the user’s API credentials from the system with a **200 OK** status. Endpoint ➡️ https://thinking-tester-contact-list.herokuapp.com/users/me

![image](https://github.com/user-attachments/assets/6bfc3dcb-beae-4c9b-86a8-85a63c1984e4)

## Testing Step

### Environment Setup
1. Download and install [Katalon Studio](https://www.katalon.com/download/).
2. Clone this repository:
   ```bash
   git clone https://github.com/Nikenarra0816/Katalon-taskapi-NikenArra.git
3. Running test pada **Test Suite**

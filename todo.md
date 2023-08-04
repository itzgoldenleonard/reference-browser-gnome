# Before first draft release
- [ ] Send write requests with form data
    - [X] Set up to store the form data in the window
    - [X] Make an int form field
    - [X] Get the int form field to store its input in the window
    - [X] Send and recieve write requests with body
    - [X] Get the data from the window to the SubmitFormField
    - [X] Make it actually respond to ids
    - [X] Send the data in a write request
    - [ ] Serialize the data properly
        - [X] Make an Input struct to store in the form_data of Window with all the data needed for serialization
        - [X] Implement PartialEq with ID for Input
        - [X] Make the form fields use Input
        - [X] Update the function that finds an element in the form_data vector by ID to use Input
        - [ ] Implement Serialize for Input
- [ ] Render all the different form field types (of course they also have to work)
- [ ] Mostly accurate form field validation
- [ ] Form field validation from server
- [ ] Make requests (both read and form response) with a client certificate (but it has to be manually generated with openssl)
- [ ] Add an option to use a custom language preference list
- [ ] Submit to flathub

# Bugs
- [ ] Files with spaces in their names dont open (no such file or directory found) (I think it's because I am using the wrong method to get the file path from the file:// uri in the get_document_by_file function)

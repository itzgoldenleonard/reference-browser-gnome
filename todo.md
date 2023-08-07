# Before first draft release
- [ ] Render all the different form field types (of course they also have to work)
    - [X] int
    - [X] float
    - [X] string
    - [X] bool
    - [ ] date
    - [ ] email
    - [ ] tel
    - [ ] file
    - [ ] list
    - [ ] Labels
- [ ] Mostly accurate form field validation
- [ ] Form field validation from server
- [ ] Make requests (both read and form response) with a client certificate (but it has to be manually generated with openssl)
- [ ] Add an option to use a custom language preference list
- [ ] Submit to flathub

# Bugs/minor issues
- [ ] Files with spaces in their names dont open (no such file or directory found) (I think it's because I am using the wrong method to get the file path from the file:// uri in the get_document_by_file function)
## Forms
- [ ] The min property doesnt work on string fields
- [ ] Secret string fields dont use the PasswordEntry widget
- [ ] Multiline string fields only work if you paste multi line text into them
- [ ] Optional boolean fields dont have a way to reset to undecided

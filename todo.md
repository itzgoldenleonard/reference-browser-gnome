# Before first draft release
- [ ] Send write requests with form data
- [ ] Render all the different form field types (of course they also have to work)
- [ ] Mostly accurate form field validation
- [ ] Form field validation from server
- [ ] Make requests (both read and form response) with a client certificate (but it has to be manually generated with openssl)
- [ ] Add an option to use a custom language preference list
- [ ] Submit to flathub

# Bugs
- [ ] Files with spaces in their names dont open (no such file or directory found) (I think it's because I am using the wrong method to get the file path from the file:// uri in the get_document_by_file function)

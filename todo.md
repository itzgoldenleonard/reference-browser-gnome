# Before first draft release
- [ ] Render all the different form field types (of course they also have to work)
    - [X] int
    - [X] float
    - [X] string
    - [X] bool
    - [X] date
    - [X] email
    - [ ] file
        - [ ] Basic
        - [ ] Max
        - [ ] MIME type
        - [ ] Serialize it
        - [ ] Error
    - [ ] Labels
- [ ] Mostly accurate form field validation
- [ ] Multiple forms on 1 page
- [ ] Form field validation from server (message)
- [ ] Make requests (both read and form response) with a client certificate (but it has to be manually generated with openssl)
- [ ] Add an option to use a custom language preference list
- [ ] Submit to flathub

# Bugs
- [ ] Files with spaces in their names dont open (no such file or directory found) (I think it's because I am using the wrong method to get the file path from the file:// uri in the get_document_by_file function)
- [ ] A line beginning with xx??? still gets registered as a form field

# After first draft release
## Forms
- [ ] The min property doesnt work on string fields
- [ ] Secret string fields dont use the PasswordEntry widget
- [ ] Multiline string fields only work if you paste multi line text into them
- [ ] Optional boolean fields dont have a way to reset to undecided
- [ ] min and max properties dont work for date form fields
- [ ] Timeonly form fields behave weirdly with timezones
- [ ] Implement tel fields
- [ ] Implement list fields
- [ ] Do form fields properly
    Make a FormFieldExt trait, and make every type of form field a subclassed class of the FormField parent class (I dont speak OOP). Give them all a unified API, make it so that you can down/upcast them between FormField and the subclass, give them some common signals that can be used to store their value in the Window, give them a common constructor so that they're easy to make with just an ID and a field enum, give them some common properties for things like server form field validation.
- [ ] Proper server side form validation


# Before first draft release
- [ ] Render all the different form field types (of course they also have to work)
    - [X] int
    - [X] float
    - [X] string
    - [X] bool
    - [ ] date
        - [ ] Get the time pickers to have the right min and maxes
        - [ ] Emit a signal on change with the right data
        - [ ] Connect to the signal to put that data into the Window
        - [ ] Implement form field properties
        - [ ] When either time or date changes send the signal with the correct value
        - [ ] Get it to serialize properly
        - [ ] Clean up a bit
    - [ ] email
    - [ ] file
    - [ ] Labels
- [ ] Mostly accurate form field validation
- [ ] Form field validation from server (message)
- [ ] Make requests (both read and form response) with a client certificate (but it has to be manually generated with openssl)
- [ ] Add an option to use a custom language preference list
- [ ] Submit to flathub

# Bugs
- [ ] Files with spaces in their names dont open (no such file or directory found) (I think it's because I am using the wrong method to get the file path from the file:// uri in the get_document_by_file function)

# After first draft release
## Forms
- [ ] The min property doesnt work on string fields
- [ ] Secret string fields dont use the PasswordEntry widget
- [ ] Multiline string fields only work if you paste multi line text into them
- [ ] Optional boolean fields dont have a way to reset to undecided
- [ ] Implement tel fields
- [ ] Implement list fields
- [ ] Do form fields properly
    Make a FormFieldExt trait, and make every type of form field a subclassed class of the FormField parent class (I dont speak OOP). Give them all a unified API, make it so that you can down/upcast them between FormField and the subclass, give them some common signals that can be used to store their value in the Window, give them a common constructor so that they're easy to make with just an ID and a field enum, give them some common properties for things like server form field validation.
- [ ] Proper server side form validation

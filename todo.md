# Before first draft release
- [ ] Add scrolling and make the lines not clickable
    There are 2 ways to do this.
    1. Every time you append an object to the canvas you wrap it in a ListBoxRow that isnt activatable
    2. At the end of the function you go through all of the children of the canvas and set their activatable property to false

- [ ] Send a form response
- [ ] Render all the different form field types (of course they also have to work)
- [ ] Mostly accurate form field validation
- [ ] Form field validation from server
- [ ] Redirect form sections
- [ ] Make requests (both read and form response) with a client certificate (but it has to be manually generated with openssl)
- [ ] Add an option to use a custom language preference list
- [ ] Submit to flathub

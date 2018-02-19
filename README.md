# Fui

Add form interface for your CLI program.

## Note:
**Use it at own risk!!**

## TODO:

* docs
* select in autocomplete load values while scrolling
* support user's history!
    * make fill-error-correct flow pleasent
* support for piping!
* create wrapper FileField
* create wrapper DirField
* ctrl+enter submits ([#151](https://github.com/gyscos/Cursive/issues/151))?
* checkbox: automatic toggle on char
* add Field.data & form on_submit returns it?
* optimalizations
    * feeders use iterators
    * thread
* tests
* error handling & unwraps
* magic stuff:
    * add magic which renders form for clap (or structopt) if args missing
    * add magic which works with current programs like: ls, grep, etc.

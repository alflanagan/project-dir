# project-dir

For some time I've been using a set of shell functions to create a command called
"project" which does the following:

1. Search a set of directories for "project" directories, mainly ones with a .git subdirectory.
2. `cd` to the directory whose base name is the argument to the project command.
3. If the directory has a python environment defined, activate it. If it's got a version of node defined, use `nvm use _version_` to enable it. If it's got a version of ruby defined... you get the general idea, right?

Big problem with this command: it uses `find` each time it's invoked, even if it's already searched for projects, and it is dog-slow. I mean really, really slow. So here's an attempt to do something similar with Rust, caching the `find` results and generally pushing performance to the maximum.

Step 3. above will be a challenge, requiring some sort of interaction with the shell. Off the top of my head, the easiest thing would be to write a series of commands to stdout and use `eval "$(project _name_)"` in the shell.

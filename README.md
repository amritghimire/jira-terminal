## Jira Terminal

This application can be used for personal usage to manage jira from terminal.

## Installation

This application can be used in multiple platform.

### MacOS
This package is available in brew as `jira-terminal`. You can install it with following command:
```
brew tap amritghimire/jira-terminal 
brew install jira-terminal
```

### Arch Linux
This package is available in aur repository as [jira-terminal-bin](https://aur.archlinux.org/packages/jira-terminal-bin/)

### Debian/Ubuntu
On debian based system, the deb file is available in [releases](https://github.com/amritghimire/jira-terminal/releases). You can download latest release from there. Please make sure libc is installed in your system.

### Cargo
If you already have a Rust environment set up, you can use the cargo install command:
```cargo install jira-terminal```

Cargo will build the jira-terminal binary and place it in $HOME/.cargo/bin.
You can also setup Rust toolchain from [Rust official site](https://www.rust-lang.org/tools/install)

### Manual Installation from Github
Compiled binary versions of jira-terminal are uploaded to GitHub when a release is made. You can install jira-terminal manually by [downloading a release](https://github.com/amritghimire/jira-terminal/releases) , extracting it, and copying the binary to a directory in your $PATH, such as /usr/local/bin.


## Autocompletion Script
The autocompletion script can be found in [the release section](https://github.com/amritghimire/jira-terminal/releases).
You can download the autocompletion script from there or use our application to generate the script.
To generate the script, run:

```bash
jira-terminal autocompletion --shell [zsh|bash|fish|powershell|elvish] > _jira.terminal
```
Depending on your shell, you can move your autocompletion file to the following location:
- *ZSH* - `/usr/share/zsh/site-functions/_jira-terminal`
- *BASH* - `/usr/share/bash-completion/completions/_jira-terminal`
- *Fish* - `/share/fish/vendor_completions.d/_jira-terminal`


## Usage
When running the application for first time, you will be asked with following values.
- hostname [This will be used to identify the jira hostname to be used.]
- email [Email address you use to login with the application.]
- token [You can obtain the app password from the link specified in the application]

After that, you can use following commands for help.
```
jira-terminal help
jira-terminal help list
jira-terminal help transition
jira-terminal help alias
jira-terminal help detail
jira-terminal help fields
jira-terminal help update
jira-terminal help new
jira-terminal help assign
jira-terminal help comment
jira-terminal help autocompletion
```

```
JIRA Terminal 2.0.0
Amrit Ghimire <oss@amritghimire.com>
This is a command line application that can be used as a personal productivity tool for interacting with JIRA

USAGE:
    jira-terminal [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    alias             Configuration for alias. One of add,list or remove is required.
    assign            Assign a ticket to user.
    autocompletion    Generate autocompletion script..
    comment           List or add comments to a ticket. Default action is adding.
    detail            Detail of a JIRA tickets..
    fields            List of possible Fields for details...
    help              Prints this message or the help of the given subcommand(s)
    list              List the issues from JIRA.
    new               Create a new ticket.
    transition        Transition of ticket across status.
    update            Update a field for a ticket
```

### List of Tickets
```

jira-terminal-list 
List the issues from JIRA.

USAGE:
    jira-terminal list [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -J, --json       JSON response
    -M, --me         Issues assigned to you.
    -V, --version    Prints version information

OPTIONS:
    -A, --alias <ALIAS>               Save the applied options as an alias. You can use it with jql option later.
    -a, --assignee <ASSIGNEE>...       Assignee username or email to filter with.
    -c, --component <COMPONENT>...    Component name or ID to filter with.
    -C, --count <COUNT>               Total number of issues to show. (Default is 50)
    -d, --display <DISPLAY>            Comma separated list of fields to display.
                                      Possible options for fields are:
                                      key,resolution,priority,assignee,status,components,creator,reporter,issuetype,project,summary
                                      
                                      You can pass alias as option for display. You can save alias using alias
                                      subcommand for the application.
                                      
                                       Default options are
                                       key,summary,status,assignee
                                                         
    -e, --epic <EPIC>...              EPIC name or issue key of epic to filter with.
    -f, --filter <FILTER>...          Filter name or filter id that you saved in JIRA.
    -j, --jql <JQL>                   JQL Query or alias to JQL query to filter with.
    -l, --label <LABEL>...            Search for issues with a label or list of labels.
    -o, --offset <OFFSET>             Offset to start the first item to return in a page of results. (Default is 0)
    -m, --main <PARENT>...            Search for subtask of a particular issue.
    -P, --priority <PRIORITY>...      Search for issues with a particular priority.
    -p, --project <PROJECT>...        Project Code to filter with.
    -r, --reporter <REPORTER>...      Search for issues that were reported by a particular user.
    -s, --sprint <SPRINT>...          Search for issues that are assigned to a particular sprint.
    -S, --status <STATUS>...          Search for issues that have a particular status.
    -T, --text <TEXT>                 This is a master-field that allows you to search all text fields for issues.
    -t, --type <TYPE>...              Search for issues that have a particular issue type. 

You can specify the following fields multiple time to filter by multiple values.
assignee, component, epic, filter, label, main, priority, project, reporter, sprint, status, type.

For example to fetch list of tickets in Backlog and In progress, you can use
jira-terminal list -s Backlog -s 'In Progress'
```


### Transition

```
jira-terminal-transition 
Transition of ticket across status.

USAGE:
    jira-terminal transition [FLAGS] <STATUS> --ticket <TICKET>

FLAGS:
    -h, --help       Prints help information
    -l, --list       List the possible transitions.
    -V, --version    Prints version information

OPTIONS:
    -t, --ticket <TICKET>    Ticket ID from JIRA.

ARGS:
    <STATUS>    Status or alias of status to move the ticket to.

```


### Alias
```
jira-terminal-alias 
Configuration for alias. One of add,list or remove is required.

USAGE:
    jira-terminal alias [FLAGS] <NAME> --add <add> --list --remove

FLAGS:
    -h, --help       Prints help information
    -l, --list       List the alias saved.
    -r, --remove     List the alias saved.
    -V, --version    Prints version information

OPTIONS:
    -a, --add <add>    Value to associate with provided alias name.

ARGS:
    <NAME>    Name of alias. (Required except for list option)
```

Sample usage:
- `jira-terminal alias -l`
- `jira-terminal alias alias_name -a "Alias Value"`
- `jira-terminal alias -r alias_name` 

### Detail
```
jira-terminal-detail 
Detail of a JIRA tickets..

USAGE:
    jira-terminal detail [OPTIONS] <TICKET>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --fields <fields>    Comma separated lists of fields or alias to show.
                             Possible options are: 
                             key,summary,description,status,issuetype,priority,labels,assignee,components,creator,reporter,project,comment
                             
                             You can use all to show all fields.
                             Default selection are:
                             key,summary,description
                                                 

ARGS:
    <TICKET>    Ticket id for details.

```

### Fields
```
jira-terminal-fields 
List of possible Fields for details...

USAGE:
    jira-terminal fields <TICKET>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <TICKET>    Ticket id for details.
```

### Update
```
jira-terminal-update 
Update a field for a ticket

USAGE:
    jira-terminal update <TICKET> --field <field> --value <value>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --field <field>    Key of field to update. You can use jira-terminal fields <TICKET> to see possible set of
                           keys.
    -v, --value <value>    Value of the field to update.

ARGS:
    <TICKET>    Ticket ID to update
```


### New
```
jira-terminal-new 
Create a new ticket.

USAGE:
    jira-terminal new [FLAGS] [OPTIONS] --main <main> --project <project>

FLAGS:
    -h, --help       Prints help information
    -M, --minimal    Only summary and description will be asked if not available.
    -q, --quiet      Do not ask for missing options.
    -V, --version    Prints version information

OPTIONS:
    -a, --assignee <assignee>          Assignee email of ticket
    -c, --components <components>      Comma separated list of components of ticket
    -C, --custom <custom>              Comma separated value pair for custom fields. You can use alias in value or key
                                       itself. Example- "customfield_12305:value,alias_to_key:value2. You can use fields
                                       subcommand to check the list of custom fields available. 
    -d, --description <description>    Description of ticket
    -l, --labels <labels>              Comma separated list of labels.
    -m, --main <main>                  Main ticket to create the sub-ticket.
    -p, --priority <priority>          Priority Of the ticket.
    -P, --project <project>            Project Key to create the ticket.
    -s, --summary <summary>            Summary of ticket
    -t, --type <type>                  Issue type for new ticket.
```

### Assign
```
jira-terminal-assign 
Assign a ticket to user.

USAGE:
    jira-terminal assign --ticket <ticket> --user <user>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --ticket <ticket>    Ticket to use.
    -u, --user <user>        Assign the ticket to the provided user.
```

### Comment
```
jira-terminal-comment 
List or add comments to a ticket. Default action is adding.

USAGE:
    jira-terminal comment [FLAGS] [OPTIONS] --ticket <ticket>

FLAGS:
    -h, --help       Prints help information
    -l, --list       List all the comments of a ticket.
    -V, --version    Prints version information

OPTIONS:
    -b, --body <body>        Body of the comment. To mention someone, you can use @(query) The query can include jira
                             username or display name or email address.
    -t, --ticket <ticket>    Ticket to use.
```

## Notes
- The credentials and other configuration are stored in a file `~/.jira_terminal_configuration.json`. The base64 encoded version of credentials are only written in the configuration file.

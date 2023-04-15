// A command with name and arguments. Optionally redirected input and output files
pub struct SingleCommand {
    pub name: String,
    pub args: Vec<String>,
    pub in_redirect: Option<String>,
    pub out_redirect: Option<String>
}

impl SingleCommand {
    pub fn make() -> Self {
        SingleCommand { name: String::new(), args: Vec::new(), in_redirect: None, out_redirect: None }
    }

    pub fn is_uninitialized(&self) -> bool {
        self.name.is_empty()
    }

    // add a token to the name, arguments, or the input/output redirection files of the command
    pub fn build(&mut self, token: &str, inp_redirection: bool, outp_redirection: bool) {
        if self.name.is_empty() {
            self.name = String::from(token);
        } else if inp_redirection {
            self.in_redirect = Some(String::from(token));
        } else if outp_redirection {
            self.out_redirect = Some(String::from(token));
        } else {
            self.args.push(String::from(token));
        }
    }
}

// A command group means a group of piped commands, i.e all commands separated by a pipe |
// there is one command group for each '&' separated sequence
pub struct CommandGroup {
    commands: Vec<SingleCommand>
}

impl CommandGroup {
    pub fn make() -> Self {
        CommandGroup { commands: Vec::new() } 
    }

    pub fn append(&mut self, cmd: SingleCommand) {
        self.commands.push(cmd);
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn x_to_last(&self, x: usize) -> &[SingleCommand] {
        &self.commands[x..] 
    } 

    pub fn first(&self) -> Option<&SingleCommand> {
        self.commands.first()
    }
}

// The Commands struct holds a vector of command groups, i.e all command groups separated by a '&'
pub struct Commands {
    command_groups: Vec<CommandGroup>
}

impl Commands {
    pub fn make() -> Self {
        Commands { command_groups: Vec::new() }
    }

    pub fn append(&mut self, cmd_group: CommandGroup) {
        self.command_groups.push(cmd_group);
    }

    pub fn get_last(&mut self) -> Option<CommandGroup> {
        self.command_groups.pop()
    }

    pub fn has_more(&self) -> bool {
        self.command_groups.len() > 0
    }
}

// Create the commands structure from the input tokens
pub fn make_commands(tokens: Vec<&str>) -> Commands {
    let mut commands: Commands = Commands::make(); 
    let mut group: CommandGroup = CommandGroup::make();
    let mut command: SingleCommand = SingleCommand::make();
    let mut next_is_input_redirection = false;
    let mut next_is_ouput_redirection = false;
    for token in tokens {
        if token.eq("&") {
            // Add the command to the group. Add the group to the commands. Start building a new group
            group.append(command);
            command = SingleCommand::make();
            commands.append(group);
            group = CommandGroup::make();
        } else if token.eq("|") {
            // Add the command to the group. Start building a new command
            group.append(command);
            command = SingleCommand::make();
        } else if token.eq("<") {
            // the next token is the name of the file to read input from
            next_is_input_redirection = true;
        } else if token.eq(">") {
            // the next token is the name of the file to write output to
            next_is_ouput_redirection = true;
        } else {
            // build up the current command
            command.build(token, next_is_input_redirection, next_is_ouput_redirection);
            next_is_input_redirection = false;
            next_is_ouput_redirection = false;
        }
    }
    if !command.is_uninitialized() {
        group.append(command);
    }

    if !group.is_empty() {
        commands.append(group);
    }

    commands
}
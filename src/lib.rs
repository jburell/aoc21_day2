enum Command {
	Forward(u32),
	Up(u32),
	Down(u32)
}

type UnvalidatedCommands = Vec<(String, u32)>;
type Commands = Vec<Command>;

pub fn calc_dive_coords(commands: UnvalidatedCommands) -> Result<u32, String> {
	let (h, v) = 
		validate_commands(commands)?
		.iter()
		.fold((0, 0), |(horiz, vert), cmd| {
			match cmd {
				Command::Forward(x) => (horiz + x, vert),
				Command::Up(y) => (horiz, vert - y),
				Command::Down(y) => (horiz, vert + y),
			}
		});
	Ok(h * v)
}

pub fn calc_dive_coords_with_aim(commands: UnvalidatedCommands) -> Result<u32, String> {
	let (h, v, _) = 
	validate_commands(commands)?
	.iter()
	.fold((0, 0, 0), |(horiz, vert, aim), cmd| {
		match cmd {
			Command::Forward(x) => (horiz + x, vert + aim * x, aim),
			Command::Up(y) => (horiz, vert, aim - y),
			Command::Down(y) => (horiz, vert, aim + y),
		}
	});
	Ok(h * v)
}

fn validate_commands(commands: UnvalidatedCommands) -> Result<Commands, String> {
	commands.iter()
	.map(to_command)
	.collect::<Result<Commands, String>>()
}

fn to_command(cmd: &(String, u32)) -> Result<Command, String> {
	match (cmd.0.as_str(), cmd.1) {
		("forward", x) => Ok(Command::Forward(x)),
		("up", x) => Ok(Command::Up(x)),
		("down", x) => Ok(Command::Down(x)),
		_ => Err(format!("Illegal command {:?}", cmd))
	}
}
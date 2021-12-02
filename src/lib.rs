enum Command {
	Forward(u32),
	Up(u32),
	Down(u32)
}

pub fn calc_dive_coords(commands: Vec<(String, u32)>) -> Result<u32, String> {
	let cmd: Vec<Command> = 
		commands.iter()
		.map(to_command)
		.collect::<Result<Vec<Command>, String>>()?;
	let (h, v) = 
		cmd.iter()
		.fold((0, 0), |(horiz, vert), cmd| {
			match cmd {
				Command::Forward(x) => (horiz + x, vert),
				Command::Up(y) => (horiz, vert - y),
				Command::Down(y) => (horiz, vert + y),
			}
		});
	Ok(h * v)
}

fn to_command(cmd: &(String, u32)) -> Result<Command, String> {
	match (cmd.0.as_str(), cmd.1) {
		("forward", x) => Ok(Command::Forward(x)),
		("up", x) => Ok(Command::Up(x)),
		("down", x) => Ok(Command::Down(x)),
		_ => Err(format!("Illegal command {:?}", cmd))
	}
}


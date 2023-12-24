use std::io::stdin;

#[derive(Debug)]
struct Member {
	name: String,
	action: MemberAction,
	age: i8,
}


impl Member {
	fn new(name: &str, action: MemberAction, age: i8) -> Self {
		Self {
			name: name.to_lowercase(),
			action,
			age,
		}
	}

	fn greet_member(&self) {
		match &self.action {
			MemberAction::Accept => println!("Welcome to the treehouse, {}.", self.name),
			MemberAction::AcceptWithNote { note } => {
				println!("Welcome to the treehouse, {}.", self.name);

				println!("{}", note);

				if self.age < 21 {
					println!("Do not serve alcohol to {}.", self.name);
				}
			},
			MemberAction::Probation => println!("{} is a probation member.", self.name),
			MemberAction::Refuse => println!("Do not allow {} in!", self.name)
		}
	}
}

#[derive(Debug)]
enum MemberAction {
	Accept,
	AcceptWithNote {note: String},
	Refuse,
	Probation,
}


fn main() {
	let mut member_list = vec![
		Member::new("Bert", MemberAction::Accept, 45),
		Member::new("Steve", MemberAction::AcceptWithNote{note: "Lactose-free milk is in the fridge.".to_string()}, 15),
		Member::new("Fred", MemberAction::Refuse, 30),
	];

	loop {
		println!("Hello, What is your name? “(Leave empty and press ENTER to quit)");

		let mut name = String::new();

		stdin()
				.read_line(&mut name)
				.expect("Failed to read name.");

		name = name
			.trim()
			.to_lowercase();

		let member = member_list
			.iter()
			.find(|member| member.name == name);

		match member {
			Some(x) => x.greet_member(),
			_ => {
				if name.is_empty() {
					break;
				}
				else {
					println!("{} is not on the member list.", name);

					member_list.push(Member::new(&name, MemberAction::Probation, 0));
				}
			}
		}
	}

	println!("The list of final members:");
	println!("{:#?}", member_list);
}

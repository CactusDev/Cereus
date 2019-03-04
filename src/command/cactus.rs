
use command::Command;
use packet::Context;

pub fn create_cactus_command() -> Command {
	command!("cactus",
		"default" => handler!(|_context| {
            Context::message(vec! [
                text!("Ohai! I'm CactusBot! "),
                emoji!("cactus")
            ])
        }),
		"docs" => handler!(|_context| {
			Context::message(vec! [
				text!("Check out my documentation at "),
				url!("https://cactusbot.rtfd.org"),
				text!(".")
			])
		}),
		"twitter" => handler!(|_context| {
			Context::message(vec! [
				text!("You can follow the team behind CactusBot at: "),
				url!("https://twitter.com/CactusDevTeam"),
				text!("!")
			])
		}),
		"help" => handler!(|_context| {
			Context::message(vec! [
				text!("Try our docs (!cactus docs). If that doesn't help, tweet us (!cactus twitter)!")
			])
		}),
		"github" => handler!(
            "default" => handler!(|_context| {
                Context::message(vec! [
                    text!("We're open source! Check it out at: "),
                    url!("https://github.com/CactusDev")
                ])
            }),
            "issue" => handler!(|_context| {
                Context::message(vec! [
                    text!("Checkout Cereus at: "),
                    url!("https://github.com/CactusDev/CactusBot/issues")
                ])
            })
        )
    )
}

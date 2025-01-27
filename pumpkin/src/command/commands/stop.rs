use async_trait::async_trait;
use pumpkin_util::text::color::NamedColor;
use pumpkin_util::text::TextComponent;

use crate::command::args::ConsumedArgs;
use crate::command::tree::CommandTree;
use crate::command::{CommandError, CommandExecutor, CommandSender};

const NAMES: [&str; 1] = ["stop"];

const DESCRIPTION: &str = "Stop the server.";

struct StopExecutor;

#[async_trait]
impl CommandExecutor for StopExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender<'a>,
        server: &crate::server::Server,
        _args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        sender
            .send_message(
                TextComponent::translate("commands.stop.stopping", [].into())
                    .color_named(NamedColor::Red),
            )
            .await;

        // TODO: Gracefully stop

        let kick_message = TextComponent::text("Server stopped");
        for player in server.get_all_players().await {
            player.kick(kick_message.clone()).await;
        }
        server.save().await;
        std::process::exit(0)
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).execute(StopExecutor)
}

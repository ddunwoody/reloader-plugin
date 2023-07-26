/*
 * Copyright © 2023 David Dunwoody.
 *
 * All rights reserved.
 */

#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::convert::Infallible;
use std::ffi::CString;

use xplm::command::{CommandHandler, OwnedCommand};
use xplm::plugin;
use xplm::plugin::Plugin;
use xplm::xplane_plugin;
use xplm_sys::{
    XPLMAppendMenuItemWithCommand, XPLMFindCommand, XPLMFindPluginsMenu, XPLMReloadPlugins,
};

struct ReloaderPlugin {
    _command: OwnedCommand,
}

static MENU_NAME: &str = "Reload all plugins";
static COMMAND_NAME: &str = "flc/reloader/reload_all_plugins";

fn add_plugin_menu_item_with_command(menu_name: &str, command_name: &str) {
    unsafe {
        let menu_name_c = CString::new(menu_name).unwrap();
        let command_name_c = CString::new(command_name).unwrap();
        XPLMAppendMenuItemWithCommand(
            XPLMFindPluginsMenu(),
            menu_name_c.as_ptr(),
            XPLMFindCommand(command_name_c.as_ptr()),
        );
    }
}

impl Plugin for ReloaderPlugin {
    type Error = Infallible;

    fn start() -> Result<Self, Self::Error> {
        let command = OwnedCommand::new(COMMAND_NAME, MENU_NAME, ReloadCommandHandler).unwrap();

        add_plugin_menu_item_with_command(MENU_NAME, COMMAND_NAME);

        Ok(ReloaderPlugin { _command: command })
    }

    fn info(&self) -> plugin::PluginInfo {
        plugin::PluginInfo {
            name: String::from("Plugin Reloader"),
            signature: String::from("uk.co.flightlevelchange.reloader"),
            description: String::from("Reloads plugins"),
        }
    }
}

struct ReloadCommandHandler;

impl CommandHandler for ReloadCommandHandler {
    fn command_begin(&mut self) {
        unsafe { XPLMReloadPlugins() }
    }
    fn command_continue(&mut self) {}
    fn command_end(&mut self) {}
}

xplane_plugin!(ReloaderPlugin);

use crate::commands::config::RustVSCodeTasksOption;
use crate::languages::rust::tasks::RustVSCodeTask;

use crate::terminal::CyanANSI;
use crate::terminal::Terminal;

pub struct RustVSCodeTaskCommand {
    option: RustVSCodeTasksOption,
    terminal: Terminal,
}

impl RustVSCodeTaskCommand {
    pub fn new(option: RustVSCodeTasksOption) -> Self {
        let terminal: Terminal = Terminal::new();
        RustVSCodeTaskCommand { option, terminal }
    }

    pub fn execute_command(&self) {
        if self.option.run_task {
            let string: &str = "[Generating Run Task]";
            self.terminal.writeln_ansi(string, &CyanANSI);

            let vscode_task: RustVSCodeTask = RustVSCodeTask::new();
            vscode_task.generate_run_task();
        }
    }
}

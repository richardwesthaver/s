use rustyline::{
  completion::{Completer, FilenameCompleter, Pair},
  config::OutputStreamType,
  error::ReadlineError,
  highlight::{Highlighter, MatchingBracketHighlighter},
  hint::{Hinter, HistoryHinter},
  validate::{self, MatchingBracketValidator, Validator},
  Cmd, CompletionType, Config, Context, EditMode, Editor, KeyEvent,
};
use std::borrow::Cow::{self, Borrowed, Owned};
use std::env;
use std::path::PathBuf;

use rustyline_derive::Helper;

#[derive(Helper)]
struct DmcHelper {
  completer: FilenameCompleter,
  highlighter: MatchingBracketHighlighter,
  validator: MatchingBracketValidator,
  hinter: HistoryHinter,
  colored_prompt: String,
}

impl Completer for DmcHelper {
  type Candidate = Pair;

  fn complete(
    &self,
    line: &str,
    pos: usize,
    ctx: &Context<'_>,
  ) -> Result<(usize, Vec<Pair>), ReadlineError> {
    self.completer.complete(line, pos, ctx)
  }
}

impl Hinter for DmcHelper {
  type Hint = String;

  fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
    self.hinter.hint(line, pos, ctx)
  }
}

impl Highlighter for DmcHelper {
  fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
    &'s self,
    prompt: &'p str,
    default: bool,
  ) -> Cow<'b, str> {
    if default {
      Borrowed(&self.colored_prompt)
    } else {
      Borrowed(prompt)
    }
  }

  fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
    Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
  }

  fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
    self.highlighter.highlight(line, pos)
  }

  fn highlight_char(&self, line: &str, pos: usize) -> bool {
    self.highlighter.highlight_char(line, pos)
  }
}

impl Validator for DmcHelper {
  fn validate(
    &self,
    ctx: &mut validate::ValidationContext,
  ) -> rustyline::Result<validate::ValidationResult> {
    self.validator.validate(ctx)
  }

  fn validate_while_typing(&self) -> bool {
    self.validator.validate_while_typing()
  }
}

pub fn run() -> rustyline::Result<()> {
  let config = Config::builder()
    .history_ignore_space(true)
    .completion_type(CompletionType::List)
    .edit_mode(EditMode::Emacs)
    .output_stream(OutputStreamType::Stdout)
    .build();
  let h = DmcHelper {
    completer: FilenameCompleter::new(),
    highlighter: MatchingBracketHighlighter::new(),
    hinter: HistoryHinter {},
    colored_prompt: "".to_owned(),
    validator: MatchingBracketValidator::new(),
  };
  let mut rl = Editor::with_config(config);
  rl.set_helper(Some(h));
  rl.bind_sequence(KeyEvent::ctrl('N'), Cmd::HistorySearchForward);
  rl.bind_sequence(KeyEvent::ctrl('P'), Cmd::HistorySearchBackward);

  let history_path = match env::var("SHED") {
    Ok(path) => {
      let mut path = PathBuf::from(path);
      path.push("data");
      path.push("repl");
      path.push("dmc_history.txt");
      path
    }
    Err(_) => ".dmc_history.txt".into(),
  };

  if rl.load_history(&history_path).is_err() {
    println!("No previous history.");
  }
  let mut count = 1;

  loop {
    let p = format!("{} >> ", count);
    rl.helper_mut().expect("No helper").colored_prompt = format!("\x1b[1;32m{}\x1b[0m", p);
    let readline = rl.readline(&p);
    match readline {
      Ok(line) => {
        rl.add_history_entry(line.as_str()); // writes to mem buffer (i think?)
        println!("{} ::>> {}", &count, line);
      }
      Err(ReadlineError::Interrupted) => {
        println!("CTRL-C");
        break;
      }
      Err(ReadlineError::Eof) => {
        println!("CTRL-D");
        break;
      }
      Err(err) => {
        println!("Error: {:?}", err);
        break;
      }
    }
    count += 1;
  }

  rl.append_history(&history_path)?;

  Ok(())
}

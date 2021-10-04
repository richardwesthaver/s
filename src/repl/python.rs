//! RustPython functions and interpreter
use rlib::kala::cmd::input::rustyline;
use rlib::logger::log::{debug, error, warn};
use rustpython_vm::builtins::{PyDictRef, PyStrRef};
use rustpython_vm::{
  builtins::PyInt, compile, exceptions::print_exception, match_class, scope::Scope, stdlib::sys,
  InitParameter, Interpreter, ItemProtocol, PyResult, PySettings, TypeProtocol, VirtualMachine,
};
use rustpython_vm::{function::ArgIterable, TryFromObject};
use std::env;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;

/// The main cli of the `rustpython` interpreter. This function will
/// exit with `process::exit()` based on the return code of the python
/// code ran through the cli.
pub fn run<F>(init: F, file: Option<&str>, cmd: Option<&str>, module: Option<&str>) -> !
where
  F: FnOnce(&mut VirtualMachine),
{
  let settings = create_settings(&file, &cmd, &module);
  let init_param = InitParameter::External;

  let interp = Interpreter::new_with_init(settings, |vm| {
    init(vm);
    init_param
  });

  let exitcode = interp.enter(move |vm| {
    let res = run_rustpython(vm, &file, &cmd, &module);
    flush_std(vm);
    // See if any exception leaked out:
    let exitcode = match res {
      Ok(()) => 0,
      Err(err) if err.isinstance(&vm.ctx.exceptions.system_exit) => {
        let args = err.args();
        match args.as_slice() {
          [] => 0,
          [arg] => match_class!(match arg {
            ref i @ PyInt => {
              use num_traits::cast::ToPrimitive;
              i.as_bigint().to_i32().unwrap_or(0)
            }
            arg => {
              if vm.is_none(arg) {
                0
              } else {
                if let Ok(s) = vm.to_str(arg) {
                  eprintln!("{}", s);
                }
                1
              }
            }
          }),
          _ => {
            if let Ok(r) = vm.to_repr(args.as_object()) {
              eprintln!("{}", r);
            }
            1
          }
        }
      }
      Err(err) => {
        print_exception(vm, err);
        1
      }
    };

    let _ = vm.run_atexit_funcs();

    flush_std(vm);

    exitcode
  });

  process::exit(exitcode)
}

fn flush_std(vm: &VirtualMachine) {
  if let Ok(stdout) = sys::get_stdout(vm) {
    let _ = vm.call_method(&stdout, "flush", ());
  }
  if let Ok(stderr) = sys::get_stderr(vm) {
    let _ = vm.call_method(&stderr, "flush", ());
  }
}

/// Create settings by examining command line arguments and environment
/// variables.
/// refer to https://github.com/RustPython/RustPython/blob/ec5b6b4e8783b211344bce4b5cb0ae09a1572ec3/vm/src/vm.rs#L137 for PySettings fields
fn create_settings(file: &Option<&str>, cmd: &Option<&str>, module: &Option<&str>) -> PySettings {
  let mut settings = PySettings {
    interactive: !cmd.is_some() && !file.is_some() && !module.is_some(),
    ..Default::default()
  };

  // add the current directory to sys.path
  settings.path_list.push("".to_owned());

  // BUILDTIME_RUSTPYTHONPATH should be set when distributing
  if let Some(paths) = option_env!("BUILDTIME_RUSTPYTHONPATH") {
    settings
      .path_list
      .extend(std::env::split_paths(paths).map(|path| path.into_os_string().into_string().unwrap()))
  } else {
    settings.path_list.push(pylib::LIB_PATH.to_owned());
  }

  settings.path_list.extend(get_paths("RUSTPYTHONPATH"));
  settings.path_list.extend(get_paths("PYTHONPATH"));

  if let Ok(value) = get_env_var_value("PYTHONOPTIMIZE") {
    settings.optimize = value;
  }

  if let Ok(value) = get_env_var_value("PYTHONVERBOSE") {
    settings.verbose = value;
  }

  let argv = if let Some(script) = file {
    script
      .split_whitespace()
      .map(ToOwned::to_owned)
      .collect::<Vec<String>>()
  } else if let Some(command) = cmd {
    std::iter::once("-c".to_owned())
      .chain(command.split_whitespace().map(ToOwned::to_owned))
      .collect::<Vec<String>>()
  } else if let Some(module) = module {
    std::iter::once("PLACEHOLDER".to_owned())
      .chain(module.split_whitespace().skip(1).map(ToOwned::to_owned))
      .collect::<Vec<String>>()
  } else {
    vec!["".to_string()]
  };

  let hash_seed = match env::var("PYTHONHASHSEED") {
    Ok(s) if s == "random" => Some(None),
    Ok(s) => s.parse::<u32>().ok().map(Some),
    Err(_) => Some(None),
  };
  settings.hash_seed = hash_seed.unwrap_or_else(|| {
        error!("Fatal Python init error: PYTHONHASHSEED must be \"random\" or an integer in range [0; 4294967295]");
        process::exit(1)
    });

  settings.argv = argv;

  settings
}

/// Get environment variable and turn it into integer.
fn get_env_var_value(name: &str) -> Result<u8, std::env::VarError> {
  env::var(name).map(|value| {
    if let Ok(value) = u8::from_str(&value) {
      value
    } else {
      1
    }
  })
}

/// Helper function to retrieve a sequence of paths from an environment
/// variable.
fn get_paths(env_variable_name: &str) -> impl Iterator<Item = String> + '_ {
  env::var_os(env_variable_name)
    .into_iter()
    .flat_map(move |paths| {
      env::split_paths(&paths)
        .map(|path| {
          path
            .into_os_string()
            .into_string()
            .unwrap_or_else(|_| panic!("{} isn't valid unicode", env_variable_name))
        })
        .collect::<Vec<_>>()
    })
}

fn run_rustpython(
  vm: &VirtualMachine,
  file: &Option<&str>,
  cmd: &Option<&str>,
  module: &Option<&str>,
) -> PyResult<()> {
  let scope = vm.new_scope_with_builtins();
  let main_module = vm.new_module("__main__", scope.globals.clone(), None);
  main_module
    .dict()
    .and_then(|d| {
      d.set_item(
        "__annotations__",
        vm.ctx.new_dict().as_object().to_owned(),
        vm,
      )
      .ok()
    })
    .expect("Failed to initialize __main__.__annotations__");

  vm.get_attribute(vm.sys_module.clone(), "modules")?
    .set_item("__main__", main_module, vm)?;

  let site_result = vm.import("site", None, 0);

  if site_result.is_err() {
    warn!(
      "Failed to import site, consider adding the Lib directory to your RUSTPYTHONPATH \
             environment variable",
    );
  }

  // Figure out if a -c option was given:
  if let Some(command) = cmd {
    match command.as_ref() {
      "install_pip" => {
        let get_getpip = rustpython_vm::py_compile!(
          source = r#"\
__import__("io").TextIOWrapper(
    __import__("urllib.request").request.urlopen("https://bootstrap.pypa.io/get-pip.py")
).read()
"#,
          mode = "eval"
        );
        eprintln!("downloading get-pip.py...");
        let getpip_code = vm.run_code_obj(vm.new_code_object(get_getpip), scope.clone())?;
        let getpip_code: rustpython_vm::builtins::PyStrRef = getpip_code
          .downcast()
          .expect("TextIOWrapper.read() should return str");
        eprintln!("running get-pip.py...");
        _run_string(vm, scope, getpip_code.as_str(), "get-pip.py".to_owned())?;
      }
      _ => {
        run_command(vm, scope, command.to_string())?;
      }
    }
  } else if let Some(filename) = file {
    run_script(vm, scope.clone(), filename)?;
  } else if let Some(module) = module {
    run_module(vm, module)?;
  } else {
    println!("Welcome to the magnificent RustPython interpreter \u{1f631} \u{1f596}");
    run_shell(vm, scope)?;
  }
  Ok(())
}

fn _run_string(vm: &VirtualMachine, scope: Scope, source: &str, source_path: String) -> PyResult {
  let code_obj = vm
    .compile(source, compile::Mode::Exec, source_path.clone())
    .map_err(|err| vm.new_syntax_error(&err))?;
  // trace!("Code object: {:?}", code_obj.borrow());
  scope
    .globals
    .set_item("__file__", vm.ctx.new_utf8_str(source_path), vm)?;
  vm.run_code_obj(code_obj, scope)
}

fn run_command(vm: &VirtualMachine, scope: Scope, source: String) -> PyResult<()> {
  debug!("Running command {}", source);
  _run_string(vm, scope, &source, "<stdin>".to_owned())?;
  Ok(())
}

fn run_module(vm: &VirtualMachine, module: &str) -> PyResult<()> {
  debug!("Running module {}", module);
  let runpy = vm.import("runpy", None, 0)?;
  let run_module_as_main = vm.get_attribute(runpy, "_run_module_as_main")?;
  vm.invoke(&run_module_as_main, (module,))?;
  Ok(())
}

fn run_script(vm: &VirtualMachine, scope: Scope, script_file: &str) -> PyResult<()> {
  debug!("Running file {}", script_file);
  let mut file_path = PathBuf::from(script_file);
  let file_meta = file_path.metadata().unwrap_or_else(|e| {
    error!("can't open file '{}': {}", file_path.display(), e);
    process::exit(1);
  });
  if file_meta.is_dir() {
    file_path.push("__main__.py");
    if !file_path.is_file() {
      error!("can't find '__main__' module in '{}'", file_path.display());
      process::exit(1);
    }
  }

  let dir = file_path.parent().unwrap().to_str().unwrap().to_owned();
  let sys_path = vm.get_attribute(vm.sys_module.clone(), "path").unwrap();
  vm.call_method(&sys_path, "insert", (0, dir))?;

  match std::fs::read_to_string(&file_path) {
    Ok(source) => {
      _run_string(vm, scope, &source, file_path.to_str().unwrap().to_owned())?;
    }
    Err(err) => {
      error!(
        "Failed reading file '{}': {:?}",
        file_path.to_str().unwrap(),
        err.kind()
      );
      process::exit(1);
    }
  }
  Ok(())
}

//
// --++--
//

use rustpython_parser::error::{LexicalErrorType, ParseErrorType};
use rustpython_vm::readline::{Readline, ReadlineResult};
use rustpython_vm::{
  builtins::PyBaseExceptionRef,
  compile::{CompileError, CompileErrorType},
};

enum ShellExecResult {
  Ok,
  PyErr(PyBaseExceptionRef),
  Continue,
}

fn shell_exec(vm: &VirtualMachine, source: &str, scope: Scope) -> ShellExecResult {
  match vm.compile(source, compile::Mode::Single, "<stdin>".to_owned()) {
    Ok(code) => match vm.run_code_obj(code, scope) {
      Ok(_val) => ShellExecResult::Ok,
      Err(err) => ShellExecResult::PyErr(err),
    },
    Err(CompileError {
      error: CompileErrorType::Parse(ParseErrorType::Lexical(LexicalErrorType::Eof)),
      ..
    })
    | Err(CompileError {
      error: CompileErrorType::Parse(ParseErrorType::Eof),
      ..
    }) => ShellExecResult::Continue,
    Err(err) => ShellExecResult::PyErr(vm.new_syntax_error(&err)),
  }
}

pub fn run_shell(vm: &VirtualMachine, scope: Scope) -> PyResult<()> {
  let mut repl = Readline::new(ShellHelper::new(vm, scope.globals.clone()));
  let mut full_input = String::new();

  // Retrieve a `history_path_str` dependent on the OS
  let repl_history_path = match env::var("SHED") {
    Ok(path) => {
      let mut path = PathBuf::from(path);
      path.push("data");
      path.push("repl");
      path.push("py_history.txt");
      path
    }
    Err(_) => ".py_history.txt".into(),
  };

  if repl.load_history(&repl_history_path).is_err() {
    println!("No previous history.");
  }

  let mut continuing = false;

  loop {
    let prompt_name = if continuing { "ps2" } else { "ps1" };
    let prompt = vm
      .get_attribute(vm.sys_module.clone(), prompt_name)
      .and_then(|prompt| vm.to_str(&prompt));
    let prompt = match prompt {
      Ok(ref s) => s.as_str(),
      Err(_) => "",
    };
    let result = match repl.readline(prompt) {
      ReadlineResult::Line(line) => {
        debug!("You entered {:?}", line);

        repl.add_history_entry(line.trim_end()).unwrap();

        let stop_continuing = line.is_empty();

        if full_input.is_empty() {
          full_input = line;
        } else {
          full_input.push_str(&line);
        }
        full_input.push('\n');

        if continuing {
          if stop_continuing {
            continuing = false;
          } else {
            continue;
          }
        }

        match shell_exec(vm, &full_input, scope.clone()) {
          ShellExecResult::Ok => {
            full_input.clear();
            Ok(())
          }
          ShellExecResult::Continue => {
            continuing = true;
            Ok(())
          }
          ShellExecResult::PyErr(err) => {
            full_input.clear();
            Err(err)
          }
        }
      }
      ReadlineResult::Interrupt => {
        continuing = false;
        full_input.clear();
        let keyboard_interrupt =
          vm.new_exception_empty(vm.ctx.exceptions.keyboard_interrupt.clone());
        Err(keyboard_interrupt)
      }
      ReadlineResult::Eof => {
        break;
      }
      ReadlineResult::EncodingError => {
        eprintln!("Invalid UTF-8 entered");
        Ok(())
      }
      ReadlineResult::Other(err) => {
        eprintln!("Readline error: {:?}", err);
        break;
      }
      ReadlineResult::Io(err) => {
        eprintln!("IO error: {:?}", err);
        break;
      }
    };

    if let Err(exc) = result {
      if exc.isinstance(&vm.ctx.exceptions.system_exit) {
        repl.save_history(&repl_history_path).unwrap();
        return Err(exc);
      }
      print_exception(vm, exc);
    }
  }
  repl.save_history(&repl_history_path).unwrap();

  Ok(())
}

pub struct ShellHelper<'vm> {
  vm: &'vm VirtualMachine,
  globals: PyDictRef,
}

fn reverse_string(s: &mut String) {
  let rev = s.chars().rev().collect();
  *s = rev;
}

fn split_idents_on_dot(line: &str) -> Option<(usize, Vec<String>)> {
  let mut words = vec![String::new()];
  let mut startpos = 0;
  for (i, c) in line.chars().rev().enumerate() {
    match c {
      '.' => {
        // check for a double dot
        if i != 0 && words.last().map_or(false, |s| s.is_empty()) {
          return None;
        }
        reverse_string(words.last_mut().unwrap());
        if words.len() == 1 {
          startpos = line.len() - i;
        }
        words.push(String::new());
      }
      c if c.is_alphanumeric() || c == '_' => words.last_mut().unwrap().push(c),
      _ => {
        if words.len() == 1 {
          if words.last().unwrap().is_empty() {
            return None;
          }
          startpos = line.len() - i;
        }
        break;
      }
    }
  }
  if words == [String::new()] {
    return None;
  }
  reverse_string(words.last_mut().unwrap());
  words.reverse();

  Some((startpos, words))
}

impl<'vm> ShellHelper<'vm> {
  pub fn new(vm: &'vm VirtualMachine, globals: PyDictRef) -> Self {
    ShellHelper { vm, globals }
  }

  #[allow(clippy::type_complexity)]
  fn get_available_completions<'w>(
    &self,
    words: &'w [String],
  ) -> Option<(&'w str, impl Iterator<Item = PyResult<PyStrRef>> + 'vm)> {
    // the very first word and then all the ones after the dot
    let (first, rest) = words.split_first().unwrap();

    let str_iter_method = |obj, name| {
      let iter = self.vm.call_special_method(obj, name, ())?;
      ArgIterable::<PyStrRef>::try_from_object(self.vm, iter)?.iter(self.vm)
    };

    let (word_start, iter1, iter2) = if let Some((last, parents)) = rest.split_last() {
      // we need to get an attribute based off of the dir() of an object

      // last: the last word, could be empty if it ends with a dot
      // parents: the words before the dot

      let mut current = self
        .globals
        .get_item_option(first.as_str(), self.vm)
        .ok()??;

      for attr in parents {
        current = self.vm.get_attribute(current.clone(), attr.as_str()).ok()?;
      }

      let current_iter = str_iter_method(current, "__dir__").ok()?;

      (last, current_iter, None)
    } else {
      // we need to get a variable based off of globals/builtins

      let globals = str_iter_method(self.globals.as_object().clone(), "keys").ok()?;
      let builtins = str_iter_method(self.vm.builtins.clone(), "__dir__").ok()?;
      (first, globals, Some(builtins))
    };
    Some((word_start, iter1.chain(iter2.into_iter().flatten())))
  }

  fn complete_opt(&self, line: &str) -> Option<(usize, Vec<String>)> {
    let (startpos, words) = split_idents_on_dot(line)?;

    let (word_start, iter) = self.get_available_completions(&words)?;

    let all_completions = iter
      .filter(|res| {
        res
          .as_ref()
          .ok()
          .map_or(true, |s| s.as_str().starts_with(word_start))
      })
      .collect::<Result<Vec<_>, _>>()
      .ok()?;
    let mut completions = if word_start.starts_with('_') {
      // if they're already looking for something starting with a '_', just give
      // them all the completions
      all_completions
    } else {
      // only the completions that don't start with a '_'
      let no_underscore = all_completions
        .iter()
        .cloned()
        .filter(|s| !s.as_str().starts_with('_'))
        .collect::<Vec<_>>();

      // if there are only completions that start with a '_', give them all of the
      // completions, otherwise only the ones that don't start with '_'
      if no_underscore.is_empty() {
        all_completions
      } else {
        no_underscore
      }
    };

    // sort the completions alphabetically
    completions.sort_by(|a, b| std::cmp::Ord::cmp(a.as_str(), b.as_str()));

    Some((
      startpos,
      completions
        .into_iter()
        .map(|s| s.as_str().to_owned())
        .collect(),
    ))
  }
}

cfg_if::cfg_if! {
    if #[cfg(not(target_os = "wasi"))] {
        use rustyline::{
            completion::Completer, highlight::Highlighter, hint::Hinter, validate::Validator, Context,
            Helper,
        };
        impl Completer for ShellHelper<'_> {
            type Candidate = String;

            fn complete(
                &self,
                line: &str,
                pos: usize,
                _ctx: &Context,
            ) -> rustyline::Result<(usize, Vec<String>)> {
                Ok(self
                    .complete_opt(&line[0..pos])
                    // as far as I can tell, there's no better way to do both completion
                    // and indentation (or even just indentation)
                    .unwrap_or_else(|| (pos, vec!["\t".to_owned()])))
            }
        }

        impl Hinter for ShellHelper<'_> {
            type Hint = String;
        }
        impl Highlighter for ShellHelper<'_> {}
        impl Validator for ShellHelper<'_> {}
        impl Helper for ShellHelper<'_> {}
    }
}

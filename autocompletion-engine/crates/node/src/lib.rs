#[macro_use]
extern crate napi_derive;

use autocompletion_engine_core::{AutocompletionEngine as Engine, Completions};

#[napi]
pub struct AutocompletionEngine {
  engine: Engine,
}

#[napi]
impl AutocompletionEngine {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      engine: Engine::new(),
    }
  }

  #[napi]
  pub fn get_all_completions_as_string(&mut self, files: Vec<String>) -> String {
    self.engine.get_all_completions_as_string(files)
  }

  #[napi]
  pub fn invalidate_cache(&mut self) {
    self.engine.invalidate_cache();
  }

  pub fn get_all_completions_for_files(&mut self, files: Vec<String>) -> &Completions {
    self.engine.get_all_completions_for_files(files)
  }
}

impl Default for AutocompletionEngine {
  fn default() -> Self {
    Self::new()
  }
}

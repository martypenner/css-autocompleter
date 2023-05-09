import test from "ava";

import { getCompletions } from "../dist/index.js";

test("get completions from test css file", (t) => {
  const completions = JSON.parse(getCompletions());
  t.deepEqual(completions, {
    'input[type="password"].focus': [],
    'input[type="email"].focus': [],
    'input[type="search"].focus': [],
    'input[type="number"].focus': [],
    'input[type="tel"].focus': [],
    'input[type="url"].focus': [],
    'input[type="text"].focus': [],
    "textarea.focus": [],
    ".focus": [],
    ".focused": [],

    ".input-contrast": [],
    ".focus.search-page-label": [],
    ".drag-and-drop": [],
    ".search-page-label": [],
    "input.input-contrast": [],
    ".wrapper": [],
  });
});

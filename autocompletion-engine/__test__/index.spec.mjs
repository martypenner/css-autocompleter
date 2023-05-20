import test from "ava";

import { getCompletionsAsString } from "../dist/index.js";

test("get completions from test css file", (t) => {
  const completions = JSON.parse(getCompletionsAsString());
  t.deepEqual(completions, {
    wrapper: [],
    focus: [],
    focused: [],
    "input-contrast": [],
    "search-page-label": [],
    "drag-and-drop": [],
    "search-page-label": [],
    "input-contrast": [],
  });
});

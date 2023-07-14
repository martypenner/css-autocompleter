import os from 'node:os';
import test from 'ava';
import { AutocompletionEngine } from '../index.js';

// TODO: moar tests
test('get completions from test css file', (t) => {
  const engine = new AutocompletionEngine();
  const completions = JSON.parse(
    engine.getAllCompletionsAsString(['./crates/node/__test__/test.atom.io.css'])
  );
  // Strip surrounding whitespace from each rule set.
  for (const [i, entry] of completions.entries()) {
    const [, ruleSet] = entry;
    completions[i][1] = ruleSet.trim();
  }

  const expected = [
    [
      'drag-and-drop',
      `input[type="text"].focus,\n#adv_code_search .focus.search-page-label,\ninput[type="text"]:focus,\n.focused .drag-and-drop,\n#adv_code_search .search-page-label:focus,\ninput[type="password"].focus,\ninput[type="password"]:focus,\ninput[type="email"].focus,\ninput[type="email"]:focus,\ninput[type="number"].focus,\ninput[type="number"]:focus,\ninput[type="tel"].focus,\ninput[type="tel"]:focus,\ninput[type="url"].focus,\ninput[type="url"]:focus,\ninput[type="search"].focus,\ninput[type="search"]:focus,\ntextarea.focus,\ntextarea:focus {\n  border-color: #51a7e8;\n  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.075),\n    0 0 5px rgba(81, 167, 232, 0.5);\n}`,
    ],
    [
      'focus',
      `input[type="text"].focus,\n#adv_code_search .focus.search-page-label,\ninput[type="text"]:focus,\n.focused .drag-and-drop,\n#adv_code_search .search-page-label:focus,\ninput[type="password"].focus,\ninput[type="password"]:focus,\ninput[type="email"].focus,\ninput[type="email"]:focus,\ninput[type="number"].focus,\ninput[type="number"]:focus,\ninput[type="tel"].focus,\ninput[type="tel"]:focus,\ninput[type="url"].focus,\ninput[type="url"]:focus,\ninput[type="search"].focus,\ninput[type="search"]:focus,\ntextarea.focus,\ntextarea:focus {\n  border-color: #51a7e8;\n  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.075),\n    0 0 5px rgba(81, 167, 232, 0.5);\n}`,
    ],
    [
      'focused',
      `input[type="text"].focus,\n#adv_code_search .focus.search-page-label,\ninput[type="text"]:focus,\n.focused .drag-and-drop,\n#adv_code_search .search-page-label:focus,\ninput[type="password"].focus,\ninput[type="password"]:focus,\ninput[type="email"].focus,\ninput[type="email"]:focus,\ninput[type="number"].focus,\ninput[type="number"]:focus,\ninput[type="tel"].focus,\ninput[type="tel"]:focus,\ninput[type="url"].focus,\ninput[type="url"]:focus,\ninput[type="search"].focus,\ninput[type="search"]:focus,\ntextarea.focus,\ntextarea:focus {\n  border-color: #51a7e8;\n  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.075),\n    0 0 5px rgba(81, 167, 232, 0.5);\n}`,
    ],
    [
      'input-contrast',
      `input.input-contrast,\n.input-contrast {\n  background-color: #fafafa;\n}\n\ninput.input-contrast:focus,\n.input-contrast:focus {\n  background-color: #fff;\n}`,
    ],
    [
      'search-page-label',
      `input[type="text"],\n#adv_code_search .search-page-label,\ninput[type="password"],\ninput[type="email"],\ninput[type="number"],\ninput[type="tel"],\ninput[type="url"],\ninput[type="search"],\ntextarea {\n  min-height: 34px;\n  padding: 7px 8px;\n  font-size: 13px;\n  color: #333;\n  vertical-align: middle;\n  background-color: #fff;\n  background-repeat: no-repeat;\n  background-position: right center;\n  border: 1px solid #ccc;\n  border-radius: 3px;\n  outline: none;\n  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.075);\n}\n\ninput[type="text"].focus,\n#adv_code_search .focus.search-page-label,\ninput[type="text"]:focus,\n.focused .drag-and-drop,\n#adv_code_search .search-page-label:focus,\ninput[type="password"].focus,\ninput[type="password"]:focus,\ninput[type="email"].focus,\ninput[type="email"]:focus,\ninput[type="number"].focus,\ninput[type="number"]:focus,\ninput[type="tel"].focus,\ninput[type="tel"]:focus,\ninput[type="url"].focus,\ninput[type="url"]:focus,\ninput[type="search"].focus,\ninput[type="search"]:focus,\ntextarea.focus,\ntextarea:focus {\n  border-color: #51a7e8;\n  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.075),\n    0 0 5px rgba(81, 167, 232, 0.5);\n}`,
    ],
    [
      'wrapper',
      '#peek .wrapper {\n  width: 860px !important;\n  padding: 0;\n}\n\n#peek2 .wrapper {\n  border: 1px solid red;\n}',
    ],
  ].map(([className, ruleSet]) => [
    className,
    // Replace newlines with OS-specific characters. Fixes windows test failures.
    ruleSet.replaceAll(/(?<!}|\n)\n/gi, os.EOL),
  ]);

  t.deepEqual(completions, expected);
});

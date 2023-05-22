import * as vscode from 'vscode';
import { getCompletionsAsString } from '../autocompletion-engine';

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {
  // Use the console to output diagnostic information (console.log) and errors (console.error)
  // This line of code will only be executed once when your extension is activated
  console.log('Congratulations, your extension "css-to-go" is now active!');

  // The command has been defined in the package.json file
  // Now provide the implementation of the command with registerCommand
  // The commandId parameter must match the command field in package.json
  const disposable = vscode.commands.registerCommand('css-to-go.helloWorld', () => {
    // The code you place here will be executed every time your command is executed
    // Display a message box to the user
    vscode.window.showInformationMessage('Hello World from css-to-go!');
  });

  const provider = vscode.languages.registerCompletionItemProvider(
    'html',
    {
      provideCompletionItems(document: vscode.TextDocument, position: vscode.Position) {
        // Get the entire line text and search for `class=""`. We only want to
        // trigger completions inside of that and nowhere else. I really wish we
        // didn't have to resort to a regex, but setting up embedded languages
        // and / or an entire language server seemed like overkill. Maybe we
        // will be bitten by a massive bug or a regex DOS attack and have to
        // rethink this, but it works for now.
        const line = document.lineAt(position).text;
        const classRegex = /class=["'][^"']*/giu;

        for (const match of line.matchAll(classRegex)) {
          // eslint-disable-next-line eqeqeq
          if (match.index == null) {
            continue;
          }

          const isWithinRange =
            position.character >= match.index &&
            position.character <= match.index + match[0].length;
          if (!isWithinRange) {
            return undefined;
          }
        }

        // TODO: cache this. On the other hand, tree-sitter claims to be able to
        // re-parse an entire file on every keystroke. Maybe it doesn't matter?
        // But we might have many CSS files to parse, and those probably won't
        // change much. So maybe we use a file watchers instead.
        const rawCompletions = Object.keys(JSON.parse(getCompletionsAsString()));
        const completions = rawCompletions.map(
          (completion) => new vscode.CompletionItem(completion, vscode.CompletionItemKind.Constant)
        );

        return completions;
      },
    },
    "'",
    '"',
    ' '
  );

  context.subscriptions.push(disposable, provider);
}

// This method is called when your extension is deactivated
export function deactivate() {}

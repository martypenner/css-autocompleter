import * as vscode from 'vscode';
import { getCompletionsForFilesAsString } from '../autocompletion-engine';

const configKey = 'css-to-go.filesList';

export function activate(context: vscode.ExtensionContext) {
  let config = vscode.workspace.getConfiguration();
  let listOfFilesToParse = getFilesToParse();

  let completions: vscode.CompletionItem[] | null = null;

  const disposable = vscode.commands.registerCommand(
    'css-to-go.addCssToAutocomplete',
    async (file) => {
      let newList = ((config.get(configKey) ?? []) as string[]).concat(file.path);
      await config.update(configKey, newList);
    }
  );
  // TODO : on workspace change, reload the completions
  // TODO : on files change, reload the completions
  vscode.workspace.onDidChangeConfiguration((event) => {
    if (!event.affectsConfiguration(configKey)) {
      return;
    }

    completions = null;
    listOfFilesToParse = getFilesToParse();
  });

  const provider = vscode.languages.registerCompletionItemProvider(
    'html',
    {
      provideCompletionItems(document: vscode.TextDocument, position: vscode.Position) {
        if (listOfFilesToParse.size === 0) {
          return;
        }

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

        // eslint-disable-next-line eqeqeq
        if (completions == null) {
          completions = [];

          for (const file of listOfFilesToParse.values()) {
            const rawCompletions: Completions = JSON.parse(getCompletionsForFilesAsString(file));
            completions = completions.concat(
              Object.entries(rawCompletions).map(([className, ruleSet]) => {
                const completion = new vscode.CompletionItem(
                  className,
                  vscode.CompletionItemKind.Constant
                );
                completion.documentation = getDocsForRuleSet(ruleSet);

                return completion;
              })
            );
          }
        }

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

function getFilesToParse(): Set<string> {
  let config = vscode.workspace.getConfiguration();
  let listOfFilesToParse = (config.get(configKey) ?? []) as string[];

  return new Set(listOfFilesToParse);
}

function getDocsForRuleSet(ruleSet: string): vscode.MarkdownString {
  return new vscode.MarkdownString(`
${CSS_MARKER}css
${ruleSet}
${CSS_MARKER}
`);
}

const CSS_MARKER = '```';

type Completions = {
  [className: string]: string;
};

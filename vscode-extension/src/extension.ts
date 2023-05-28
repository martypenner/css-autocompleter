import * as vscode from 'vscode';
import { AutocompletionEngine } from '../autocompletion-engine';

const filesConfigKey = 'css-to-go.filesList';

export function activate(context: vscode.ExtensionContext) {
  const config = vscode.workspace.getConfiguration();
  const engine = new AutocompletionEngine();

  // TODO: filter out files not in workspace
  let filesToParse = Array.from(new Set(getFilesToParseFromConfig(config)));

  const disposable = vscode.commands.registerCommand(
    'css-to-go.addCssToAutocomplete',
    async (file) => {
      const newList = Array.from(new Set(getFilesToParseFromConfig(config).concat(file.path)));
      await config.update(filesConfigKey, newList);
    }
  );
  // TODO : on workspace change, reload the completions
  // TODO : on files change, reload the completions
  vscode.workspace.onDidChangeConfiguration((event) => {
    if (!event.affectsConfiguration(filesConfigKey)) {
      return;
    }

    // TODO: this isn't picking up the changes properly
    // filesToParse = getFilesToParseFromConfig(config);
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

        // TODO: don't provide completions for classes already in the class list
        const rawCompletions: Completions = JSON.parse(
          engine.getAllCompletionsAsString(filesToParse)
        );
        const completions = rawCompletions.map(([className, ruleSet]) => {
          const completion = new vscode.CompletionItem(
            className,
            vscode.CompletionItemKind.Constant
          );
          completion.documentation = getDocsForRuleSet(ruleSet);

          return completion;
        });

        return completions;
      },
    },
    "'",
    '"',
    ' '
  );

  context.subscriptions.push(disposable, provider);
}

export function deactivate() {}

type ClassName = string;
type RuleSet = string;
type Completions = Array<[ClassName, RuleSet]>;

const CSS_MARKER = '```';

// TODO:
function setupFileWatchers() {
  // let filesAndTheirWatchers = new Map();
  // for (const file of filesToParse) {
  //   if (filesAndTheirWatchers.has(file)) {
  //     console.warn(`Duplicate file found in file list: ${file}`);
  //     continue;
  //   }
  //
  //   const watcher = vscode.workspace.createFileSystemWatcher(
  //     new vscode.RelativePattern(path.dirname(file), path.basename(file))
  //   );
  //
  //   watcher.onDidChange((uri) => {});
  //   watcher.onDidCreate((uri) => {});
  //   watcher.onDidDelete((uri) => {
  //     watcher.dispose();
  //   });
  //
  //   filesAndTheirWatchers.set(file, watcher);
  // }
}

// TODO: guard against invalid values
function getFilesToParseFromConfig(config: vscode.WorkspaceConfiguration) {
  let files = config.get(filesConfigKey, []) as string[];
  if (!Array.isArray(files)) {
    console.warn(`Invalid config value for ${filesConfigKey}. Expected an array of strings.`);
    files = [];
  }

  return files;
}

function getDocsForRuleSet(ruleSet: string): vscode.MarkdownString {
  return new vscode.MarkdownString(`
${CSS_MARKER}css
${ruleSet}
${CSS_MARKER}
`);
}

// function getWatcherForFiles(files: string[]): Map<string, vscode.FileSystemWatcher> {}

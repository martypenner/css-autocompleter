import * as vscode from 'vscode';
import { getCompletionsForFilesAsString } from '../autocompletion-engine';

const configKey = 'css-to-go.filesList';

export function activate(context: vscode.ExtensionContext) {
  const config = vscode.workspace.getConfiguration();
  const engine = new Autocompletions(config);

  const initialFiles = Array.from(new Set(getFilesToParseFromConfig(config)));
  engine.refreshCompletionForFiles(initialFiles);

  const disposable = vscode.commands.registerCommand(
    'css-to-go.addCssToAutocomplete',
    async (file) => {
      const newList = Array.from(new Set(getFilesToParseFromConfig(config).concat(file.path)));
      await config.update(configKey, newList);
    }
  );
  // TODO : on workspace change, reload the completions
  // TODO : on files change, reload the completions
  vscode.workspace.onDidChangeConfiguration((event) => {
    if (!event.affectsConfiguration(configKey)) {
      return;
    }

    // TODO: this isn't picking up the changes properly
    const filesToParse = getFilesToParseFromConfig(config);
    engine.setFilesToParse(filesToParse);
  });

  const provider = vscode.languages.registerCompletionItemProvider(
    'html',
    {
      provideCompletionItems(document: vscode.TextDocument, position: vscode.Position) {
        // TODO: don't run if there are no completions

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

        // TODO: get initial completions
        const completions = engine.cachedCompletions;

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

type FilePath = string;
type Completion = string;
type Completions = {
  [className: string]: Completion;
};

// TODO: we're currently structured around classnames. This is nice for looping and
// creating the final completions list, but makes it harder to invalidate the cache
// for a single file. Need to rethink this.
class Autocompletions {
  private _config = vscode.workspace.getConfiguration();
  private _cachedCompletions: vscode.CompletionItem[] = [];

  constructor(config: vscode.WorkspaceConfiguration) {
    this._config = config;
  }

  get cachedCompletions() {
    return this._cachedCompletions;
  }

  // Note that currently, this resets ALL completions.
  // TODO: don't do that ;). Will be more possible once we restructure around file maps.
  refreshCompletionForFiles(files: string[]) {
    const rawCompletions: Completions = JSON.parse(getCompletionsForFilesAsString(files));
    this._cachedCompletions = Object.entries(rawCompletions).map(([className, ruleSet]) => {
      const completion = new vscode.CompletionItem(className, vscode.CompletionItemKind.Constant);
      completion.documentation = this.getDocsForRuleSet(ruleSet);

      return completion;
    });
  }

  setFilesToParse(filesToParse: string[]) {
    // Remove completions that are no longer present in the file list.
    // TODO: I don't like this double conversion
    const files = Array.from(new Set(filesToParse));
    this.refreshCompletionForFiles(files);
  }

  // TODO:
  private setupFileWatchers() {
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

  private getDocsForRuleSet(ruleSet: string): vscode.MarkdownString {
    return new vscode.MarkdownString(`
${CSS_MARKER}css
${ruleSet}
${CSS_MARKER}
`);
  }
}

// TODO: gaurd against invalid values
function getFilesToParseFromConfig(config: vscode.WorkspaceConfiguration) {
  return config.get(configKey, []) as string[];
}

// function getWatcherForFiles(files: string[]): Map<string, vscode.FileSystemWatcher> {}

const CSS_MARKER = '```';

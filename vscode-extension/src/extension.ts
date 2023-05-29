import * as vscode from 'vscode';
import * as path from 'node:path';
import { AutocompletionEngine } from '../autocompletion-engine';

const filesConfigKey = 'css-to-go.filesList';

export function activate(context: vscode.ExtensionContext) {
  const config = vscode.workspace.getConfiguration();
  const engine = new AutocompletionEngine();

  let filesAndWatchers = getFilesToParseFromConfig(config, engine);
  if (vscode.workspace.workspaceFolders === undefined) {
    return;
  }

  const disposable = vscode.commands.registerCommand(
    'css-to-go.addCssToAutocomplete',
    async (file) => {
      const newList = Array.from(getFilesToParseFromConfig(config, engine).keys()).concat(
        file.path
      );
      try {
        await config.update(filesConfigKey, newList, true);
      } catch (error) {
        vscode.window.showErrorMessage(
          `We couldn't update your configuration for some reason. Please see the debug logs for more info.`
        );
        console.error(`We couldn't update your configuration for the following reason: ${error}`);
      }
    }
  );

  vscode.workspace.onDidChangeWorkspaceFolders(() => {
    cleanupFileWatchers(filesAndWatchers);
    // Re-filter the files to parse.
    filesAndWatchers = getFilesToParseFromConfig(config, engine);
  });

  vscode.workspace.onDidChangeConfiguration((event) => {
    if (!event.affectsConfiguration(filesConfigKey)) {
      return;
    }

    // TODO: these don't work
    // cleanupFileWatchers(filesAndWatchers);
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
          engine.getAllCompletionsAsString(Array.from(filesAndWatchers.keys()))
        );
        const completions = rawCompletions.map(([className, ruleSet]) => {
          const completion = new vscode.CompletionItem(
            className,
            vscode.CompletionItemKind.Constant
          );
          completion.documentation = new vscode.MarkdownString(
            ['```css', ruleSet, '```'].join('\n')
          );

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

function cleanupFileWatchers(filesAndWatchers: Map<string, vscode.FileSystemWatcher>) {
  for (const watcher of filesAndWatchers.values()) {
    watcher.dispose();
  }
}

function getFilesToParseFromConfig(
  config: vscode.WorkspaceConfiguration,
  engine: AutocompletionEngine
): Map<string, vscode.FileSystemWatcher> {
  let files = config.get(filesConfigKey, []) as string[];
  if (!Array.isArray(files)) {
    vscode.window.showErrorMessage(
      `Found an invalid config value for ${filesConfigKey}. Expected an array of strings. Falling back to [].`
    );
    files = [];
  }

  const workspaceFolderNames =
    vscode.workspace.workspaceFolders?.map((folder) => folder.uri.path) ?? [];
  files = files.filter(
    (file) =>
      workspaceFolderNames.find((folder) => path.dirname(file).startsWith(folder)) !== undefined
  );

  const filesAndWatchers = new Map(
    files.map((file) => {
      const watcher = vscode.workspace.createFileSystemWatcher(
        new vscode.RelativePattern(path.dirname(file), path.basename(file))
      );
      // This one gets fired when a file in our list does not exist until now.
      watcher.onDidCreate(() => {
        engine.invalidateCache();
      });
      watcher.onDidChange(() => {
        engine.invalidateCache();
      });
      watcher.onDidDelete(() => {
        watcher.dispose();
        engine.invalidateCache();
      });

      return [file, watcher];
    })
  );

  return filesAndWatchers;
}

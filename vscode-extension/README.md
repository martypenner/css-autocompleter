## Features

Autocomplete CSS class names based on CSS files in your workspace.

![autocomplete](media/2023-07-04.png)

## Extension Settings

This extension contributes the following settings:

- `css-to-go.filesList`: The list of CSS files that will have their classes made available for autocompletion. Autocomplete will only be provided for files within the workspace you currently have open.
- `css-to-go.htmlLanguages`: A list of HTML-based languages where suggestions are enabled.
- `css-to-go.javascriptLanguages`: A list of JavaScript-based languages where suggestions are enabled.

## Known Issues

Adding a CSS file to the autocomplete list doesn't always persist the setting. Probably something to do with workspace vs. global settings.

## Release Notes

### 1.0.0

Initial release

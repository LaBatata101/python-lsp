import {
    languages,
    workspace,
    EventEmitter,
    ExtensionContext,
    window,
    InlayHintsProvider,
    TextDocument,
    CancellationToken,
    Range,
    InlayHint,
    TextDocumentChangeEvent,
    ProviderResult,
    commands,
    WorkspaceEdit,
    TextEdit,
    Selection,
    Uri,
} from "vscode";

import {
    Disposable,
    Executable,
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;
// type a = Parameters<>;

export async function activate(context: ExtensionContext) {
    // let disposable = commands.registerCommand("helloworld.helloWorld", async uri => {
    //   // The code you place here will be executed every time your command is executed
    //   // Display a message box to the user
    //   const url = Uri.parse('/home/victor/Documents/test-dir/nrs/another.nrs')
    //   let document = await workspace.openTextDocument(uri);
    //   await window.showTextDocument(document);
    //
    //   // console.log(uri)
    //   window.activeTextEditor.document
    //   let editor = window.activeTextEditor;
    //   let range = new Range(1, 1, 1, 1)
    //   editor.selection = new Selection(range.start, range.end);
    // });

    // context.subscriptions.push(disposable);

    // console.log(`found LSP server binary ${process.env.SERVER_PATH}`);
    const traceOutputChannel = window.createOutputChannel("Python Language Server trace");
    const command = process.env.SERVER_PATH || "python-language-server";
    traceOutputChannel.appendLine(`found LSP server binary ${command}`);
    const run: Executable = {
        command,
        options: {
            env: {
                ...process.env,
                // eslint-disable-next-line @typescript-eslint/naming-convention
                RUST_LOG: "debug",
            },
        },
    };
    const serverOptions: ServerOptions = {
        run,
        debug: run,
    };
    // If the extension is launched in debug mode then the debug server options are used
    // Otherwise the run options are used
    // Options to control the language client
    let clientOptions: LanguageClientOptions = {
        // Register the server for plain text documents
        documentSelector: [{ scheme: "file", language: "python" }],
        synchronize: {
            // Notify the server about file changes to '.clientrc files contained in the workspace
            fileEvents: workspace.createFileSystemWatcher("**/.clientrc"),
        },
        traceOutputChannel,
    };

    // Create the language client and start the client.
    client = new LanguageClient(
        "python-language-server",
        "python language server",
        serverOptions,
        clientOptions,
    );
    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

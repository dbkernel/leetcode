const path = require("path");
const vscode = require("vscode");

/**
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
  context.subscriptions.push(
    vscode.commands.registerCommand("leetcode.debugCurrentFile", () =>
      startCurrent("debug")
    ),
    vscode.commands.registerCommand("leetcode.runCurrentFile", () =>
      startCurrent("run")
    )
  );
}

/**
 * @param {"debug" | "run"} mode
 */
async function startCurrent(mode) {
  const editor = vscode.window.activeTextEditor;
  if (!editor) {
    vscode.window.showErrorMessage("没有打开的编辑器");
    return;
  }

  const folder = vscode.workspace.getWorkspaceFolder(editor.document.uri);
  if (!folder) {
    vscode.window.showErrorMessage("当前文件不在工作区内");
    return;
  }

  const file = editor.document.uri.fsPath;
  const lang = editor.document.languageId;
  const noDebug = mode === "run";

  if (lang === "go") {
    const ok = await vscode.debug.startDebugging(
      folder,
      {
        name: "Debug Go (current file)",
        type: "go",
        request: "launch",
        mode: "debug",
        program: file,
        cwd: path.dirname(file),
        console: "integratedTerminal",
        dlvFlags: ["--check-go-version=false"],
      },
      { noDebug }
    );
    if (!ok) {
      vscode.window.showErrorMessage(
        "Go 调试启动失败。请确认已安装 Go 扩展与 dlv。"
      );
    }
    return;
  }

  if (lang === "cpp" || lang === "c") {
    const built = await runBuildTask(folder, file);
    if (!built) {
      return;
    }

    const program = path.join(
      path.dirname(file),
      ".build",
      path.basename(file, path.extname(file))
    );

    const ok = await vscode.debug.startDebugging(
      folder,
      {
        name: "Debug C++ (current file)",
        type: "cppdbg",
        request: "launch",
        program,
        args: [],
        stopAtEntry: false,
        cwd: path.dirname(file),
        environment: [],
        externalConsole: false,
        MIMode: "gdb",
        miDebuggerPath: "/usr/bin/gdb",
        setupCommands: [
          {
            description: "Enable pretty-printing for gdb",
            text: "-enable-pretty-printing",
            ignoreFailures: true,
          },
        ],
      },
      { noDebug }
    );
    if (!ok) {
      vscode.window.showErrorMessage(
        "C++ 调试启动失败。请确认已安装 C/C++ 扩展与 gdb。"
      );
    }
    return;
  }

  vscode.window.showWarningMessage(
    `当前语言 ${lang} 不在自动调试范围内，请手动选择 launch 配置。`
  );
}

/**
 * @param {vscode.WorkspaceFolder} folder
 * @param {string} file
 */
async function runBuildTask(folder, file) {
  const tasks = await vscode.tasks.fetchTasks();
  const task = tasks.find((t) => t.name === "build current C++ file");
  if (!task) {
    vscode.window.showErrorMessage('找不到任务 "build current C++ file"');
    return false;
  }

  // Ensure ${file} resolves to the active C/C++ file for the task.
  await vscode.window.showTextDocument(vscode.Uri.file(file), {
    preview: false,
    preserveFocus: true,
  });

  const execution = await vscode.tasks.executeTask(task);
  const success = await new Promise((resolve) => {
    const disposable = vscode.tasks.onDidEndTaskProcess((e) => {
      if (e.execution === execution) {
        disposable.dispose();
        resolve(e.exitCode === 0);
      }
    });
  });

  if (!success) {
    vscode.window.showErrorMessage("C++ 编译失败，已取消调试。");
  }
  return success;
}

function deactivate() {}

module.exports = {
  activate,
  deactivate,
};


using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'shc' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'shc'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'shc' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'initialize the shed')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'edit all the things')
            [CompletionResult]::new('krypt', 'krypt', [CompletionResultType]::ParameterValue, 'blackbox')
            [CompletionResult]::new('clean', 'clean', [CompletionResultType]::ParameterValue, 'clean stuff up')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'print basic info')
            [CompletionResult]::new('pack', 'pack', [CompletionResultType]::ParameterValue, 'create packages from file or directory')
            [CompletionResult]::new('unpack', 'unpack', [CompletionResultType]::ParameterValue, 'unpack .z or .tz files')
            [CompletionResult]::new('download', 'download', [CompletionResultType]::ParameterValue, 'fetch resources')
            [CompletionResult]::new('pull', 'pull', [CompletionResultType]::ParameterValue, 'fetch resources')
            [CompletionResult]::new('push', 'push', [CompletionResultType]::ParameterValue, 'commit changes to upstream')
            [CompletionResult]::new('store', 'store', [CompletionResultType]::ParameterValue, 'shared block storage')
            [CompletionResult]::new('stash', 'stash', [CompletionResultType]::ParameterValue, 'local storage')
            [CompletionResult]::new('serve', 'serve', [CompletionResultType]::ParameterValue, 'network services')
            [CompletionResult]::new('build', 'build', [CompletionResultType]::ParameterValue, 'build scripts')
            [CompletionResult]::new('x', 'x', [CompletionResultType]::ParameterValue, 'do things with runtimes')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'shc;init' {
            [CompletionResult]::new('--fmt', 'fmt', [CompletionResultType]::ParameterName, 'config format')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--force', 'force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--db', 'db', [CompletionResultType]::ParameterName, 'db')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            break
        }
        'shc;edit' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            break
        }
        'shc;krypt' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            break
        }
        'shc;clean' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            break
        }
        'shc;status' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'system info')
            [CompletionResult]::new('--sys', 'sys', [CompletionResultType]::ParameterName, 'system info')
            [CompletionResult]::new('-i', 'i', [CompletionResultType]::ParameterName, 'my ip')
            [CompletionResult]::new('--ip', 'ip', [CompletionResultType]::ParameterName, 'my ip')
            [CompletionResult]::new('-u', 'u', [CompletionResultType]::ParameterName, 'usb devices')
            [CompletionResult]::new('--usb', 'usb', [CompletionResultType]::ParameterName, 'usb devices')
            [CompletionResult]::new('-m', 'm', [CompletionResultType]::ParameterName, 'midi devices')
            [CompletionResult]::new('--midi', 'midi', [CompletionResultType]::ParameterName, 'midi devices')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'weather report')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'show repo status')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'query remote for changes')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            break
        }
        'shc;pack' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            break
        }
        'shc;unpack' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'consume input package')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            break
        }
        'shc;download' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            break
        }
        'shc;pull' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            break
        }
        'shc;push' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            break
        }
        'shc;store' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            break
        }
        'shc;stash' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            break
        }
        'shc;serve' {
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'specify packages to serve')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            break
        }
        'shc;build' {
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'package to build')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            break
        }
        'shc;x' {
            [CompletionResult]::new('-x', 'x', [CompletionResultType]::ParameterName, 'execute a command')
            [CompletionResult]::new('-m', 'm', [CompletionResultType]::ParameterName, 'execute a module')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'execute a script')
            [CompletionResult]::new('-i', 'i', [CompletionResultType]::ParameterName, 'use the specified interpreter (dialect)')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            break
        }
        'shc;help' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'override configuration values')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-?', '?', [CompletionResultType]::ParameterName, 'set the log level')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}

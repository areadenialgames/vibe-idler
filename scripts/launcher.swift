import Foundation
import AppKit

let bundle = Bundle.main
let macosPath = (bundle.bundlePath as NSString).appendingPathComponent("Contents/MacOS")
let resourcesPath = (bundle.bundlePath as NSString).appendingPathComponent("Contents/Resources")
let binaryPath = (macosPath as NSString).appendingPathComponent("vibe-idler-bin")

// Detect user's default terminal from the system's default handler for .command files
// Falls back to Terminal.app
func defaultTerminalBundleID() -> String {
    if let handler = LSCopyDefaultRoleHandlerForContentType(
        "public.unix-executable" as CFString, .shell)?.takeRetainedValue() {
        return handler as String
    }
    // Also check what handles .command files
    if let url = NSWorkspace.shared.urlForApplication(
        toOpen: URL(fileURLWithPath: "/tmp/test.command")) {
        if let bid = Bundle(url: url)?.bundleIdentifier {
            return bid
        }
    }
    return "com.apple.Terminal"
}

// Create a temporary .command file that runs our binary
// This works with any terminal emulator (Ghostty, Kitty, iTerm2, WezTerm, etc.)
let commandFile = NSTemporaryDirectory() + "vibe-idler-launch.command"
let script = """
#!/bin/bash
cd "\(resourcesPath)"
"\(binaryPath)"
exit
"""

try? script.write(toFile: commandFile, atomically: true, encoding: .utf8)

// Make it executable
let fm = FileManager.default
try? fm.setAttributes([.posixPermissions: 0o755], ofItemAtPath: commandFile)

// Open with the default terminal
NSWorkspace.shared.open(
    URL(fileURLWithPath: commandFile),
    configuration: NSWorkspace.OpenConfiguration()
) { _, error in
    if let error = error {
        NSLog("Launch error: \(error)")
    }
    DispatchQueue.main.asyncAfter(deadline: .now() + 2.0) {
        NSApplication.shared.terminate(nil)
    }
}

RunLoop.main.run(until: Date(timeIntervalSinceNow: 5.0))

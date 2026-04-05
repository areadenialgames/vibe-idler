import Foundation
import AppKit

let bundle = Bundle.main
let macosPath = (bundle.bundlePath as NSString).appendingPathComponent("Contents/MacOS")
let resourcesPath = (bundle.bundlePath as NSString).appendingPathComponent("Contents/Resources")
let binaryPath = (macosPath as NSString).appendingPathComponent("vibe-idler-bin")

// Create a temporary .command file
let commandFile = NSTemporaryDirectory() + "vibe-idler-launch.command"
let script = """
#!/bin/bash
clear
cd "\(resourcesPath)"
"\(binaryPath)"
exit
"""
try? script.write(toFile: commandFile, atomically: true, encoding: .utf8)
try? FileManager.default.setAttributes(
    [.posixPermissions: 0o755], ofItemAtPath: commandFile)

// Find the user's preferred terminal from LaunchServices
// This reads the handler for public.unix-executable or com.apple.terminal.shell-script
func preferredTerminalURL() -> URL? {
    // Check LSHandlers for shell script types
    if let plistPath = NSHomeDirectory() + "/Library/Preferences/com.apple.LaunchServices/com.apple.launchservices.secure.plist" as String?,
       let data = FileManager.default.contents(atPath: plistPath),
       let plist = try? PropertyListSerialization.propertyList(from: data, format: nil) as? [String: Any],
       let handlers = plist["LSHandlers"] as? [[String: Any]] {
        for handler in handlers {
            if let role = handler["LSHandlerContentType"] as? String,
               role == "public.unix-executable",
               let bundleID = handler["LSHandlerRoleAll"] as? String,
               bundleID != "-" {
                return NSWorkspace.shared.urlForApplication(
                    withBundleIdentifier: bundleID)
            }
        }
    }
    return nil
}

let fileURL = URL(fileURLWithPath: commandFile)

if let terminalURL = preferredTerminalURL() {
    // Open with user's preferred terminal
    let config = NSWorkspace.OpenConfiguration()
    NSWorkspace.shared.open(
        [fileURL], withApplicationAt: terminalURL,
        configuration: config
    ) { _, error in
        if let error = error {
            NSLog("Failed to open with preferred terminal: \(error)")
            // Fallback: open normally
            NSWorkspace.shared.open(fileURL)
        }
        DispatchQueue.main.asyncAfter(deadline: .now() + 2.0) {
            NSApplication.shared.terminate(nil)
        }
    }
} else {
    // No preference found — use system default
    NSWorkspace.shared.open(fileURL)
    DispatchQueue.main.asyncAfter(deadline: .now() + 2.0) {
        NSApplication.shared.terminate(nil)
    }
}

RunLoop.main.run(until: Date(timeIntervalSinceNow: 5.0))

import Foundation
import AppKit

// Tiny launcher that opens Terminal.app and runs the actual binary
let bundle = Bundle.main
let bundlePath = bundle.bundlePath
let macosPath = (bundlePath as NSString).appendingPathComponent("Contents/MacOS")
let binaryPath = (macosPath as NSString).appendingPathComponent("vibe-idler-bin")

let script = """
tell application "Terminal"
    activate
    do script "cd \\\"\(macosPath)\\\" && ./vibe-idler-bin; exit"
end tell
"""

if let appleScript = NSAppleScript(source: script) {
    var error: NSDictionary?
    appleScript.executeAndReturnError(&error)
    if let error = error {
        NSLog("AppleScript error: \(error)")
    }
}

// Quit ourselves after launching Terminal
DispatchQueue.main.asyncAfter(deadline: .now() + 1.0) {
    NSApplication.shared.terminate(nil)
}

RunLoop.main.run(until: Date(timeIntervalSinceNow: 2.0))

import Foundation
import AppKit

let bundle = Bundle.main
let bundlePath = bundle.bundlePath
let macosPath = (bundlePath as NSString).appendingPathComponent("Contents/MacOS")
let resourcesPath = (bundlePath as NSString).appendingPathComponent("Contents/Resources")
let binaryPath = (macosPath as NSString).appendingPathComponent("vibe-idler-bin")

// Open Terminal.app and run the game binary from the Resources dir
// so it finds assets/ relative to cwd
let script = """
tell application "Terminal"
    activate
    do script "cd \\\"\(resourcesPath)\\\" && \\\"\(binaryPath)\\\"; exit"
end tell
"""

if let appleScript = NSAppleScript(source: script) {
    var error: NSDictionary?
    appleScript.executeAndReturnError(&error)
}

// Quit after launching
DispatchQueue.main.asyncAfter(deadline: .now() + 1.0) {
    NSApplication.shared.terminate(nil)
}

RunLoop.main.run(until: Date(timeIntervalSinceNow: 2.0))
